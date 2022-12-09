//! Module for direct memory operations.
//!
//! Generally you don't need to call these yourself. Instead, the compiler will
//! insert calls to the functions defined here as necessary.

use core::ffi::c_void;

/// Byte copy between exclusive regions.
///
/// * This will *always* copy one byte at a time, making it suitable for use
///   with SRAM memory.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memcpy1"]
pub unsafe extern "C" fn __aeabi_memcpy1(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  core::arch::asm! {
    "1:",
    "subs    {count}, {count}, #1",
    "ldrbge  {temp}, [{src}], #1",
    "strbge  {temp}, [{dest}], #1",
    "bgt     1b",
    temp = out(reg) _,
    count = inout(reg) byte_count => _,
    src = inout(reg) src => _,
    dest = inout(reg) dest => _,
    options(nostack)
  }
}

/// Halfword copy between exclusive regions.
///
/// * **Safety:** The pointers must start aligned to 2.
/// * If the `byte_count` is odd then a single byte copy will happen at the end.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memcpy2"]
pub unsafe extern "C" fn __aeabi_memcpy2(
  mut dest: *mut u16, mut src: *const u16, mut byte_count: usize,
) {
  core::arch::asm! {
    "1:",
    "subs    {count}, {count}, #2",
    "ldrhge  {temp}, [{src}], #2",
    "strhge  {temp}, [{dest}], #2",
    "bgt     1b",
    temp = out(reg) _,
    count = inout(reg) byte_count,
    src = inout(reg) src,
    dest = inout(reg) dest,
    options(nostack)
  }
  if byte_count != 0 {
    let dest = dest.cast::<u8>();
    let src = src.cast::<u8>();
    dest.write_volatile(src.read_volatile());
  }
}

/// Word copy between exclusive regions.
///
/// * **Safety:** The pointers must start aligned to 4.
/// * If `byte_count` is not a multiple of 4 then a halfword and/or byte copy
///   will happen at the end.
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memcpy4"]
pub unsafe extern "C" fn __aeabi_memcpy4(
  dest: *mut u32, src: *const u32, byte_count: usize,
) {
  core::arch::asm! {
    bracer::when!( "r2" >=u "#32" [label_id=2] {
      bracer::with_pushed_registers!("{{r4-r9}}", {
        "1:",
        "subs   r2, r2, #32",
        "ldmge  r1!, {{r3-r9, r12}}",
        "stmge  r0!, {{r3-r9, r12}}",
        "bgt    1b",
      }),
      "bxeq   lr",
    }),

    // copy 4 words, two at a time
    "tst    r2, #0b10000",
    "ldmne  r1!, {{r3, r12}}",
    "stmne  r0!, {{r3, r12}}",
    "ldmne  r1!, {{r3, r12}}",
    "stmne  r0!, {{r3, r12}}",
    "bics   r2, r2, #0b10000",
    "bxeq   lr",

    // copy 2 and/or 1 words
    "lsls   r3, r2, #29",
    "ldmcs  r1!, {{r3, r12}}",
    "stmcs  r0!, {{r3, r12}}",
    "ldrmi  r3, [r1], #4",
    "strmi  r3, [r0], #4",
    "bics   r2, r2, #0b1100",
    "bxeq   lr",

    // copy halfword and/or byte
    "lsls   r3, r2, #31",
    "ldrhcs r3, [r1], #2",
    "strhcs r3, [r0], #2",
    "ldrbmi r3, [r1], #1",
    "strbmi r3, [r0], #1",
    "bx     lr",
    options(noreturn),
  }
}

/// Just call [`__aeabi_memcpy4`] instead.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memcpy8"]
pub unsafe extern "C" fn __aeabi_memcpy8(
  dest: *mut u32, src: *const u32, byte_count: usize,
) {
  __aeabi_memcpy4(dest, src, byte_count);
}

/// Arbitrary width copy between exclusive regions.
///
/// * The pointers do not have a minimum alignment.
/// * The function will automatically use the best type of copy possible, based
///   on the pointers given.
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memcpy"]
pub unsafe extern "C" fn __aeabi_memcpy(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  core::arch::asm! {
    "cmp    r2, #7", // if count <= (fix+word): just byte copy
    "ble    {__aeabi_memcpy1}",

    // check max coalign
    "eor    r3, r0, r1",
    "lsls   r3, r3, #31",
    "bmi    {__aeabi_memcpy1}",
    "bcs    2f",

    // max coalign4, possible fixup and jump
    "lsls   r3, r0, #31",
    "submi  r2, r2, #1",
    "ldrbmi r3, [r1], #1",
    "strbmi r3, [r0], #1",
    "subcs  r2, r2, #2",
    "ldrhcs r3, [r1], #2",
    "strhcs r3, [r0], #2",
    "b      {__aeabi_memcpy4}",

    // max coalign2, possible fixup and jump
    "2:",
    "lsls   r3, r0, #31",
    "submi  r2, r2, #1",
    "ldrbmi r3, [r1], #1",
    "strbmi r3, [r0], #1",
    "b      {__aeabi_memcpy2}",

    //
    __aeabi_memcpy4 = sym __aeabi_memcpy4,
    __aeabi_memcpy2 = sym __aeabi_memcpy2,
    __aeabi_memcpy1 = sym __aeabi_memcpy1,
    options(noreturn)
  }
}

/// Copy between exclusive regions, prefer [`__aeabi_memcpy`] if possible.
///
/// This is the libc version of a memory copy. It's required to return the
/// `dest` pointer at the end of the call, which makes it need an extra
/// push/pop compared to a direct call to `__aeabi_memcpy`.
///
/// * **Returns:** The `dest` pointer.
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.memcpy"]
pub unsafe extern "C" fn memcpy(
  dest: *mut u8, src: *const u8, byte_count: usize,
) -> *mut u8 {
  core::arch::asm! {
    bracer::with_pushed_registers!("{{r0, lr}}", {
      "bl {__aeabi_memcpy}",
    }),
    "bx lr",
    __aeabi_memcpy = sym __aeabi_memcpy,
    options(noreturn)
  }
}

// MOVE

// used by `__aeabi_memmove` in some cases
#[inline]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.reverse_copy_u8"]
unsafe extern "C" fn reverse_copy_u8(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  core::arch::asm! {
    "1:",
    "subs    {count}, {count}, #1",
    "ldrbge  {temp}, [{src}, #-1]!",
    "strbge  {temp}, [{dest}, #-1]!",
    "bgt     1b",
    temp = out(reg) _,
    count = inout(reg) byte_count => _,
    src = inout(reg) src => _,
    dest = inout(reg) dest => _,
    options(nostack)
  }
}

// used by `__aeabi_memmove` in some cases
#[inline]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.reverse_copy_u16"]
unsafe extern "C" fn reverse_copy_u16(
  mut dest: *mut u16, mut src: *const u16, mut byte_count: usize,
) {
  core::arch::asm! {
    "1:",
    "subs    {count}, {count}, #2",
    "ldrhge  {temp}, [{src}, #-2]!",
    "strhge  {temp}, [{dest}, #-2]!",
    "bgt     1b",
    temp = out(reg) _,
    count = inout(reg) byte_count,
    src = inout(reg) src,
    dest = inout(reg) dest,
    options(nostack)
  }
  if byte_count != 0 {
    let dest = dest.cast::<u8>().sub(1);
    let src = src.cast::<u8>().sub(1);
    dest.write_volatile(src.read_volatile());
  }
}

// used by `__aeabi_memmove` in some cases
#[naked]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.reverse_copy_u32"]
unsafe extern "C" fn reverse_copy_u32(
  dest: *mut u32, src: *const u32, byte_count: usize,
) {
  core::arch::asm! {
    bracer::when!( "r2" >=u "#32" [label_id=2] {
      bracer::with_pushed_registers!("{{r4-r9}}", {
        "1:",
        "subs    r2, r2, #32",
        "ldmdbcs r1!, {{r3-r9, r12}}",
        "stmdbcs r0!, {{r3-r9, r12}}",
        "bgt     1b",
      }),
      "bxeq   lr",
    }),

    // copy 4 words, two at a time
    "tst     r2, #0b10000",
    "ldmdbne r1!, {{r3, r12}}",
    "stmdbne r0!, {{r3, r12}}",
    "ldmdbne r1!, {{r3, r12}}",
    "stmdbne r0!, {{r3, r12}}",
    "bics    r2, r2, #0b10000",
    "bxeq    lr",

    // copy 2 and/or 1 words
    "lsls    r3, r2, #29",
    "ldmdbcs r1!, {{r3, r12}}",
    "stmdbcs r0!, {{r3, r12}}",
    "ldrmi   r3, [r1, #-4]!",
    "strmi   r3, [r0, #-4]!",
    "bxeq    lr",

    // copy halfword and/or byte
    "lsls    r2, r2, #31",
    "ldrhcs  r3, [r1, #-2]!",
    "strhcs  r3, [r0, #-2]!",
    "ldrbmi  r3, [r1, #-1]!",
    "strbmi  r3, [r0, #-1]!",
    "bx      lr",
    options(noreturn),
  }
}

/// Copy between non-exclusive regions, prefer [`__aeabi_memmove`] if possible.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memmove4"]
pub unsafe extern "C" fn __aeabi_memmove4(
  dest: *mut u32, src: *const u32, byte_count: usize,
) {
  __aeabi_memmove(dest.cast(), src.cast(), byte_count)
}

/// Copy between non-exclusive regions, prefer [`__aeabi_memmove`] if possible.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memmove8"]
pub unsafe extern "C" fn __aeabi_memmove8(
  dest: *mut u32, src: *const u32, byte_count: usize,
) {
  __aeabi_memmove(dest.cast(), src.cast(), byte_count)
}

/// Copy between non-exclusive regions.
///
/// * The pointers do not have a minimum alignment. The function will
///   automatically detect the best type of copy to perform.
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memmove"]
pub unsafe extern "C" fn __aeabi_memmove(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  core::arch::asm! {
    "cmp    r2, #7", // if count <= (fix+word): just byte copy
    "ble    {__aeabi_memcpy1}",
    bracer::when!("r0" >=u "r1" [label_id=1] {
      // when d > s we need to reverse-direction copy
      "add     r0, r0, r2",
      "add     r1, r1, r2",
      "eor     r3, r0, r1",
      "lsls    r3, r3, #31",
      "bmi     {reverse_copy_u8}",
      "bcs     2f",

      // max coalign4, possible fixup and jump
      "lsls    r3, r0, #31",
      "submi   r2, r2, #1",
      "ldrbmi  r3, [r1, #-1]!",
      "strbmi  r3, [r0, #-1]!",
      "subcs   r2, r2, #2",
      "ldrhcs  r3, [r1, #-2]!",
      "strhcs  r3, [r0, #-2]!",
      "b       {reverse_copy_u32}",

      // max coalign2, possible fixup and jump
      "2:",
      "tst     r0, #1",
      "sub     r2, r2, #1",
      "ldrb    r3, [r1, #-1]!",
      "strb    r3, [r0, #-1]!",
      "b       {reverse_copy_u16}",
    }),
    "b      {__aeabi_memcpy}",
    __aeabi_memcpy = sym __aeabi_memcpy,
    __aeabi_memcpy1 = sym __aeabi_memcpy1,
    reverse_copy_u8 = sym reverse_copy_u8,
    reverse_copy_u16 = sym reverse_copy_u16,
    reverse_copy_u32 = sym reverse_copy_u32,
    options(noreturn),
  }
}

/// Copy between non-exclusive regions, prefer [`__aeabi_memmove`] if possible.
///
/// This is the libc version of a memory move. It's required to return the
/// `dest` pointer at the end of the call, which makes it need an extra
/// push/pop compared to a direct call to `__aeabi_memmove`.
///
/// * **Returns:** The `dest` pointer.
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.memmove"]
pub unsafe extern "C" fn memmove(
  dest: *mut u8, src: *const u8, byte_count: usize,
) -> *mut u8 {
  core::arch::asm! {
    bracer::with_pushed_registers!("{{r0, lr}}", {
      "bl {__aeabi_memmove}",
    }),
    "bx lr",
    __aeabi_memmove = sym __aeabi_memmove,
    options(noreturn)
  }
}

// SET

/// Copy between non-exclusive regions, prefer [`__aeabi_memset`] if possible.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memset4"]
pub unsafe extern "C" fn __aeabi_memset4(
  dest: *mut u32, byte_count: usize, byte: i32,
) {
  __aeabi_memset(dest.cast(), byte_count, byte)
}

/// Copy between non-exclusive regions, prefer [`__aeabi_memset`] if possible.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memset8"]
pub unsafe extern "C" fn __aeabi_memset8(
  dest: *mut u32, byte_count: usize, byte: i32,
) {
  __aeabi_memset(dest.cast(), byte_count, byte)
}

/// Sets all bytes in the region to the `byte` given.
///
/// Because of historical reasons, the byte is passed in as an `i32`, but only
/// the lowest 8 bits are used.
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memset"]
pub unsafe extern "C" fn __aeabi_memset(
  dest: *mut u8, byte_count: usize, byte: i32,
) {
  core::arch::asm! {
    bracer::when!("r1" >=u "#8" [label_id=7] {
      // duplicate the byte across all of r2 and r3
      "and    r2, r2, #0xFF",
      "orr    r2, r2, r2, lsl #8",
      "orr    r2, r2, r2, lsl #16",
      "mov    r3, r2",

      // carry/sign test on the address, then do fixup
      "lsls   r12, r0, #31",
      "submi  r1, r1, #1",
      "strbmi r2, [r0], #1",
      "subcs  r1, r1, #2",
      "strhcs r2, [r0], #2",

      bracer::when!("r1" >=u "#32" [label_id=8] {
        bracer::with_pushed_registers!("{{r4-r9}}", {
          "mov    r4, r2",
          "mov    r5, r2",
          "mov    r6, r2",
          "mov    r7, r2",
          "mov    r8, r2",
          "mov    r9, r2",
          "1:",
          "subs   r1, r1, #32",
          "stmge  r0!, {{r2-r9}}",
          "bgt    1b",
        }),
        "bxeq   lr",
      }),

      // set 4 words
      "tst    r1, #0b10000",
      "stmne  r0!, {{r2, r3}}",
      "stmne  r0!, {{r2, r3}}",

      // set 2 and/or 1 words
      "lsls   r12, r1, #29",
      "stmcs  r0!, {{r2, r3}}",
      "strmi  r2, [r0], #4",

      // set halfword and/or byte
      "lsls   r12, r1, #31",
      "strhcs r2, [r0], #2",
      "strbmi r2, [r0], #1",
      "bx     lr",
    }),
    // byte loop
    "9:",
    "subs   r1, r1, #1",
    "strbcs r2, [r0], #1",
    "bgt    9b",
    "bx     lr",
    options(noreturn)
  }
}

/// Copy between non-exclusive regions, prefer [`__aeabi_memset`] if possible.
///
/// This is the libc version of a memory set. It's required to return the
/// `dest` pointer at the end of the call, which makes it need an extra
/// push/pop compared to a direct call to `__aeabi_memset`.
///
/// * **Returns:** The `dest` pointer.
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.memset"]
pub unsafe extern "C" fn memset(
  dest: *mut u8, byte: i32, byte_count: usize,
) -> *mut u8 {
  core::arch::asm! {
    bracer::with_pushed_registers!("{{r0, lr}}", {
      "bl {__aeabi_memset}",
    }),
    "bx lr",
    __aeabi_memset = sym __aeabi_memset,
    options(noreturn)
  }
}

// CLEAR

/// Just call [`__aeabi_memset`] with 0 as the `byte` instead.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memclr4"]
pub unsafe extern "C" fn __aeabi_memclr4(dest: *mut u32, byte_count: usize) {
  __aeabi_memset(dest.cast(), byte_count, 0)
}

/// Just call [`__aeabi_memset`] with 0 as the `byte` instead.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memclr8"]
pub unsafe extern "C" fn __aeabi_memclr8(dest: *mut u32, byte_count: usize) {
  __aeabi_memset(dest.cast(), byte_count, 0)
}

/// Just call [`__aeabi_memset`] with 0 as the `byte` instead.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memclr"]
pub unsafe extern "C" fn __aeabi_memclr(dest: *mut u8, byte_count: usize) {
  __aeabi_memset(dest, byte_count, 0)
}

/// Reads 4 bytes, starting at the address given.
///
/// See [__aeabi_uread4]
///
/// [__aeabi_uread4]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.aeabi.uread4"]
unsafe extern "C" fn __aeabi_uread4(address: *const c_void) -> u32 {
  core::arch::asm!(
    "ldrb r2, [r0]",
    "ldrb r3, [r0, #1]",
    "orr  r2, r2, r3, lsl #8",
    "ldrb r3, [r0, #2]",
    "orr  r2, r2, r3, lsl #16",
    "ldrb r3, [r0, #3]",
    "orr  r2, r2, r3, lsl #24",
    "mov  r0, r2",
    "bx   lr",
    options(noreturn),
  )
}

/// Writes 4 bytes, starting at the address given.
///
/// See [__aeabi_uwrite4]
///
/// [__aeabi_uwrite4]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.aeabi.uwrite4"]
unsafe extern "C" fn __aeabi_uwrite4(value: u32, address: *mut c_void) {
  core::arch::asm!(
    "strb r0, [r1]",
    "lsr  r2, r0, #8",
    "strb r2, [r1, #1]",
    "lsr  r2, r2, #8",
    "strb r2, [r1, #2]",
    "lsr  r2, r2, #8",
    "strb r2, [r1, #3]",
    "bx   lr",
    options(noreturn),
  )
}

/// Reads 8 bytes, starting at the address given.
///
/// See [__aeabi_uread8]
///
/// [__aeabi_uread8]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.aeabi.uread8"]
unsafe extern "C" fn __aeabi_uread8(address: *const c_void) -> u64 {
  core::arch::asm!(
    "ldrb r1, [r0, #4]",
    "ldrb r2, [r0, #5]",
    "orr  r1, r1, r2, lsl #8",
    "ldrb r2, [r0, #6]",
    "orr  r1, r1, r2, lsl #16",
    "ldrb r2, [r0, #7]",
    "orr  r1, r1, r2, lsl #24",
    "b    {__aeabi_uread4}",
    __aeabi_uread4 = sym __aeabi_uread4,
    options(noreturn),
  )
}

/// Writes 8 bytes, starting at the address given.
///
/// See [__aeabi_uwrite8]
///
/// [__aeabi_uwrite8]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.aeabi.uwrite8"]
unsafe extern "C" fn __aeabi_uwrite8(value: u64, address: *mut c_void) {
  core::arch::asm!(
    "strb r0, [r2]",
    "lsr  r3, r0, #8",
    "strb r3, [r2, #1]",
    "lsr  r3, r3, #8",
    "strb r3, [r2, #2]",
    "lsr  r3, r3, #8",
    "strb r3, [r2, #3]",
    "strb r1, [r2, #4]",
    "lsr  r3, r1, #8",
    "strb r3, [r2, #5]",
    "lsr  r3, r3, #8",
    "strb r3, [r2, #6]",
    "lsr  r3, r3, #8",
    "strb r3, [r2, #7]",
    "bx   lr",
    options(noreturn),
  )
}
