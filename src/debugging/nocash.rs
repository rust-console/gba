//! Special utils for if you're running on the NO$GBA emulator.
//!
//! Note that this assumes that you're using the very latest version (3.03). If
//! you've got some older version of things there might be any number of
//! differences or problems.

use super::{DebugInterface, DebugLevel};
use crate::prelude::InitOnce;
use core::fmt::{Arguments, Write};
use voladdress::*;

const CHAR_OUT: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x04FFFA1C) };
const SIGNATURE_ADDR: VolBlock<u8, Safe, Safe, 16> = unsafe { VolBlock::new(0x04FFFA00) };

const SIGNATURE: [u8; 7] = *b"no$gba ";
static NO_CASH_DEBUGGING: InitOnce<bool> = InitOnce::new();

/// Returns whether we are running in `NO$GBA`.
#[inline(never)]
pub fn detect() -> bool {
  *NO_CASH_DEBUGGING.get(|| {
    for i in 0..7 {
      if SIGNATURE_ADDR.index(i).read() != SIGNATURE[i] {
        return false;
      }
    }
    true
  })
}

/// Allows writing to the `NO$GBA` debug output.
#[derive(Debug, PartialEq, Eq)]
pub struct NoCashDebug(());
impl NoCashDebug {
  /// Gives a new NoCashDebug, if running within `NO$GBA`
  ///
  /// # Fails
  ///
  /// If you're not running in the `NO$GBA` emulator.
  pub fn new() -> Option<Self> {
    if detect() {
      Some(NoCashDebug(()))
    } else {
      None
    }
  }
}
impl core::fmt::Write for NoCashDebug {
  fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
    for b in s.bytes() {
      CHAR_OUT.write(b);
    }
    Ok(())
  }
}

/// The [`DebugInterface`] for `NO$GBA`.
pub struct NoCashDebugInterface;
impl DebugInterface for NoCashDebugInterface {
  fn device_attached(&self) -> bool {
    detect()
  }

  fn debug_print(&self, debug: DebugLevel, args: &Arguments<'_>) -> Result<(), core::fmt::Error> {
    if let Some(mut out) = NoCashDebug::new() {
      write!(out, "User: [{:?}] {}\n", debug, args)?;
      if debug == DebugLevel::Fatal {
        super::crash();
      }
    }
    Ok(())
  }
}
