//! Special utils for if you're running on the mGBA emulator.
//!
//! Note that this assumes that you're using the very latest version (0.7). If
//! you've got some older version of things there might be any number of
//! differences or problems.

use super::{DebugInterface, DebugLevel};
use crate::sync::InitOnce;
use core::fmt::{Arguments, Write};
use voladdress::VolAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
#[allow(missing_docs)]
pub enum MGBADebugLevel {
  /// Warning! This causes the emulator to halt emulation!
  Fatal = 0,
  Error = 1,
  Warning = 2,
  Info = 3,
  Debug = 4,
}

// MGBADebug related addresses.
const ENABLE_ADDRESS: VolAddress<u16> = unsafe { VolAddress::new(0x4fff780) };
const ENABLE_ADDRESS_INPUT: u16 = 0xC0DE;
const ENABLE_ADDRESS_OUTPUT: u16 = 0x1DEA;

const OUTPUT_BASE: VolAddress<u8> = unsafe { VolAddress::new(0x4fff600) };

const SEND_ADDRESS: VolAddress<u16> = unsafe { VolAddress::new(0x4fff700) };
const SEND_FLAG: u16 = 0x100;

// Only enable MGBA debugging once.
static MGBA_DEBUGGING: InitOnce<bool> = InitOnce::new();

/// Returns whether we are running in mGBA.
#[inline(never)]
pub fn detect() -> bool {
  *MGBA_DEBUGGING.get(|| {
    ENABLE_ADDRESS.write(ENABLE_ADDRESS_INPUT);
    ENABLE_ADDRESS.read() == ENABLE_ADDRESS_OUTPUT
  })
}

/// Allows writing to the `mGBA` debug output.
#[derive(Debug, PartialEq, Eq)]
pub struct MGBADebug {
  bytes_written: u8,
}
impl MGBADebug {
  /// Gives a new MGBADebug, if running within `mGBA`
  ///
  /// # Fails
  ///
  /// If you're not running in the `mGBA` emulator.
  pub fn new() -> Option<Self> {
    if detect() {
      Some(MGBADebug { bytes_written: 0 })
    } else {
      None
    }
  }

  /// Once output is buffered you must send it out with a level.
  ///
  /// If the `Fatal` level is selected, the buffer is sent out as `Error`
  /// followed by a blank message being sent as `Error`. This is done because
  /// the `Fatal` message appears in a popup without showing up in the log, so
  /// it might accidentally be discarded.
  pub fn send(&mut self, level: MGBADebugLevel) {
    if level == MGBADebugLevel::Fatal {
      // Note(Lokathor): A Fatal send causes the emulator to halt!
      SEND_ADDRESS.write(SEND_FLAG | MGBADebugLevel::Fatal as u16);
    } else {
      SEND_ADDRESS.write(SEND_FLAG | level as u16);
      self.bytes_written = 0;
    }
  }
}

impl core::fmt::Write for MGBADebug {
  fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
    unsafe {
      let mut current = OUTPUT_BASE.offset(self.bytes_written as isize);
      let mut str_iter = s.bytes();
      while self.bytes_written < 255 {
        match str_iter.next() {
          Some(byte) => {
            current.write(byte);
            current = current.offset(1);
            self.bytes_written += 1;
          }
          None => return Ok(()),
        }
      }
      Ok(())
    }
  }
}

/// The [`DebugInterface`] for MGBA.
pub struct MGBADebugInterface;
impl DebugInterface for MGBADebugInterface {
  fn device_attached(&self) -> bool {
    detect()
  }

  fn debug_print(&self, debug: DebugLevel, args: &Arguments<'_>) -> Result<(), core::fmt::Error> {
    if let Some(mut out) = MGBADebug::new() {
      write!(out, "{}", args)?;
      out.send(match debug {
        DebugLevel::Fatal => MGBADebugLevel::Fatal,
        DebugLevel::Error => MGBADebugLevel::Error,
        DebugLevel::Warning => MGBADebugLevel::Warning,
        DebugLevel::Info => MGBADebugLevel::Info,
        DebugLevel::Debug => MGBADebugLevel::Debug,
      });
      if debug == DebugLevel::Fatal {
        super::crash();
      }
    }
    Ok(())
  }
}
