//! A module that just re-exports all the other modules of the crate.

pub use crate::{
  asm_runtime::*,
  bios::*,
  builtin_art::*,
  dma::*,
  fixed::*,
  gba_cell::*,
  include_aligned_bytes,
  interrupts::*,
  keys::*,
  mgba::*,
  mmio::*,
  sound::*,
  timers::*,
  video::{obj::*, *},
  Align4,
};
