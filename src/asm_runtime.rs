use crate::{
  dma::DmaControl,
  gba_cell::GbaCell,
  mmio::{DMA3_SRC, IME},
  IrqFn,
};

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

/// The function that the assembly runtime calls when an interrupt occurs.
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

    /* TODO: set the stack pointers here in case of main returning to the rom base? */

    /* iwram copy */
    "ldr r4, =__iwram_word_copy_count",
    "cmp r4, #0",
    "beq 1f",
    "add r3, r12, #{dma3_offset}",
    "mov r5, #{dma3_setting}",
    "ldr r0, =__iwram_start",
    "ldr r2, =__iwram_position_in_rom",
    "str r2, [r3]", /* source */
    "str r0, [r3, #4]", /* destination */
    "strh r4, [r3, #8]", /* word count */
    "strh r5, [r3, #10]", /* set control bits */
    "1:", /* post iwram copy */

    /* ewram copy */
    "ldr r4, =__ewram_word_copy_count",
    "cmp r4, #0",
    "beq 1f",
    "add r3, r12, #{dma3_offset}",
    "mov r5, #{dma3_setting}",
    "ldr r0, =__ewram_start",
    "ldr r2, =__ewram_position_in_rom",
    "str r2, [r3]", /* source */
    "str r0, [r3, #4]", /* destination */
    "strh r4, [r3, #8]", /* word count */
    "strh r5, [r3, #10]", /* set control bits */
    "1:", /* post ewram copy */

    /* bss zero */
    "ldr r4, =__bss_word_clear_count",
    "cmp r4, #0",
    "beq 1f",
    "ldr r0, =__bss_start",
    "mov r2, #0",
    "2:",
    "str r2, [r0], #4",
    "subs r4, r4, #1",
    "bne 2b",
    "1:", /* post bss zero */

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
    /*handle_irq_with_interrupts_off*/
    "add r12, r0, #{ime_offset}",
    "mov r3, #0",
    "swp r3, r3, [r12]",
    /* Still Important
    * r12, IME
    * r3, ime_previous
    */

    /*read_update_hardware_flags*/
    "ldr r0, [r12, #-8]",       // read IE_IF
    "and r0, r0, r0, LSR #16",  // combine
    "strh r0, [r12, #-6]",      // write IF
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r0, irq_flags
    */

    /*read_update_bios_flags*/
    "sub  r2, r12, #(0x208+8)",
    "ldrh r1, [r2]",
    "orr  r1, r1, r0",
    "strh r1, [r2]",
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r0, irq_flags
    */

    /*get_rust_fn_ptr*/
    "ldr r1, ={RUST_IRQ_HANDLER}",
    "ldr r1, [r1]",       //r1==RUST_IRQ_HANDLER
    "cmp r1, #0",         //if r1==0
    "beq 9f",             //then skip
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r1, rust_irq_fn
    * r0, irq_flags
    */

    /*call_rust_fn_in_sys_mode*/
    "mrs r2, SPSR",      //save SPSR

    set_cpu_control!(System, irq_masked: false, fiq_masked: false),

    with_pushed_registers!("{{r2, r3, r12, lr}}",{
      adr_lr_then_bx_to!(reg="r1", label_id="1")
    }),

    set_cpu_control!(Supervisor, irq_masked: true, fiq_masked: false),

    "msr SPSR, r2",
    /* Still Important
    * r3, ime_previous
    */

    /*end_of_rt0*/
    "9:",
    "swp r3, r3, [r12]", // IME swap previous
    "bx lr",
    ime_offset = const IME_OFFSET,
    RUST_IRQ_HANDLER = sym RUST_IRQ_HANDLER,
    options(noreturn)
  )
}
