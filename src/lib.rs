#![no_std]
#![feature(asm_sym)]
#![feature(asm_const)]
#![feature(isa_attribute)]
#![feature(naked_functions)]

mod macros;

pub mod asm_runtime;
pub mod bios;
pub mod dma;
pub mod gba_cell;
pub mod interrupts;
pub mod keys;
pub mod mmio;
pub mod sound;
pub mod timers;
pub mod video;

/// A function you want called during an interrupt.
pub type IrqFn = unsafe extern "C" fn(u16);
