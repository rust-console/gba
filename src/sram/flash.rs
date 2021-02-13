//! A module containing support for battery backed SRAM.

use core::cmp;
use core::ops::Range;
use crate::io::timers::*;
use crate::sync::{RawMutex, RawMutexGuard, Static, InitOnce};
use super::{
    Error, SramAccess, SramType, read_raw_buf, read_raw_byte, write_raw_buf, verify_raw_buf,
};
use typenum::consts::U65536;
use voladdress::{VolAddress, VolBlock};

const FLASH_PORT_BANK: VolAddress<u8> = unsafe { VolAddress::new(0x0E000000) };
const FLASH_PORT_A: VolAddress<u8> = unsafe { VolAddress::new(0x0E005555) };
const FLASH_PORT_B: VolAddress<u8> = unsafe { VolAddress::new(0x0E002AAA) };
const FLASH_DATA: VolBlock<u8, U65536> = unsafe { VolBlock::new(0x0E000000) };

const ATMEL_SECTOR_SHIFT: usize = 9; // 512 bytes
const SECTOR_SHIFT: usize = 12; // 4 KiB

const SRAM_BANK_SHIFT: usize = 16; // 64 KiB
const SRAM_BANK_MASK: usize = (1 << SRAM_BANK_SHIFT) - 1;

/// A lock to prevent operations on Flash memory from overlapping and causing
/// unwanted side effects. This is the entirety of our protection against IRQ
/// shenanigans, simplifying the code.
static LOCK: RawMutex = unsafe { RawMutex::new() };
fn lock() -> Result<RawMutexGuard<'static>, Error> {
    match LOCK.try_lock() {
        Some(x) => Ok(x),
        None => Err(Error::MediaInUse),
    }
}

/// Internal representation for our active timer.
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum Timer {
    None,
    T0,
    T1,
    T2,
    T3,
}
impl Timer {
    fn timer_l(&self) -> VolAddress<u16> {
        match *self {
            Timer::T0 => TM0CNT_L,
            Timer::T1 => TM1CNT_L,
            Timer::T2 => TM2CNT_L,
            Timer::T3 => TM3CNT_L,
            _ => unimplemented!(),
        }
    }
    fn timer_h(&self) -> VolAddress<TimerControlSetting> {
        match *self {
            Timer::T0 => TM0CNT_H,
            Timer::T1 => TM1CNT_H,
            Timer::T2 => TM2CNT_H,
            Timer::T3 => TM3CNT_H,
            _ => unimplemented!(),
        }
    }
}
static TIMER_ID: Static<Timer> = Static::new(Timer::None);

/// A simple thing to avoid excessive bank switches
static CURRENT_BANK: Static<u8> = Static::new(!0);
fn set_bank(bank: u8) -> Result<(), Error> {
    if bank == 0xFF {
        Err(Error::OutOfRange)
    } else if bank != CURRENT_BANK.read() {
        issue_raw_command(0xAA, 0x55, 0xB0);
        FLASH_PORT_BANK.write(bank as u8);
        CURRENT_BANK.write(bank);
        Ok(())
    } else {
        Ok(())
    }
}

/// Sets the timer to use to implement timeouts for operations that may hang.
///
/// SRAM operations using Flash memory will use this timer in order to check
/// for timeouts.
pub fn set_timer_id(id: u8) {
    if id >= 4 {
        panic!("Timer ID must be 0-3.");
    } else {
        TIMER_ID.write([Timer::T0, Timer::T1, Timer::T2, Timer::T3][id as usize])
    }
}

/// Disables the timeout for operations that may hang.
pub fn disable_timer() {
    TIMER_ID.write(Timer::None);
}

/// Helper function for issuing the usual commands.
fn issue_raw_command(c0: u8, c1: u8, c2: u8) {
    FLASH_PORT_A.write(c0);
    FLASH_PORT_B.write(c1);
    FLASH_PORT_A.write(c2);
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
    let _lock = lock()?;
    issue_raw_command(0xAA, 0x55, 0x90); // set chip ID read mode
    let high = unsafe { read_raw_byte(0x0E000001) };
    let low = unsafe { read_raw_byte(0x0E000000) };
    let id = (high as u16) << 8 | low as u16;
    issue_raw_command(0xAA, 0x55, 0xF0); // unset chip ID read mode
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
    /// Whether this is an Atmel chip, which uses a different API.
    is_atmel: bool,
    /// Whether this is an Macronix chip, which uses a slightly different API.
    is_macronix: bool,
}

// Chip info for the various chipsets.
static CHIP_INFO_SST_64K: ChipInfo = ChipInfo {
    read_wait: 2, // 2 cycles
    write_wait: 1, // 3 cycles
    write_timeout: 10,
    erase_sector_timeout: 40,
    erase_chip_timeout: 200,
    bank_count: 1,
    is_atmel: false,
    is_macronix: false,
};
static CHIP_INFO_MACRONIX_64K: ChipInfo = ChipInfo {
    read_wait: 1, // 3 cycles
    write_wait: 3, // 8 cycles
    write_timeout: 10,
    erase_sector_timeout: 2000,
    erase_chip_timeout: 2000,
    bank_count: 1,
    is_atmel: false,
    is_macronix: true,
};
static CHIP_INFO_PANASONIC_64K: ChipInfo = ChipInfo {
    read_wait: 2, // 2 cycles
    write_wait: 0, // 4 cycles
    write_timeout: 10,
    erase_sector_timeout: 500,
    erase_chip_timeout: 500,
    bank_count: 1,
    is_atmel: false,
    is_macronix: false,
};
static CHIP_INFO_ATMEL_64K: ChipInfo = ChipInfo {
    read_wait: 3, // 8 cycles
    write_wait: 3, // 8 cycles
    write_timeout: 40,
    erase_sector_timeout: 40,
    erase_chip_timeout: 40,
    bank_count: 1,
    is_atmel: true,
    is_macronix: false,
};
static CHIP_INFO_GENERIC_64K: ChipInfo = ChipInfo {
    read_wait: 3, // 8 cycles
    write_wait: 3, // 8 cycles
    write_timeout: 40,
    erase_sector_timeout: 2000,
    erase_chip_timeout: 2000,
    bank_count: 1,
    is_atmel: false, // we're assuming an unidentified chip does *not* use the Atmel API.
    is_macronix: true, // issuing the cancel can't hurt on the case it isn't macronix
};
static CHIP_INFO_GENERIC_128K: ChipInfo = ChipInfo {
    read_wait: 1, // 3 cycles
    write_wait: 3, // 8 cycles
    write_timeout: 10,
    erase_sector_timeout: 2000,
    erase_chip_timeout: 2000,
    bank_count: 2,
    is_atmel: false,
    is_macronix: false,
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

/// Helper iterator for chunking buffers into chunks.
struct IterOffsets {
    shift: usize,

    base_offset: usize,
    len: usize,

    current: usize,
    end: usize,
    limit: usize,
}
impl IterOffsets {
    fn new(shift: usize, offset: usize, len: usize, limit: usize) -> Result<Self, Error> {
        if offset + len > limit {
            Err(Error::OutOfRange)
        } else {
            Ok(IterOffsets {
                shift,
                base_offset: offset,
                len,
                current: offset,
                end: offset + len,
                limit,
            })
        }
    }
}
#[derive(Debug)]
struct IterResult {
    bank: usize,
    sram_offset: usize,
    buf_offset: Range<usize>,
}
impl Iterator for IterOffsets {
    type Item = IterResult;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let bank = self.current >> SRAM_BANK_SHIFT;

            let sector_len = 1 << self.shift;
            let sector_end = (self.current + sector_len) & !(sector_len - 1);
            let end_offset = cmp::min(self.end, sector_end);

            let start = self.current-self.base_offset;
            let end = end_offset-self.base_offset;
            let res = IterResult {
                bank,
                sram_offset: self.current,
                buf_offset: start..end,
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
        for bank in IterOffsets::new(SRAM_BANK_SHIFT, offset, buf.len(), self.total_len())? {
            self.set_bank(bank.bank)?;
            let offset = bank.sram_offset & SRAM_BANK_MASK;
            unsafe { read_raw_buf(&mut buf[bank.buf_offset], 0x0E000000 + offset); }
        }
        Ok(())
    }

    /// Verifies that a buffer was properly stored into SRAM.
    fn verify_buffer(&self, offset: usize, buf: &[u8]) -> Result<bool, Error> {
        for bank in IterOffsets::new(SRAM_BANK_SHIFT, offset, buf.len(), self.total_len())? {
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
        let offset = 0x0E000000 + offset;
        let timer = TIMER_ID.read();
        if timer == Timer::None {
            while unsafe { read_raw_byte(offset) } != val { }
            Ok(())
        } else {
            // set up the timer
            let target_timer = ms * 17; // 17408 cycles ~= 1.03 ms
            let timer_l = timer.timer_l();
            let timer_h = timer.timer_h();

            timer_l.write(0);
            let timer_ctl = TimerControlSetting::new()
                .with_tick_rate(TimerTickRate::CPU1024)
                .with_enabled(true);
            timer_h.write(TimerControlSetting::new());
            timer_h.write(timer_ctl);

            while unsafe { read_raw_byte(offset) != val } {
                if timer_l.read() > target_timer {
                    if self.is_macronix {
                        FLASH_PORT_A.write(0xF0);
                    }
                    return Err(Error::WriteError)
                }
            }
            Ok(())
        }
    }

    /// Erases a 4K sector on non-Atmel devices.
    fn erase_4k_sector_raw(&self, offset: usize) -> Result<(), Error> {
        issue_raw_command(0xAA, 0x55, 0x80);
        FLASH_PORT_A.write(0xAA);
        FLASH_PORT_B.write(0x55);
        FLASH_DATA.index(offset).write(0x30);
        self.wait_for_timeout(offset, 0xFF, self.erase_sector_timeout)
    }
    /// Writes a byte on non-Atmel devices.
    fn write_4k_sector_byte(&self, offset: usize, byte: u8) -> Result<(), Error> {
        issue_raw_command(0xAA, 0x55, 0xA0);
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
    /// Writes an entire 4K sector on non-Atmel devices.
    fn write_4k_sector(
        &self, offset: usize, buf: &[u8], start: usize, exact: bool,
    ) -> Result<(), Error> {
        if !exact || (start == 0 && buf.len() == 4096) {
            self.write_4k_sector_raw(offset, buf, start)
        } else {
            self.write_4k_sector_exact(offset, buf, start)
        }
    }

    /// Writes a sector.
    fn write_sector(
        &self, offset: usize, buf: &[u8], start: usize, exact: bool,
    ) -> Result<(), Error> {
        if self.is_atmel {
            unimplemented!()
        } else {
            self.write_4k_sector(offset, buf, start, exact)
        }
    }
    /// The shift value to use to break buffers up into sectors.
    fn sector_shift(&self) -> usize {
        if self.is_atmel { ATMEL_SECTOR_SHIFT } else { SECTOR_SHIFT }
    }
    /// Writes a buffer into SRAM.
    fn write_buffer(
        &self, offset: usize, buf: &[u8], exact: bool,
    ) -> Result<(), Error> {
        let mut cur_bank = !0;
        let shift = self.sector_shift();
        let mask = (1 << shift) - 1;
        for bank in IterOffsets::new(shift, offset, buf.len(), self.total_len())? {
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
        let _lock = lock()?;
        info.read_buffer(offset, buffer)
    }

    fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error> {
        let info = cached_detect()?.chip_info();
        let _lock = lock()?;
        info.verify_buffer(offset, buffer)
    }

    fn write_raw(&self, offset: usize, buffer: &[u8], exact: bool) -> Result<(), Error> {
        let info = cached_detect()?.chip_info();
        let _lock = lock()?;
        info.write_buffer(offset, buffer, exact)
    }

    fn get_sram_write_ranges(&self, offset: usize, len: usize) -> Result<(usize, usize), Error> {
        todo!()
    }
}

/// A static instance of a [`SramAccess`] appropriate for battery backed SRAM.
pub static ACCESS: &'static dyn SramAccess = &FlashAccess;