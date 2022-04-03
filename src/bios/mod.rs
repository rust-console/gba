#![allow(non_snake_case)]

//! Binds the various BIOS functions for use by Rust.
//!
//! The BIOS function code is stored in the GBA hardware itself, you don't need
//! to link any special files into your program to have them available.
//!
//! BIOS functions are called by using a "software interrupt" instruction
//! (`swi`) instead of the usual branch-exchange. This means that there's
//! significantly more overhead when calling BIOS functions compared to normal
//! functions. That said, some of the BIOS functions are useful enough to
//! justify this extra call overhead.
//!
//! All of the BIOS function definitions here use the same names as given by
//! [GBATEK](https://problemkaputt.de/gbatek.htm#biosfunctions). This means that
//! they're all in `PascalCase` instead of the Rust-style `snake_case`.

use core::arch::asm;

use crate::{
  interrupts::IrqBits,
  macros::{const_new, u8_bool_field},
};

pub mod affine_setup;
pub mod arithmetic;
pub mod decompression;
pub mod halt;
pub mod memory_copy;
pub mod reset;

/*
TODO:

MultiBoot

MidiKey2Freq
SoundBias
SoundChannelClear
SoundDriverInit
SoundDriverMain
SoundDriverMode
SoundDriverVSync
SoundDriverVSyncOff
SoundDriverVSyncOn

*/
