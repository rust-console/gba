// For now, the division fns can just keep living here.

/// Returns 0 in `r0`, while placing the `numerator` into `r1`.
///
/// This is written in that slightly strange way so that `div` function and
/// `divmod` functions can share the same code path.
///
/// See: [__aeabi_idiv0][aeabi-division-by-zero]
///
/// [aeabi-division-by-zero]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#division-by-zero
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
// this should literally never get called for real, so we leave it in ROM
extern "C" fn __aeabi_idiv0(numerator: i32) -> i32 {
  unsafe {
    core::arch::asm!(
      // this comment stops rustfmt from making this a one-liner
      "mov r1, r0",
      "mov r0, #0",
      "bx  lr",
      options(noreturn)
    )
  }
}

/// Returns `u32 / u32`
///
/// This implementation is *not* the fastest possible division, but it is
/// extremely compact.
///
/// See: [__aeabi_uidiv][aeabi-integer-32-32-division]
///
/// [aeabi-integer-32-32-division]:
///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.aeabi.uidiv"]
extern "C" fn __aeabi_uidiv(numerator: u32, denominator: u32) -> u32 {
  // Note(Lokathor): Other code in this module relies on being able to call this
  // function without affecting r12, so any future implementations of this code
  // **must not** destroy r12.
  unsafe {
    core::arch::asm!(
      // Check for divide by 0
      "cmp   r1, #0",
      "beq   {__aeabi_idiv0}",
      // r3(shifted_denom) = denom
      "mov   r3, r1",
      // while shifted_denom < (num>>1): shifted_denom =<< 1;
      "cmp   r3, r0, lsr #1",
      "2:",
      "lslls r3, r3, #1",
      "cmp   r3, r0, lsr #1",
      "bls   2b",
      // r0=quot(init 0), r1=denom, r2=num, r3=shifted_denom
      "mov   r2, r0",
      "mov   r0, #0",
      // subtraction loop
      "3:",
      "cmp   r2, r3",
      "subcs r2, r2, r3",
      "adc   r0, r0, r0",
      "mov   r3, r3, lsr #1",
      "cmp   r3, r1",
      "bcs   3b",
      "bx    lr",
      __aeabi_idiv0 = sym __aeabi_idiv0,
      options(noreturn)
    )
  }
}

/// Returns `i32 / i32`
///
/// See: [__aeabi_idiv][aeabi-integer-32-32-division]
///
/// [aeabi-integer-32-32-division]:
///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.aeabi.idiv"]
extern "C" fn __aeabi_idiv(numerator: i32, denominator: i32) -> u32 {
  unsafe {
    core::arch::asm!(
      // determine if `numerator` and `denominator` are the same sign
      "eor   r12, r1, r0",
      // convert both values to their unsigned absolute value.
      "cmp   r0, #0",
      "rsblt r0, r0, #0",
      "cmp   r1, #0",
      "rsclt r1, r1, #0",
      bracer::with_pushed_registers!("{{lr}}", {
        // divide them using `u32` division (this will check for divide by 0)
        "bl    {__aeabi_uidiv}",
      }),
      // if they started as different signs, flip the output's sign.
      "cmp   r12, #0",
      "rsblt r0, r0, #0",
      "bx    lr",
      __aeabi_uidiv = sym __aeabi_uidiv,
      options(noreturn)
    )
  }
}

/// Returns `(u32 / u32, u32 % u32)` in `(r0, r1)`.
///
/// The `u64` return value is a mild lie that gets Rust to grab up both the `r0`
/// and `r1` values when the function returns. If you transmute the return value
/// into `[u32; 2]` then you can separate the two parts of the return value, and
/// it will have no runtime cost.
///
/// See: [__aeabi_uidivmod][aeabi-integer-32-32-division]
///
/// [aeabi-integer-32-32-division]:
///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.aeabi.uidivmod"]
extern "C" fn __aeabi_uidivmod(numerator: u32, denominator: u32) -> u64 {
  unsafe {
    core::arch::asm!(
      // We need to save *both* input args until after the uidiv call. One of
      // them can be saved in `r12` because we know our uidiv doesn't actually
      // touch `r12`, while the other will be pushed onto the stack along with
      // `lr`. Since the function's output will be in `r0`, we push/pop `r1`.
      "mov   r12, r0",
      bracer::with_pushed_registers!("{{r1, lr}}", {
        "bl    {__aeabi_uidiv}",
      }),
      // Now r0 holds the `quot`, and we use it along with the input args to
      // calculate the `rem`.
      "mul   r2, r0, r1",
      "sub   r1, r12, r2",
      "bx    lr",
      __aeabi_uidiv = sym __aeabi_uidiv,
      options(noreturn)
    )
  }
}

/// Returns `(i32 / i32, i32 % i32)` in `(r0, r1)`.
///
/// The `u64` return value is a mild lie that gets Rust to grab up both the `r0`
/// and `r1` values when the function returns. If you transmute the return value
/// into `[i32; 2]` then you can separate the two parts of the return value, and
/// it will have no runtime cost.
///
/// See: [__aeabi_idivmod][aeabi-integer-32-32-division]
///
/// [aeabi-integer-32-32-division]:
///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.aeabi.idivmod"]
extern "C" fn __aeabi_idivmod(numerator: i32, denominator: i32) -> u64 {
  unsafe {
    core::arch::asm!(
      bracer::with_pushed_registers!("{{r4, r5, lr}}", {
        // store old numerator then make it the unsigned absolute
        "movs  r4, r0",
        "rsblt r0, r0, #0",
        // store old denominator then make it the unsigned absolute
        "movs  r5, r1",
        "rsblt r1, r1, #0",
        // divmod using unsigned.
        "bl    {__aeabi_uidivmod}",
        // if signs started opposite, quot becomes negative
        "eors  r12, r4, r5",
        "rsblt r0, r0, #0",
        // if numerator started negative, rem is negative
        "cmp   r4, #0",
        "rsblt r1, r1, #0",
      }),
      "bx    lr",
      __aeabi_uidivmod = sym __aeabi_uidivmod,
      options(noreturn)
    )
  }
}
