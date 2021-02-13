//! A module containing support for battery backed SRAM.

use core::cmp;
use core::ops::Range;
use crate::sync::{Static, InitOnce, disable_irqs};
use super::{
    Error, SramAccess, SramType,
    read_raw_buf, read_raw_byte, write_raw_buf, verify_raw_buf, lock_sram,
};
use typenum::consts::U65536;
use voladdress::{VolAddress, VolBlock};

// Volatile address ports for Flash
const FLASH_PORT_BANK: VolAddress<u8> = unsafe { VolAddress::new(0x0E000000) };
const FLASH_PORT_A: VolAddress<u8> = unsafe { VolAddress::new(0x0E005555) };
const FLASH_PORT_B: VolAddress<u8> = unsafe { VolAddress::new(0x0E002AAA) };
const FLASH_DATA: VolBlock<u8, U65536> = unsafe { VolBlock::new(0x0E000000) };

// Various constants related to sector sizes
const ATMEL_SECTOR_SHIFT: usize = 7; // 128 bytes
const SECTOR_SHIFT: usize = 12; // 4 KiB

const SRAM_BANK_SHIFT: usize = 16; // 64 KiB
const SRAM_BANK_MASK: usize = (1 << SRAM_BANK_SHIFT) - 1;

// Constants relating to flash commands.
const CMD_SET_BANK: u8 = 0xB0;
const CMD_READ_CHIP_ID: u8 = 0x90;
const CMD_READ_CONTENTS: u8 = 0xF0;
const CMD_WRITE: u8 = 0xA0;
const CMD_ERASE_SECTOR_BEGIN: u8 = 0x80;
const CMD_ERASE_SECTOR_CONFIRM: u8 = 0x30;

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
        Err(Error::OutOfRange)
    } else if bank != CURRENT_BANK.read() {
        issue_flash_command(CMD_SET_BANK);
        FLASH_PORT_BANK.write(bank as u8);
        CURRENT_BANK.write(bank);
        Ok(())
    } else {
        Ok(())
    }
}

/// Identifies a particular flash chip in use by a Game Pak.
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
    /// Returns the type of the SRAM chip currently in use.
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

/// Determines the raw ID of the SRAM chip currently in use.
pub fn detect_chip_id() -> Result<u16, Error> {
    let _lock = lock_sram()?;
    issue_flash_command(CMD_READ_CHIP_ID);
    let high = unsafe { read_raw_byte(0x0E000001) };
    let low = unsafe { read_raw_byte(0x0E000000) };
    let id = (high as u16) << 8 | low as u16;
    issue_flash_command(CMD_READ_CONTENTS);
    Ok(id)
}

static CHIP_ID: InitOnce<FlashChipType> = InitOnce::new();
fn cached_detect() -> Result<FlashChipType, Error> {
    CHIP_ID.try_get(|| FlashChipType::detect()).map(Clone::clone)
}

/// Information relating to a particular Flash chip that could be found in a
/// Game Pak.
struct ChipInfo {
    /// The wait state required to read from the chip.
    read_wait: u8,
    /// The wait state required to write to the chip.
    write_wait: u8,

    /// The timeout in milliseconds for writes to this chip.
    write_timeout: u16,
    /// The timeout in mililseconds for erasing a sector in this chip.
    erase_sector_timeout: u16,
    /// The timeout in milliseconds for erasing the entire chip.
    erase_chip_timeout: u16,

    /// The number of 64KiB banks in this chip.
    bank_count: u8,
    /// Whether this is an Atmel chip, which has 128 byte sectors instead of 4K.
    uses_atmel_128_sectors: bool,
    /// Whether this is an Macronix chip, which requires an additional command
    /// to cancel the current action after a timeout.
    uses_macronix_cancel_command: bool,
}

// Chip info for the various chipsets.
static CHIP_INFO_SST_64K: ChipInfo = ChipInfo {
    read_wait: 2, // 2 cycles
    write_wait: 1, // 3 cycles
    write_timeout: 10,
    erase_sector_timeout: 40,
    erase_chip_timeout: 200,
    bank_count: 1,
    uses_atmel_128_sectors: false,
    uses_macronix_cancel_command: false,
};
static CHIP_INFO_MACRONIX_64K: ChipInfo = ChipInfo {
    read_wait: 1, // 3 cycles
    write_wait: 3, // 8 cycles
    write_timeout: 10,
    erase_sector_timeout: 2000,
    erase_chip_timeout: 2000,
    bank_count: 1,
    uses_atmel_128_sectors: false,
    uses_macronix_cancel_command: true,
};
static CHIP_INFO_PANASONIC_64K: ChipInfo = ChipInfo {
    read_wait: 2, // 2 cycles
    write_wait: 0, // 4 cycles
    write_timeout: 10,
    erase_sector_timeout: 500,
    erase_chip_timeout: 500,
    bank_count: 1,
    uses_atmel_128_sectors: false,
    uses_macronix_cancel_command: false,
};
static CHIP_INFO_ATMEL_64K: ChipInfo = ChipInfo {
    read_wait: 3, // 8 cycles
    write_wait: 3, // 8 cycles
    write_timeout: 40,
    erase_sector_timeout: 40,
    erase_chip_timeout: 40,
    bank_count: 1,
    uses_atmel_128_sectors: true,
    uses_macronix_cancel_command: false,
};
static CHIP_INFO_GENERIC_64K: ChipInfo = ChipInfo {
    read_wait: 3, // 8 cycles
    write_wait: 3, // 8 cycles
    write_timeout: 40,
    erase_sector_timeout: 2000,
    erase_chip_timeout: 2000,
    bank_count: 1,
    uses_atmel_128_sectors: false,
    uses_macronix_cancel_command: true,
};
static CHIP_INFO_GENERIC_128K: ChipInfo = ChipInfo {
    read_wait: 1, // 3 cycles
    write_wait: 3, // 8 cycles
    write_timeout: 10,
    erase_sector_timeout: 2000,
    erase_chip_timeout: 2000,
    bank_count: 2,
    uses_atmel_128_sectors: false,
    uses_macronix_cancel_command: false,
};

impl FlashChipType {
    /// Returns the internal info for this chip.
    #[inline(never)]
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

/// Helper iterator for chunking buffers into chunks.
struct ChunkArray {
    shift: usize,
    base_offset: usize,
    current: usize,
    end: usize,
}
impl ChunkArray {
    fn new(shift: usize, offset: usize, len: usize, limit: usize) -> Result<Self, Error> {
        if offset + len > limit {
            Err(Error::OutOfRange)
        } else {
            Ok(ChunkArray {
                shift,
                base_offset: offset,
                current: offset,
                end: offset + len,
            })
        }
    }
}
#[derive(Debug)]
struct ChunkResult {
    bank: usize,
    sram_offset: usize,
    buf_offset: Range<usize>,
}
impl Iterator for ChunkArray {
    type Item = ChunkResult;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let sector_len = 1 << self.shift;
            let sector_end = (self.current + sector_len) & !(sector_len - 1);
            let end_offset = cmp::min(self.end, sector_end);
            let res = ChunkResult {
                bank: self.current >> SRAM_BANK_SHIFT,
                sram_offset: self.current,
                buf_offset: self.current-self.base_offset .. end_offset-self.base_offset,
            };
            self.current = end_offset;
            Some(res)
        }
    }
}

/// Actual implementation of the ChipInfo functions.
impl ChipInfo {
    /// Returns the total length of this chip.
    fn total_len(&self) -> usize {
        (self.bank_count as usize) << SRAM_BANK_SHIFT
    }

    /// Sets the currently active bank.
    fn set_bank(&self, bank: usize) -> Result<(), Error> {
        if bank >= self.bank_count as usize {
            Err(Error::OutOfRange)
        } else if self.bank_count != 0 {
            set_bank(bank as u8)
        } else {
            Ok(())
        }
    }

    /// Reads a buffer from SRAM into memory.
    fn read_buffer(&self, offset: usize, buf: &mut [u8]) -> Result<(), Error> {
        for bank in ChunkArray::new(SRAM_BANK_SHIFT, offset, buf.len(), self.total_len())? {
            self.set_bank(bank.bank)?;
            let offset = bank.sram_offset & SRAM_BANK_MASK;
            unsafe { read_raw_buf(&mut buf[bank.buf_offset], 0x0E000000 + offset); }
        }
        Ok(())
    }

    /// Verifies that a buffer was properly stored into SRAM.
    fn verify_buffer(&self, offset: usize, buf: &[u8]) -> Result<bool, Error> {
        for bank in ChunkArray::new(SRAM_BANK_SHIFT, offset, buf.len(), self.total_len())? {
            self.set_bank(bank.bank)?;
            let offset = bank.sram_offset & SRAM_BANK_MASK;
            if !unsafe { verify_raw_buf(&buf[bank.buf_offset], 0x0E000000 + offset) } {
                return Ok(false)
            }
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
                if self.uses_macronix_cancel_command {
                    FLASH_PORT_A.write(0xF0);
                }
                return Err(Error::WriteError)
            }
        }
        Ok(())
    }

    /// Erases a 4K sector on non-Atmel devices.
    fn erase_4k_sector_raw(&self, offset: usize) -> Result<(), Error> {
        issue_flash_command(CMD_ERASE_SECTOR_BEGIN);
        start_flash_command();
        FLASH_DATA.index(offset).write(CMD_ERASE_SECTOR_CONFIRM);
        self.wait_for_timeout(offset, 0xFF, self.erase_sector_timeout)
    }
    /// Writes a byte on non-Atmel devices.
    fn write_4k_sector_byte(&self, offset: usize, byte: u8) -> Result<(), Error> {
        issue_flash_command(CMD_WRITE);
        FLASH_DATA.index(offset).write(byte);
        self.wait_for_timeout(offset, byte, self.write_timeout)
    }
    /// Writes an entire 4K sector on non-Atmel devices, potentially erasing data
    /// for non-sector aligned writes.
    fn write_4k_sector_raw(
        &self, offset: usize, buf: &[u8], start: usize,
    ) -> Result<(), Error> {
        assert_eq!(offset & 0xFFF, 0, "Invalid offset passed.");
        assert!(start + buf.len() <= 4096, "Invalid buffer length.");

        self.set_bank(offset >> SRAM_BANK_SHIFT)?;
        let offset_high = offset & !((1 << SECTOR_SHIFT) - 1) & SRAM_BANK_MASK;
        self.erase_4k_sector_raw(offset_high)?;
        for i in 0..buf.len() {
            self.write_4k_sector_byte((offset + start + i) & SRAM_BANK_MASK, buf[i])?;
        }
        Ok(())
    }
    /// Writes an entire 4K sector on non-Atmel devices, copying existing data
    /// in case of non-sector aligned writes.
    #[inline(never)] // just in case. This avoids the 4096 byte allocation from
                     // leaking into an outer function's frame when it's not used.
    fn write_4k_sector_exact(
        &self, offset: usize, buf: &[u8], start: usize,
    ) -> Result<(), Error> {
        assert_eq!(offset & 0xFFF, 0, "Invalid offset passed.");
        assert!(start + buf.len() <= 4096, "Invalid buffer length.");

        let mut sector = [0u8; 4096];
        self.read_buffer(offset, &mut sector[0..start])?;
        sector[start..start+buf.len()].copy_from_slice(buf);
        self.read_buffer(offset+start+buf.len(), &mut sector[start+buf.len()..4096])?;
        self.write_4k_sector_raw(offset, &sector, 0)
    }

    /// Writes an entire 128b sector on Atmel devices.
    fn write_128b_sector_raw(
        &self, offset: usize, buf: &[u8],
    ) -> Result<(), Error> {
        assert_eq!(offset & 0x7F, 0, "Invalid offset passed.");
        assert_eq!(buf.len(), 128, "Invalid buffer length.");

        disable_irqs(|| {
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
    #[inline(never)]
    fn write_128b_sector_exact(
        &self, offset: usize, buf: &[u8], start: usize, exact: bool,
    ) -> Result<(), Error> {
        assert_eq!(offset & 0x7F, 0, "Invalid offset passed.");
        assert!(start + buf.len() <= 128, "Invalid buffer length.");

        let mut sector = [0u8; 128];
        if exact {
            self.read_buffer(offset, &mut sector[0..start])?;
        }
        sector[start..start+buf.len()].copy_from_slice(buf);
        if exact {
            self.read_buffer(offset + start + buf.len(), &mut sector[start + buf.len()..128])?;
        }
        self.write_128b_sector_raw(offset, &sector)
    }

    /// Writes a sector.
    fn write_sector(
        &self, offset: usize, buf: &[u8], start: usize, exact: bool,
    ) -> Result<(), Error> {
        if self.uses_atmel_128_sectors {
            if !exact || (start == 0 && buf.len() == 128) {
                self.write_128b_sector_raw(offset, buf)
            } else {
                self.write_128b_sector_exact(offset, buf, start, exact)
            }
        } else {
            if !exact || (start == 0 && buf.len() == 4096) {
                self.write_4k_sector_raw(offset, buf, start)
            } else {
                self.write_4k_sector_exact(offset, buf, start)
            }
        }
    }
    /// The shift value to use to break buffers up into sectors.
    fn sector_shift(&self) -> usize {
        if self.uses_atmel_128_sectors { ATMEL_SECTOR_SHIFT } else { SECTOR_SHIFT }
    }
    /// Writes a buffer into SRAM.
    fn write_buffer(
        &self, offset: usize, buf: &[u8], exact: bool,
    ) -> Result<(), Error> {
        let mut cur_bank = !0;
        let shift = self.sector_shift();
        let mask = (1 << shift) - 1;
        for bank in ChunkArray::new(shift, offset, buf.len(), self.total_len())? {
            if cur_bank != bank.bank {
                self.set_bank(bank.bank)?;
                cur_bank = bank.bank;
            }
            self.write_sector(
                bank.sram_offset & !mask, &buf[bank.buf_offset.clone()],
                bank.sram_offset & mask, exact,
            )?;
        }
        Ok(())
    }
}

/// The [`SramAccess`] used for Flash SRAM.
pub struct FlashAccess;
impl SramAccess for FlashAccess {
    fn sram_type(&self) -> Result<SramType, Error> {
        cached_detect().and_then(|id| match id.chip_info().bank_count {
            1 => Ok(SramType::Flash64K),
            2 => Ok(SramType::Flash128K),
            _ => Err(Error::ProtocolError),
        })
    }

    fn len(&self) -> Result<usize, Error> {
        cached_detect().map(|id| id.chip_info().total_len())
    }

    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error> {
        let info = cached_detect()?.chip_info();
        let _lock = lock_sram()?;
        info.read_buffer(offset, buffer)
    }

    fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error> {
        let info = cached_detect()?.chip_info();
        let _lock = lock_sram()?;
        info.verify_buffer(offset, buffer)
    }

    fn write_raw(&self, offset: usize, buffer: &[u8], exact: bool) -> Result<(), Error> {
        let info = cached_detect()?.chip_info();
        let _lock = lock_sram()?;
        info.write_buffer(offset, buffer, exact)
    }

    fn sector_shift(&self) -> Result<usize, Error> {
        Ok(cached_detect()?.chip_info().sector_shift())
    }
}

/// A static instance of a [`SramAccess`] appropriate for battery backed SRAM.
pub static ACCESS: &'static dyn SramAccess = &FlashAccess;