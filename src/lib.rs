#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), feature(asm))]
#![warn(missing_docs)]
//#![allow(clippy::cast_lossless)]
#![deny(clippy::float_arithmetic)]

//! This crate helps you write GBA ROMs.
//!
//! # SAFETY POLICY
//!
//! Some parts of this crate are safe wrappers around unsafe operations. This is
//! good, and what you'd expect from a Rust crate.
//!
//! However, the safe wrappers all assume that you will _only_ attempt to
//! execute this crate on a GBA or in a GBA Emulator.
//!
//! **Do not** use this crate in programs that aren't running on the GBA. If you
//! do, it's a giant bag of Undefined Behavior.
//!
//! # TESTING POLICY
//!
//! It is the intent of the crate authors that as much of the crate as possible
//! be written so that you can use `cargo test` for at least some parts of your
//! code without everything exploding instantly. To that end, where possible we
//! attempt to use `cfg` flags to make things safe for `cargo test`. Hopefully
//! we got it all.

pub mod core_extras;
pub(crate) use crate::core_extras::*;

#[cfg(not(test))]
pub mod bios;

pub mod io_registers;

pub mod video_ram;
pub(crate) use crate::video_ram::*;
