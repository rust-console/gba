#![no_std]
#![feature(isa_attribute)]

//! The crate for writing GBA games.
//!
//! ## Safety
//!
//! The safety of all `unsafe` code within this crate assumes that you're
//! building using the `thumbv4t-none-eabi` target, using our build script and
//! runtime, and then running the code on a GBA. In all other situations, this
//! crate is very likely to be wildly unsound.

mod macros;

pub mod audio;
pub mod bios;
pub mod dma;
pub mod interrupts;
pub mod key_input;
pub mod save;
pub mod timers;
pub mod video;

pub mod debugging;
pub mod random;
pub mod sync;

pub mod prelude;

#[doc(hidden)]
pub mod bit_utils;

#[no_mangle]
#[allow(dead_code)]
extern "C" fn there_can_be_only_one_version_of_the_lib_in_the_build() {}

core::arch::global_asm!(include_str!("assembly_runtime.s"), options(raw));
