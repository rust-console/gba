//! A module that just re-exports all the other modules of the crate.

pub use crate::{
  asm_runtime::*, bios::*, builtin_art::*, dma::*, gba_cell::*, interrupts::*,
  keys::*, mmio::*, sound::*, timers::*, video::*,
};
