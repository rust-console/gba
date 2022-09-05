#![no_std]
#![feature(asm_sym)]
#![feature(asm_const)]
#![feature(isa_attribute)]
#![feature(naked_functions)]
#![allow(soft_unstable)]

mod macros;

pub mod mmio;

pub mod bios;
pub mod gba_cell;
pub mod interrupts;
pub mod keys;
pub mod runtime;
pub mod video;

/// A function you want called during an interrupt.
pub type IrqFn = unsafe extern "C" fn(u16);
