#![no_std]

//! The crate for writing GBA games.
//!
//! ## Safety
//! All safety is considered assuming that you build for the
//! `thumbv4t-none-eabi` target and then run code on a GBA. For example, MMIO
//! address safety is considered only for the GBA. If you use this crate on any
//! other device there are countless ways for things to go wrong.

mod macros;

mod bit_utils;
pub use bit_utils::*;

mod bios;
pub use bios::*;

mod gba_cell;
pub use gba_cell::*;

mod color;
pub use color::*;

mod display_control;
pub use display_control::*;

mod interrupts;
pub use interrupts::*;

mod key_input;
pub use key_input::*;

core::arch::global_asm!(include_str!("header_and_runtime.S"));
