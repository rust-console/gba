//! A module that just re-exports all the other modules of the crate.

#[cfg(feature = "on_gba")]
pub use crate::{
  asm_runtime::*, bios::*, dma::*, gba_cell::*, mgba::*, mmio::*,
  RUST_IRQ_HANDLER,
};

pub use crate::{
  builtin_art::*,
  fixed::*,
  include_aligned_bytes,
  interrupts::*,
  keys::*,
  sound::*,
  timers::*,
  video::{obj::*, *},
  Align4,
};
