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
pub mod mgba;
pub mod mmio;
pub mod prelude;
pub mod sound;
pub mod timers;
pub mod video;
