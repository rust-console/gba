//! Lets you interact with the mGBA debug output buffer.
//!
//! This buffer works as a "standard output" sort of interface:
//! * First `use core::fmt::Write;` so that the [`Write`](core::fmt::Write)
//!   trait is in scope.
//! * Try to make a logger with `MgbaBufferedLogger::try_new(log_level)`.
//! * Use the `write!` macro to write data into the logger.
//! * The logger will automatically flush itself (using the log level you set)
//!   when the buffer is full, on a newline, and when it's dropped.
//!
//! Logging is not always available. Obviously the mGBA output buffer can't be
//! used if the game isn't running within the mGBA emulator.
//! [`MgbaBufferedLogger::try_new`] will fail to make a logger when logging
//! isn't available. You can also call [`mgba_logging_available`] directly to
//! check if mGBA logging is possible.
//!
//! ```no_run
//! # use gba::prelude::*;
//! use core::fmt::Write;
//! let log_level = MgbaMessageLevel::Debug;
//! if let Ok(logger) = MgbaBufferedLogger::try_new(log_level) {
//!   writeln!(logger, "hello").ok();
//! }
//! ```
//!
//! ## Fine Details
//! Even when the program is running within mGBA, the [`MGBA_LOG_ENABLE`]
//! address needs to be written with the [`MGBA_LOGGING_ENABLE_REQUEST`] value
//! to allow logging. This is automatically done for you by the assembly
//! runtime. If the `MGBA_LOG_ENABLE` address reads back
//! [`MGBA_LOGGING_ENABLE_RESPONSE`] then mGBA logging is possible. If you're
//! running outside of mGBA then the `MGBA_LOG_ENABLE` address maps to nothing.
//! Writes will do no harm, and reads won't read the correct value.
//!
//! Once you know that logging is possible, write your message to
//! [`MGBA_LOG_BUFFER`]. This works similar to a C-style string: the first 0
//! byte in the buffer will be considered the end of the message.
//!
//! When the message is ready to go out, write a message level to
//! [`MGBA_LOG_SEND`]. This makes the message available within the emulator's
//! logs at that message level and also implicitly zeroes the message buffer so
//! that it's ready for the next message.

use crate::mmio::{MGBA_LOG_BUFFER, MGBA_LOG_ENABLE, MGBA_LOG_SEND};

pub const MGBA_LOGGING_ENABLE_REQUEST: u16 = 0xC0DE;

pub const MGBA_LOGGING_ENABLE_RESPONSE: u16 = 0x1DEA;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum MgbaMessageLevel {
  /// Warning! This causes mGBA to halt emulation!
  Fatal = 0x100,
  Error = 0x101,
  Warning = 0x102,
  Info = 0x103,
  #[default]
  Debug = 0x104,
}

/// Returns if mGBA logging is possible.
#[inline]
pub fn mgba_logging_available() -> bool {
  // the `__start` function writes the request, so here we just check success.
  MGBA_LOG_ENABLE.read() == MGBA_LOGGING_ENABLE_RESPONSE
}

pub struct MgbaBufferedLogger {
  byte_count: u8,
  pub message_level: MgbaMessageLevel,
}
impl MgbaBufferedLogger {
  pub fn try_new(message_level: MgbaMessageLevel) -> Result<Self, ()> {
    if mgba_logging_available() {
      Ok(Self { byte_count: 0, message_level })
    } else {
      Err(())
    }
  }
  fn flush(&mut self) {
    MGBA_LOG_SEND.write(self.message_level);
    self.byte_count = 0;
  }
}
impl Drop for MgbaBufferedLogger {
  fn drop(&mut self) {
    if self.byte_count != 0 {
      self.flush();
    }
  }
}
impl core::fmt::Write for MgbaBufferedLogger {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    for b in s.as_bytes().iter().copied() {
      if b == b'\n' {
        self.flush();
      } else {
        MGBA_LOG_BUFFER.index(self.byte_count as usize).write(b);
        if self.byte_count == u8::MAX {
          self.flush();
        } else {
          self.byte_count += 1;
        }
      }
    }
    Ok(())
  }
}
