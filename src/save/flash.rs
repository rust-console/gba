//! Module for flash save media support.
//!
//! Flash may be read with ordinary read commands, but writing requires
//! sending structured commands to the flash chip.

use super::{
  lock_media, read_raw_buf, read_raw_byte, verify_raw_buf, Error, MediaInfo, MediaType,
  RawSaveAccess,
};
use crate::sync::{with_irqs_disabled, InitOnce, Static};
use core::cmp;
use voladdress::*;

// Volatile address ports for flash
const FLASH_PORT_BANK: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x0E000000) };
const FLASH_PORT_A: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x0E005555) };
const FLASH_PORT_B: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x0E002AAA) };
const FLASH_DATA: VolBlock<u8, Safe, Safe, 65536> = unsafe { VolBlock::new(0x0E000000) };

// Various constants related to sector sizes
const BANK_SHIFT: usize = 16; // 64 KiB
const BANK_LEN: usize = 1 << BANK_SHIFT;
const BANK_MASK: usize = BANK_LEN - 1;

// Constants relating to flash commands.
const CMD_SET_BANK: u8 = 0xB0;
const CMD_READ_CHIP_ID: u8 = 0x90;
const CMD_READ_CONTENTS: u8 = 0xF0;
const CMD_WRITE: u8 = 0xA0;
const CMD_ERASE_SECTOR_BEGIN: u8 = 0x80;
const CMD_ERASE_SECTOR_CONFIRM: u8 = 0x30;
const CMD_ERASE_SECTOR_ALL: u8 = 0x10;

/// Starts a command to the flash chip.
fn start_flash_command() {
  FLASH_PORT_A.write(0xAA);
  FLASH_PORT_B.write(0x55);
}

/// Helper function for issuing commands to the flash chip.
fn issue_flash_command(c2: u8) {
  start_flash_command();
  FLASH_PORT_A.write(c2);
}

/// A simple thing to avoid excessive bank switches
static CURRENT_BANK: Static<u8> = Static::new(!0);
fn set_bank(bank: u8) -> Result<(), Error> {
  if bank == 0xFF {
    Err(Error::OutOfBounds)
  } else if bank != CURRENT_BANK.read() {
    issue_flash_command(CMD_SET_BANK);
    FLASH_PORT_BANK.write(bank as u8);
    CURRENT_BANK.write(bank);
    Ok(())
  } else {
    Ok(())
  }
}

/// Identifies a particular f
/// lash chip in use by a Game Pak.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum FlashChipType {
  /// 64KiB SST chip
  Sst64K,
  /// 64KiB Macronix chip
  Macronix64K,
  /// 64KiB Panasonic chip
  Panasonic64K,
  /// 64KiB Atmel chip
  Atmel64K,
  /// 128KiB Sanyo chip
  Sanyo128K,
  /// 128KiB Macronix chip
  Macronix128K,
  /// An unidentified chip
  Unknown,
}
impl FlashChipType {
  /// Returns the type of the flash chip currently in use.
  pub fn detect() -> Result<Self, Error> {
    Ok(Self::from_id(detect_chip_id()?))
  }

  /// Determines the flash chip type from an ID.
  pub fn from_id(id: u16) -> Self {
    match id {
      0xD4BF => FlashChipType::Sst64K,
      0x1CC2 => FlashChipType::Macronix64K,
      0x1B32 => FlashChipType::Panasonic64K,
      0x3D1F => FlashChipType::Atmel64K,
      0x1362 => FlashChipType::Sanyo128K,
      0x09C2 => FlashChipType::Macronix128K,
      _ => FlashChipType::Unknown,
    }
  }

  /// Returns the `u16` id of the chip, or `0` for `Unknown`.
  pub fn id(&self) -> u16 {
    match *self {
      FlashChipType::Sst64K => 0xD4BF,
      FlashChipType::Macronix64K => 0x1CC2,
      FlashChipType::Panasonic64K => 0x1B32,
      FlashChipType::Atmel64K => 0x3D1F,
      FlashChipType::Sanyo128K => 0x1362,
      FlashChipType::Macronix128K => 0x09C2,
      FlashChipType::Unknown => 0x0000,
    }
  }
}

/// Determines the raw ID of the flash chip currently in use.
pub fn detect_chip_id() -> Result<u16, Error> {
  let _lock = lock_media()?;
  issue_flash_command(CMD_READ_CHIP_ID);
  let high = unsafe { read_raw_byte(0x0E000001) };
  let low = unsafe { read_raw_byte(0x0E000000) };
  let id = (high as u16) << 8 | low as u16;
  issue_flash_command(CMD_READ_CONTENTS);
  Ok(id)
}

/// Information relating to a particular flash chip that could be found in a
/// Game Pak.
#[allow(dead_code)]
struct ChipInfo {
  /// The wait state required to read from the chip.
  read_wait: u8,
  /// The wait state required to write to the chip.
  write_wait: u8,

  /// The timeout in milliseconds for writes to this chip.
  write_timeout: u16,
  /// The timeout in milliseconds for erasing a sector in this chip.
  erase_sector_timeout: u16,
  /// The timeout in milliseconds for erasing the entire chip.
  erase_chip_timeout: u16,

  /// The number of 64KiB banks in this chip.
  bank_count: u8,
  /// Whether this is an Atmel chip, which has 128 byte sectors instead of 4K.
  uses_atmel_api: bool,
  /// Whether this is an Macronix chip, which requires an additional command
  /// to cancel the current action after a timeout.
  requires_cancel_command: bool,

  /// The [`MediaInfo`] to return for this chip type.
  info: &'static MediaInfo,
}

// Media info for the various chipsets.
static INFO_64K: MediaInfo = MediaInfo {
  media_type: MediaType::Flash64K,
  sector_shift: 12, // 4 KiB
  sector_count: 16, // 4 KiB * 16 = 64 KiB
  requires_prepare_write: true,
};
static INFO_64K_ATMEL: MediaInfo = MediaInfo {
  media_type: MediaType::Flash64K,
  sector_shift: 7,   // 128 bytes
  sector_count: 512, // 128 bytes * 512 = 64 KiB
  requires_prepare_write: false,
};
static INFO_128K: MediaInfo = MediaInfo {
  media_type: MediaType::Flash128K,
  sector_shift: 12,
  sector_count: 32, // 4 KiB * 32 = 128 KiB
  requires_prepare_write: true,
};

// Chip info for the various chipsets.
static CHIP_INFO_SST_64K: ChipInfo = ChipInfo {
  read_wait: 2,  // 2 cycles
  write_wait: 1, // 3 cycles
  write_timeout: 10,
  erase_sector_timeout: 40,
  erase_chip_timeout: 200,
  bank_count: 1,
  uses_atmel_api: false,
  requires_cancel_command: false,
  info: &INFO_64K,
};
static CHIP_INFO_MACRONIX_64K: ChipInfo = ChipInfo {
  read_wait: 1,  // 3 cycles
  write_wait: 3, // 8 cycles
  write_timeout: 10,
  erase_sector_timeout: 2000,
  erase_chip_timeout: 2000,
  bank_count: 1,
  uses_atmel_api: false,
  requires_cancel_command: true,
  info: &INFO_64K,
};
static CHIP_INFO_PANASONIC_64K: ChipInfo = ChipInfo {
  read_wait: 2,  // 2 cycles
  write_wait: 0, // 4 cycles
  write_timeout: 10,
  erase_sector_timeout: 500,
  erase_chip_timeout: 500,
  bank_count: 1,
  uses_atmel_api: false,
  requires_cancel_command: false,
  info: &INFO_64K,
};
static CHIP_INFO_ATMEL_64K: ChipInfo = ChipInfo {
  read_wait: 3,  // 8 cycles
  write_wait: 3, // 8 cycles
  write_timeout: 40,
  erase_sector_timeout: 40,
  erase_chip_timeout: 40,
  bank_count: 1,
  uses_atmel_api: true,
  requires_cancel_command: false,
  info: &INFO_64K_ATMEL,
};
static CHIP_INFO_GENERIC_64K: ChipInfo = ChipInfo {
  read_wait: 3,  // 8 cycles
  write_wait: 3, // 8 cycles
  write_timeout: 40,
  erase_sector_timeout: 2000,
  erase_chip_timeout: 2000,
  bank_count: 1,
  uses_atmel_api: false,
  requires_cancel_command: true,
  info: &INFO_128K,
};
static CHIP_INFO_GENERIC_128K: ChipInfo = ChipInfo {
  read_wait: 1,  // 3 cycles
  write_wait: 3, // 8 cycles
  write_timeout: 10,
  erase_sector_timeout: 2000,
  erase_chip_timeout: 2000,
  bank_count: 2,
  uses_atmel_api: false,
  requires_cancel_command: false,
  info: &INFO_128K,
};

impl FlashChipType {
  /// Returns the internal info for this chip.
  fn chip_info(&self) -> &'static ChipInfo {
    match *self {
      FlashChipType::Sst64K => &CHIP_INFO_SST_64K,
      FlashChipType::Macronix64K => &CHIP_INFO_MACRONIX_64K,
      FlashChipType::Panasonic64K => &CHIP_INFO_PANASONIC_64K,
      FlashChipType::Atmel64K => &CHIP_INFO_ATMEL_64K,
      FlashChipType::Sanyo128K => &CHIP_INFO_GENERIC_128K,
      FlashChipType::Macronix128K => &CHIP_INFO_GENERIC_128K,
      FlashChipType::Unknown => &CHIP_INFO_GENERIC_64K,
    }
  }
}
static CHIP_INFO: InitOnce<&'static ChipInfo> = InitOnce::new();
fn cached_chip_info() -> Result<&'static ChipInfo, Error> {
  CHIP_INFO
    .try_get(|| -> Result<_, Error> { Ok(FlashChipType::detect()?.chip_info()) })
    .map(Clone::clone)
}

/// Actual implementation of the ChipInfo functions.
impl ChipInfo {
  /// Returns the total length of this chip.
  fn total_len(&self) -> usize {
    self.info.sector_count << self.info.sector_shift
  }

  // Checks whether a byte offset is in bounds.
  fn check_len(&self, offset: usize, len: usize) -> Result<(), Error> {
    if offset.checked_add(len).is_some() && offset + len <= self.total_len() {
      Ok(())
    } else {
      Err(Error::OutOfBounds)
    }
  }

  // Checks whether a sector offset is in bounds.
  fn check_sector_len(&self, offset: usize, len: usize) -> Result<(), Error> {
    if offset.checked_add(len).is_some() && offset + len <= self.info.sector_count {
      Ok(())
    } else {
      Err(Error::OutOfBounds)
    }
  }

  /// Sets the currently active bank.
  fn set_bank(&self, bank: usize) -> Result<(), Error> {
    if bank >= self.bank_count as usize {
      Err(Error::OutOfBounds)
    } else if self.bank_count > 1 {
      set_bank(bank as u8)
    } else {
      Ok(())
    }
  }

  /// Reads a buffer from save media into memory.
  fn read_buffer(&self, mut offset: usize, mut buf: &mut [u8]) -> Result<(), Error> {
    while buf.len() != 0 {
      self.set_bank(offset >> BANK_SHIFT)?;
      let start = offset & BANK_MASK;
      let end_len = cmp::min(BANK_LEN - start, buf.len());
      unsafe {
        read_raw_buf(&mut buf[..end_len], 0x0E000000 + start);
      }
      buf = &mut buf[end_len..];
      offset += end_len;
    }
    Ok(())
  }

  /// Verifies that a buffer was properly stored into save media.
  fn verify_buffer(&self, mut offset: usize, mut buf: &[u8]) -> Result<bool, Error> {
    while buf.len() != 0 {
      self.set_bank(offset >> BANK_SHIFT)?;
      let start = offset & BANK_MASK;
      let end_len = cmp::min(BANK_LEN - start, buf.len());
      if !unsafe { verify_raw_buf(&buf[..end_len], 0x0E000000 + start) } {
        return Ok(false);
      }
      buf = &buf[end_len..];
      offset += end_len;
    }
    Ok(true)
  }

  /// Waits for a timeout, or an operation to complete.
  fn wait_for_timeout(&self, offset: usize, val: u8, ms: u16) -> Result<(), Error> {
    let timeout = super::Timeout::new()?;
    timeout.start();
    let offset = 0x0E000000 + offset;

    while unsafe { read_raw_byte(offset) != val } {
      if timeout.is_timeout_met(ms) {
        if self.requires_cancel_command {
          FLASH_PORT_A.write(0xF0);
        }
        return Err(Error::OperationTimedOut);
      }
    }
    Ok(())
  }

  /// Erases a sector to flash.
  fn erase_sector(&self, sector: usize) -> Result<(), Error> {
    let offset = sector << self.info.sector_shift;
    self.set_bank(offset >> BANK_SHIFT)?;
    issue_flash_command(CMD_ERASE_SECTOR_BEGIN);
    start_flash_command();
    FLASH_DATA.index(offset & BANK_MASK).write(CMD_ERASE_SECTOR_CONFIRM);
    self.wait_for_timeout(offset & BANK_MASK, 0xFF, self.erase_sector_timeout)
  }

  /// Erases the entire chip.
  fn erase_chip(&self) -> Result<(), Error> {
    issue_flash_command(CMD_ERASE_SECTOR_BEGIN);
    issue_flash_command(CMD_ERASE_SECTOR_ALL);
    self.wait_for_timeout(0, 0xFF, 3000)
  }

  /// Writes a byte to the save media.
  fn write_byte(&self, offset: usize, byte: u8) -> Result<(), Error> {
    issue_flash_command(CMD_WRITE);
    FLASH_DATA.index(offset).write(byte);
    self.wait_for_timeout(offset, byte, self.write_timeout)
  }

  /// Writes an entire buffer to the save media.
  fn write_buffer(&self, offset: usize, buf: &[u8]) -> Result<(), Error> {
    self.set_bank(offset >> BANK_SHIFT)?;
    for i in 0..buf.len() {
      let byte_off = offset + i;
      if (byte_off & BANK_MASK) == 0 {
        self.set_bank(byte_off >> BANK_SHIFT)?;
      }
      self.write_byte(byte_off & BANK_MASK, buf[i])?;
    }
    Ok(())
  }

  /// Erases and writes an entire 128b sector on Atmel devices.
  fn write_atmel_sector_raw(&self, offset: usize, buf: &[u8]) -> Result<(), Error> {
    with_irqs_disabled(|| {
      issue_flash_command(CMD_WRITE);
      for i in 0..128 {
        FLASH_DATA.index(offset + i).write(buf[i]);
      }
      self.wait_for_timeout(offset + 127, buf[127], self.erase_sector_timeout)
    })?;
    Ok(())
  }

  /// Writes an entire 128b sector on Atmel devices, copying existing data in
  /// case of non-sector aligned writes.
  #[inline(never)] // avoid allocating the 128 byte buffer for no reason.
  fn write_atmel_sector_safe(&self, offset: usize, buf: &[u8], start: usize) -> Result<(), Error> {
    let mut sector = [0u8; 128];
    self.read_buffer(offset, &mut sector[0..start])?;
    sector[start..start + buf.len()].copy_from_slice(buf);
    self.read_buffer(offset + start + buf.len(), &mut sector[start + buf.len()..128])?;
    self.write_atmel_sector_raw(offset, &sector)
  }

  /// Writes an entire 128b sector on Atmel devices, copying existing data in
  /// case of non-sector aligned writes.
  ///
  /// This avoids allocating stack if there is no need to.
  fn write_atmel_sector(&self, offset: usize, buf: &[u8], start: usize) -> Result<(), Error> {
    if start == 0 && buf.len() == 128 {
      self.write_atmel_sector_raw(offset, buf)
    } else {
      self.write_atmel_sector_safe(offset, buf, start)
    }
  }
}

/// The [`RawSaveAccess`] used for flash save media.
pub struct FlashAccess;
impl RawSaveAccess for FlashAccess {
  fn info(&self) -> Result<&'static MediaInfo, Error> {
    Ok(cached_chip_info()?.info)
  }

  fn read(&self, offset: usize, buf: &mut [u8]) -> Result<(), Error> {
    let chip = cached_chip_info()?;
    chip.check_len(offset, buf.len())?;

    let _lock = lock_media()?;
    chip.read_buffer(offset, buf)
  }

  fn verify(&self, offset: usize, buf: &[u8]) -> Result<bool, Error> {
    let chip = cached_chip_info()?;
    chip.check_len(offset, buf.len())?;

    let _lock = lock_media()?;
    chip.verify_buffer(offset, buf)
  }

  fn prepare_write(&self, sector: usize, count: usize) -> Result<(), Error> {
    let chip = cached_chip_info()?;
    chip.check_sector_len(sector, count)?;

    let _lock = lock_media()?;
    if chip.uses_atmel_api {
      Ok(())
    } else if count == chip.info.sector_count {
      chip.erase_chip()
    } else {
      for i in sector..sector + count {
        chip.erase_sector(i)?;
      }
      Ok(())
    }
  }

  fn write(&self, mut offset: usize, mut buf: &[u8]) -> Result<(), Error> {
    let chip = cached_chip_info()?;
    chip.check_len(offset, buf.len())?;

    let _lock = lock_media()?;
    if chip.uses_atmel_api {
      while buf.len() != 0 {
        let start = offset & 127;
        let end_len = cmp::min(128 - start, buf.len());
        chip.write_atmel_sector(offset & !127, &buf[..end_len], start)?;
        buf = &buf[end_len..];
        offset += end_len;
      }
      Ok(())
    } else {
      // Write the bytes one by one.
      chip.write_buffer(offset, buf)?;
      Ok(())
    }
  }
}
