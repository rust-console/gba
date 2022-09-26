use core::ffi::c_void;

use crate::{
  dma::DmaControl,
  gba_cell::GbaCell,
  interrupts::IrqFn,
  mmio::{DMA3_SRC, IME},
};

/// Builds an assembly string that puts the contained code in the section
/// specified.
///
/// ```txt
/// put_code_in_section!( ".example.section", {
///   "lines"
///   "go"
///   "here"
/// });
/// ```
macro_rules! put_code_in_section {
  ($section_name:expr, {
    $($asm_line:expr),+ $(,)?
  }) => {
    concat!(
      concat!(".section ", $section_name, "\n"),
      $( concat!($asm_line, "\n") ),+ ,
      concat!(".previous\n"),
    )
  }
}

/// Builds an assembly string wrapped in `.code 32` and `.code 16` as necessary
///
/// ```txt
/// emit_a32_code!{
///   "lines"
///   "go"
///   "here"
/// };
/// ```
#[cfg(target_feature = "thumb-mode")]
macro_rules! emit_a32_code {
  ($($asm_line:expr),+ $(,)?) => {
    concat!(
      concat!(".code 32\n"),
      $( concat!($asm_line, "\n") ),+ ,
      concat!(".code 16\n"),
    )
  }
}

/// Builds an assembly string wrapped in `.code 32` and `.code 16` as necessary
///
/// ```txt
/// emit_a32_code!{
///   "lines"
///   "go"
///   "here"
/// };
/// ```
#[cfg(not(target_feature = "thumb-mode"))]
macro_rules! emit_a32_code {
  ($($asm_line:expr),+ $(,)?) => {
    concat!(
      $( concat!($asm_line, "\n") ),+ ,
    )
  }
}

/// Builds an assembly string that pushes some regs, does the body, then pops
/// the regs.
///
/// The `reglist` expression should include the appropriate level of braces for
/// the enclosing assembly block (two for normal asm, or one for raw asm).
///
/// ```txt
/// with_pushed_registers!( "reglist", {
///   "lines"
///   "go"
///   "here"
/// });
/// ```
macro_rules! with_pushed_registers {
  ($reglist:expr, {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("push ", $reglist, "\n"),
      $( concat!($asm_line, "\n") ),* ,
      concat!("pop ", $reglist, "\n"),
    )
  }
}

/// Reads SPSR into the register named, does the block, and writes the same
/// register back to SPSR.
macro_rules! with_spsr_held_in {
  ($reg:literal {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("mrs ", $reg, ", SPSR\n"),
      $( concat!($asm_line, "\n") ),* ,
      concat!("msr SPSR, ", $reg, "\n"),
    )
  }
}

/// Sets `lr` to just after the `bx`, then uses `bx` with the given register.
///
/// This generates a label, so pick a `label_id` that won't interfere with any
/// nearby code.
macro_rules! adr_lr_then_bx_to {
  (reg=$reg_name:expr, label_id=$label:expr) => {
    concat!(
      concat!("adr lr, ", $label, "f\n"),
      concat!("bx ", $reg_name, "\n"),
      concat!($label, ":\n"),
    )
  };
}

/// Expands to the asm line to set the control bits of CPSR.
///
/// * Can only be used in `a32`
/// * Only sets the control bits, all other bits (eg: flag bits) are unchanged.
///
/// Currently, not all possible patterns are covered by this macro, just the
/// patterns needed by this runtime when it was written. In general, any of the
/// five CPU modes can be combined with irq and fiq masking each being either
/// off or on. If a desired combination is missing just add it.
macro_rules! set_cpu_control {
  // CPSR low bits are: `I F T MMMMM`, and T must always be left as 0.
  // * 0b10011: Supervisor (SVC)
  // * 0b11111: System (SYS)
  (System, irq_masked: false, fiq_masked: false) => {
    "msr CPSR_c, #0b00011111\n"
  };
  (Supervisor, irq_masked: true, fiq_masked: false) => {
    "msr CPSR_c, #0b10010010\n"
  };
}

/// Performs the appropriate test, then either runs the block or jumps past it,
/// depending on the test result.
///
/// Currently supports:
/// * `$reg == $op2`
/// * `$reg != $op2`
macro_rules! when {
  ($reg:literal == $op2:literal [label_id=$label:literal] {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("cmp ", $reg, ", ", $op2, "\n"),
      concat!("bne ", $label, "f\n"),
      $( concat!($asm_line, "\n") ),* ,
      concat!($label, ":\n"),
    )
  };
  ($reg:literal != $op2:literal [label_id=$label:literal] {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("cmp ", $reg, ", ", $op2, "\n"),
      concat!("beq ", $label, "f\n"),
      $( concat!($asm_line, "\n") ),* ,
      concat!($label, ":\n"),
    )
  };
}

/// The function pointer that the assembly runtime calls when an interrupt
/// occurs.
pub static RUST_IRQ_HANDLER: GbaCell<Option<IrqFn>> = GbaCell::new(None);

const DMA_32_BIT_MEMCPY: DmaControl =
  DmaControl::new().with_transfer_32bit(true).with_enabled(true);

const DMA3_OFFSET: usize = DMA3_SRC.as_usize() - 0x0400_0000;
const IME_OFFSET: usize = IME.as_usize() - 0x0400_0000;

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text.gba_rom_header"]
unsafe extern "C" fn __start() -> ! {
  core::arch::asm!(
    "b 1f",
    ".space 0xE0",
    "1:", /* post header */
    "mov r12, #{mmio_base}",
    "add r0, r12, #{waitcnt_offset}",
    "ldr r1, ={waitcnt_setting}",
    "strh r1, [r0]",

    /* iwram copy */
    "ldr r4, =__iwram_word_copy_count",
    when!("r4" != "#0" [label_id=1] {
      "add r3, r12, #{dma3_offset}",
      "mov r5, #{dma3_setting}",
      "ldr r0, =__iwram_start",
      "ldr r2, =__iwram_position_in_rom",
      "str r2, [r3]", /* source */
      "str r0, [r3, #4]", /* destination */
      "strh r4, [r3, #8]", /* word count */
      "strh r5, [r3, #10]", /* set control bits */
    }),

    /* ewram copy */
    "ldr r4, =__ewram_word_copy_count",
    when!("r4" != "#0" [label_id=1] {
      "add r3, r12, #{dma3_offset}",
      "mov r5, #{dma3_setting}",
      "ldr r0, =__ewram_start",
      "ldr r2, =__ewram_position_in_rom",
      "str r2, [r3]", /* source */
      "str r0, [r3, #4]", /* destination */
      "strh r4, [r3, #8]", /* word count */
      "strh r5, [r3, #10]", /* set control bits */
    }),

    /* bss zero */
    "ldr r4, =__bss_word_clear_count",
    when!("r4" != "#0" [label_id=1] {
      "ldr r0, =__bss_start",
      "mov r2, #0",
      "2:",
      "str r2, [r0], #4",
      "subs r4, r4, #1",
      "bne 2b",
    }),

    /* assign the runtime irq handler */
    "ldr r1, ={rt0_irq_handler}",
    "str r1, [r12, #-4]",

    /* call to rust main */
    "ldr r0, =main",
    "bx r0",
    // main shouldn't return, but if it does just SoftReset
    "swi #0",
    mmio_base = const 0x0400_0000,
    waitcnt_offset = const 0x204,
    waitcnt_setting = const 0x4317 /*sram8,r0:3.1,r1:4.2,r2:8.2,no_phi,prefetch*/,
    dma3_offset = const DMA3_OFFSET,
    dma3_setting = const DMA_32_BIT_MEMCPY.to_u16(),
    rt0_irq_handler = sym rt0_irq_handler,
    options(noreturn)
  )
}

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.rt0.irq_handler"]
unsafe extern "C" fn rt0_irq_handler() {
  // On Entry: r0 = 0x0400_0000 (mmio_base)
  core::arch::asm!(
    /* swap IME off, user can turn it back on if they want */
    "add r12, r0, #{ime_offset}",
    "mov r3, #0",
    "swp r3, r3, [r12]",

    /* Read/Update IE and IF */
    "ldr r0, [r12, #-8]",
    "and r0, r0, r0, LSR #16",
    "strh r0, [r12, #-6]",

    /* Read/Update BIOS_IF */
    "sub  r2, r12, #(0x208+8)",
    "ldrh r1, [r2]",
    "orr  r1, r1, r0",
    "strh r1, [r2]",

    /* Call the Rust fn pointer (if set), using System mode */
    "ldr r1, ={RUST_IRQ_HANDLER}",
    "ldr r1, [r1]",
    when!("r1" != "#0" [label_id=9] {
      with_spsr_held_in!("r2" {
        set_cpu_control!(System, irq_masked: false, fiq_masked: false),

        // Note(Lokathor): We are *SKIPPING* the part where we ensure that the
        // System stack pointer is aligned to 8 during the call to the rust
        // function. This is *technically* against the AAPCS ABI, but the GBA's
        // ARMv4T CPU does not even support any instructions that require an
        // alignment of 8. By not bothering to align the stack, we save about 5
        // cycles total. Which is neat, but if this were on the DS (which has an
        // ARMv5TE CPU) you'd want to ensure the aligned stack.

        with_pushed_registers!("{{r2, r3, r12, lr}}", {
          adr_lr_then_bx_to!(reg="r1", label_id=1)
        }),

        set_cpu_control!(Supervisor, irq_masked: true, fiq_masked: false),
      }),
    }),

    /* Restore initial IME setting and return */
    "swp r3, r3, [r12]",
    "bx lr",
    ime_offset = const IME_OFFSET,
    RUST_IRQ_HANDLER = sym RUST_IRQ_HANDLER,
    options(noreturn)
  )
}

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
      "beq   __aeabi_idiv0",
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
      with_pushed_registers!("{{lr}}", {
        // divide them using `u32` division (this will check for divide by 0)
        "bl    __aeabi_uidiv",
      }),
      // if they started as different signs, flip the output's sign.
      "cmp   r12, #0",
      "rsblt r0, r0, #0",
      "bx    lr",
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
      with_pushed_registers!("{{r1, lr}}", {
        "bl    __aeabi_uidiv",
      }),
      // Now r0 holds the `quot`, and we use it along with the input args to
      // calculate the `rem`.
      "mul   r2, r0, r1",
      "sub   r1, r12, r2",
      "bx    lr",
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
      with_pushed_registers!("{{r4, r5, lr}}", {
        // store old numerator then make it the unsigned absolute
        "movs  r4, r0",
        "rsblt r0, r0, #0",
        // store old denominator then make it the unsigned absolute
        "movs  r5, r1",
        "rsblt r1, r1, #0",
        // divmod using unsigned.
        "bl    __aeabi_uidivmod",
        // if signs started opposite, quot becomes negative
        "eors  r12, r4, r5",
        "rsblt r0, r0, #0",
        // if numerator started negative, rem is negative
        "cmp   r4, #0",
        "rsblt r1, r1, #0",
      }),
      "bx    lr",
      options(noreturn)
    )
  }
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
    "b    __aeabi_uread4",
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

/// Provides the `libc` styled memory copy (transfer between exclusive regions)
#[inline]
#[no_mangle]
unsafe extern "C" fn memcpy(
  dest: *mut u8, src: *const u8, byte_count: usize,
) -> *mut u8 {
  __aeabi_memcpy(dest, src, byte_count);
  dest
}

/// Provides the `libc` styled memory move (transfer between non-exclusive
/// regions)
#[inline]
#[no_mangle]
unsafe extern "C" fn memmove(
  dest: *mut u8, src: *const u8, byte_count: usize,
) -> *mut u8 {
  __aeabi_memmove(dest, src, byte_count);
  dest
}

/// Provides the `libc` styled memory set (assign `u8` in `byte` to the entire
/// region)
#[inline]
#[no_mangle]
unsafe extern "C" fn memset(
  dest: *mut u8, byte: i32, byte_count: usize,
) -> *mut u8 {
  __aeabi_memset(dest, byte_count, byte);
  dest
}

extern "C" {
  pub fn __aeabi_memcpy(dest: *mut u8, src: *const u8, byte_count: usize);
  pub fn __aeabi_memcpy4(dest: *mut u8, src: *const u8, byte_count: usize);
  pub fn __aeabi_memcpy8(dest: *mut u8, src: *const u8, byte_count: usize);

  pub fn gba_sram_memcpy(dest: *mut u8, src: *const u8, byte_count: usize);

  pub fn __aeabi_memmove(dest: *mut u8, src: *const u8, byte_count: usize);
  pub fn __aeabi_memmove4(dest: *mut u8, src: *const u8, byte_count: usize);
  pub fn __aeabi_memmove8(dest: *mut u8, src: *const u8, byte_count: usize);

  pub fn __aeabi_memset(dest: *mut u8, byte_count: usize, byte: i32);
  pub fn __aeabi_memset4(dest: *mut u8, byte_count: usize, byte: i32);
  pub fn __aeabi_memset8(dest: *mut u8, byte_count: usize, byte: i32);

  pub fn __aeabi_memclr(dest: *mut u8, byte_count: usize);
  pub fn __aeabi_memclr4(dest: *mut u8, byte_count: usize);
  pub fn __aeabi_memclr8(dest: *mut u8, byte_count: usize);
}

core::arch::global_asm! {
  emit_a32_code!{
    put_code_in_section!(".iwram.aeabi.memory.copy.and.move", {
      "__aeabi_memmove8:",
      "__aeabi_memmove4:",
      "__aeabi_memmove:",
      "cmp    r0, r1", // if d > s, reverse copy
      "bgt    .L_r_copy_gain_align",
      // else fallthrough

      "__aeabi_memcpy:",
      ".L_f_copy_gain_align:",
      "eor    r3, r0, r1",
      "lsls   r3, r3, #31",
      "bmi    .L_f_copy_max_coalign1",
      "bcs    .L_f_copy_max_coalign2",
      // else fallthrough

      ".L_f_copy_max_coalign4:",
      "tst    r0, #3",
      "bne    .L_f_copy_fixup4",
      // else fallthrough

      "__aeabi_memcpy8:",
      "__aeabi_memcpy4:",
      ".L_f_copy_coalign4_assured:",
      "cmp    r2, #32",
      "bge    .L_f_copy_block",

      ".L_f_copy_post_block:",
      // copy 4 words, two at a time
      "tst    r2, #0b10000",
      "ldmne  r1!, {r3, r12}",
      "stmne  r0!, {r3, r12}",
      "ldmne  r1!, {r3, r12}",
      "stmne  r0!, {r3, r12}",
      "bics   r2, r2, #0b10000",
      "bxeq   lr",

      // copy 2 and/or 1 words
      "lsls   r3, r2, #29",
      "ldmcs  r1!, {r3, r12}",
      "stmcs  r0!, {r3, r12}",
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

      ".L_f_copy_block:",
      with_pushed_registers!("{r4-r9}", {
        "1:",
        "subs   r2, r2, #32",
        "ldmge  r1!, {r3-r9, r12}",
        "stmge  r0!, {r3-r9, r12}",
        "bgt    1b",
      }),
      "bxeq   lr",
      "b      .L_f_copy_post_block",

      ".L_f_copy_fixup4:",
      "cmp    r2, #7", // if count <= (fix+word): just byte copy
      "ble    .L_f_copy_max_coalign1",
      "lsls   r3, r0, #31",
      "submi  r2, r2, #1",
      "ldrbmi r3, [r1], #1",
      "strbmi r3, [r0], #1",
      "subcs  r2, r2, #2",
      "ldrhcs r3, [r1], #2",
      "strhcs r3, [r0], #2",
      "b      .L_f_copy_coalign4_assured",

      ".L_f_copy_max_coalign2:",
      "tst     r0, #1",
      "bne     .L_f_copy_fixup2",
      ".L_f_copy_coalign2_assured:",
      "1:",
      "subs    r2, r2, #2",
      "ldrhge  r3, [r1], #2",
      "strhge  r3, [r0], #2",
      "bgt     1b",
      "bxeq    lr",
      "tst     r2, #1",
      "ldrbne  r3, [r1], #1",
      "strbne  r3, [r0], #1",
      "bx      lr",

      ".L_f_copy_fixup2:",
      "cmp     r2, #3", // if count <= (fix+halfword): just byte copy
      "ble     .L_f_copy_max_coalign1",
      "sub     r2, r2, #1",
      "ldrb    r3, [r1], #1",
      "strb    r3, [r0], #1",
      "b       .L_f_copy_coalign2_assured",

      "gba_sram_memcpy:",
      ".L_f_copy_max_coalign1:",
      "1:",
      "subs    r2, r2, #1",
      "ldrbge  r3, [r1], #1",
      "strbge  r3, [r0], #1",
      "bgt     1b",
      "bx      lr",

      ".L_r_copy_gain_align:",
      "add     r0, r0, r2",
      "add     r1, r1, r2",
      "eor     r3, r0, r1",
      "lsls    r3, r3, #31",
      "bmi     .L_r_copy_max_coalign1",
      "bcs     .L_r_copy_max_coalign2",
      // else fallthrough

      ".L_r_copy_max_coalign4:",
      "tst     r0, #3",
      "bne     .L_r_copy_fixup4",
      ".L_r_copy_coalign4_assured:",
      "cmp     r2, #32",
      "bge     .L_r_copy_block",
      ".L_r_copy_post_block:",
      // copy 4 words, two at a time
      "tst     r2, #0b10000",
      "ldmdbne r1!, {r3, r12}",
      "stmdbne r0!, {r3, r12}",
      "ldmdbne r1!, {r3, r12}",
      "stmdbne r0!, {r3, r12}",
      "bics    r2, r2, #0b10000",
      "bxeq    lr",

      // copy 2 and/or 1 words
      "lsls    r3, r2, #29",
      "ldmdbcs r1!, {r3, r12}",
      "stmdbcs r0!, {r3, r12}",
      "ldrmi   r3, [r1, #-4]!",
      "strmi   r3, [r0, #-4]!",
      "bxeq    lr",
      "lsls    r2, r2, #31",
      "ldrhcs  r3, [r1, #-2]!",
      "strhcs  r3, [r0, #-2]!",
      "ldrbmi  r3, [r1, #-1]!",
      "strbmi  r3, [r0, #-1]!",
      "bx      lr",

      ".L_r_copy_block:",
      with_pushed_registers!("{r4-r9}", {
        "1:",
        "subs    r2, r2, #32",
        "ldmdbcs r1!, {r3-r9, r12}",
        "stmdbcs r0!, {r3-r9, r12}",
        "bgt     1b",
      }),
      "bxeq    lr",
      "b       .L_r_copy_post_block",

      ".L_r_copy_fixup4:",
      "cmp     r2, #7", // if count <= (fix+word): just byte copy
      "ble     .L_r_copy_max_coalign1",
      "lsls    r3, r0, #31",
      "submi   r2, r2, #1",
      "ldrbmi  r3, [r1, #-1]!",
      "strbmi  r3, [r0, #-1]!",
      "subcs   r2, r2, #2",
      "ldrhcs  r3, [r1, #-2]!",
      "strhcs  r3, [r0, #-2]!",
      "b       .L_r_copy_coalign4_assured",

      ".L_r_copy_max_coalign2:",
      "tst     r0, #1",
      "bne     .L_r_copy_fixup2",
      ".L_r_copy_coalign2_assured:",
      "1:",
      "subs    r2, r2, #2",
      "ldrhge  r3, [r1, #-2]!",
      "strhge  r3, [r0, #-2]!",
      "bgt     1b",
      "bxeq    lr",
      "tst     r2, #1",
      "ldrbne  r3, [r1, #-1]!",
      "strbne  r3, [r0, #-1]!",
      "bx      lr",

      ".L_r_copy_fixup2:",
      "cmp     r2, #3", // if count <= (fix+halfword): just byte copy
      "ble     .L_r_copy_max_coalign1",
      "sub     r2, r2, #1",
      "ldrb    r3, [r1, #-1]!",
      "strb    r3, [r0, #-1]!",
      "b       .L_r_copy_coalign2_assured",

      ".L_r_copy_max_coalign1:",
      "1:",
      "subs    r2, r2, #1",
      "ldrbge  r3, [r1, #-1]!",
      "strbge  r3, [r0, #-1]!",
      "bgt     1b",
      "bx      lr",
    }),
  },
  options(raw)
}

core::arch::global_asm! {
  emit_a32_code!{
    put_code_in_section!(".iwram.aeabi.memory.clear.and.set", {
      "__aeabi_memclr8:",
      "__aeabi_memclr4:",
      "mov    r2, #0",
      "mov    r3, #0",
      "b      .L_memset_check_for_block_work",
      "__aeabi_memclr:",
      "mov    r2, #0",
      "__aeabi_memset8:",
      "__aeabi_memset4:",
      "__aeabi_memset:", // r0(dest), r1(count), r2(byte)
      // duplicate the byte across all of r2 and r3
      "and    r2, r2, #0xFF",
      "orr    r2, r2, r2, lsl #8",
      "orr    r2, r2, r2, lsl #16",
      "mov    r3, r2",
      // for 'sets' too small to fixup we just byte loop
      "cmp    r1, #3",
      "ble    .L_memset_byte_loop",
      // carry/sign test on the address, then do fixup
      "lsls   r12, r0, #31",
      "submi  r1, r1, #1",
      "strbmi r2, [r0], #1",
      "subcs  r1, r1, #2",
      "strhcs r2, [r0], #2",
      ".L_memset_check_for_block_work:",
      "cmp    r1, #32",
      "bge    .L_memset_block_work",

      ".L_memset_post_block_work:",
      // set 4 words
      "tst    r1, #0b10000",
      "stmne  r0!, {r2, r3}",
      "stmne  r0!, {r2, r3}",
      // set 2 and/or 1 words
      "lsls   r12, r1, #29",
      "stmcs  r0!, {r2, r3}",
      "strmi  r2, [r0], #4",
      // set halfword and/or byte
      "lsls   r12, r1, #31",
      "strhcs r2, [r0], #2",
      "strbmi r2, [r0], #1",
      "bx     lr",

      ".L_memset_block_work:",
      with_pushed_registers!("{r4-r9}", {
        "mov    r4, r2",
        "mov    r5, r2",
        "mov    r6, r2",
        "mov    r7, r2",
        "mov    r8, r2",
        "mov    r9, r2",
        "1:",
        "subs   r1, r1, #32",
        "stmge  r0!, {r2-r9}",
        "bgt    1b",
      }),
      "bxeq   lr",
      "b      .L_memset_post_block_work",

      ".L_memset_byte_loop:",
      "1:",
      "subs   r1, r1, #1",
      "strbcs r2, [r0], #1",
      "bgt    1b",
      "bx     lr",
    }),
  },
  options(raw),
}
