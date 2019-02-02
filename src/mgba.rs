//! Special utils for if you're running on the mGBA emulator.
//!
//! Note that this assumes that you're using the very latest version (0.7). If
//! you've got some older version of things there might be any number of
//! differences or problems.

use super::*;

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

/// Allows writing to the `mGBA` debug output.
#[derive(Debug, PartialEq, Eq)]
pub struct MGBADebug {
  bytes_written: u8,
}
impl MGBADebug {
  const ENABLE_ADDRESS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x4fff780) };
  const ENABLE_ADDRESS_INPUT: u16 = 0xC0DE;
  const ENABLE_ADDRESS_OUTPUT: u16 = 0x1DEA;

  const OUTPUT_BASE: VolAddress<u8> = unsafe { VolAddress::new_unchecked(0x4fff600) };

  const SEND_ADDRESS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x4fff700) };
  const SEND_FLAG: u16 = 0x100;

  /// Gives a new MGBADebug, if running within `mGBA`
  ///
  /// # Fails
  ///
  /// If you're not running in the `mGBA` emulator.
  pub fn new() -> Option<Self> {
    Self::ENABLE_ADDRESS.write(Self::ENABLE_ADDRESS_INPUT);
    if Self::ENABLE_ADDRESS.read() == Self::ENABLE_ADDRESS_OUTPUT {
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
      Self::SEND_ADDRESS.write(Self::SEND_FLAG | MGBADebugLevel::Error as u16);

      // Note(Lokathor): A Fatal send causes the emulator to halt!
      Self::SEND_ADDRESS.write(Self::SEND_FLAG | MGBADebugLevel::Fatal as u16);
    } else {
      Self::SEND_ADDRESS.write(Self::SEND_FLAG | level as u16);
      self.bytes_written = 0;
    }
  }
}

impl core::fmt::Write for MGBADebug {
  fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
    unsafe {
      let mut current = Self::OUTPUT_BASE.offset(self.bytes_written as isize);
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
      Err(core::fmt::Error)
    }
  }
}
