//! This module holds the assembly runtime that supports your Rust program.
//!
//! Most importantly, you can set the [`RUST_IRQ_HANDLER`] variable to assign
//! which function should be run during a hardware interrupt.
//! * When a hardware interrupt occurs, control first goes to the BIOS, which
//!   will then call the assembly runtime's handler.
//! * The assembly runtime handler will properly acknowledge the interrupt
//!   within the system on its own without you having to do anything.
//! * If a function is set in the `RUST_IRQ_HANDLER` variable then that function
//!   will be called and passed the bits for which interrupt(s) occurred.

use core::ffi::c_void;

use crate::{
  dma::DmaControl,
  gba_cell::GbaCell,
  interrupts::IrqFn,
  mgba::MGBA_LOGGING_ENABLE_REQUEST,
  mmio::{DMA3_SRC, IME, MGBA_LOG_ENABLE},
};
use bracer::*;

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
    "ldr r1, ={runtime_irq_handler}",
    "str r1, [r12, #-4]",

    /* ask for mGBA logging to be enabled. This should be harmless if we're not using mgba. */
    "ldr r0, ={mgba_log_enable}",
    "ldr r1, ={mgba_logging_enable_request}",
    "str r1, [r0]",

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
    runtime_irq_handler = sym runtime_irq_handler,
    mgba_log_enable = const MGBA_LOG_ENABLE.as_usize(),
    mgba_logging_enable_request = const MGBA_LOGGING_ENABLE_REQUEST,
    options(noreturn)
  )
}

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.runtime.irq.handler"]
unsafe extern "C" fn runtime_irq_handler() {
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
      with_spsr_held_in!("r2", {
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
