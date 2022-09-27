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
