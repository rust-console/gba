//! Module for reading and writing to SRAM.
//!
//! This module provides both an interface that directly accesses various kinds
//! of SRAM and also an interface that abstracts over the various kinds of SRAM
//! available to GBA games.
//!
//! ## SRAM Types
//!
//! There are, broadly speaking, three different kinds of SRAM that can be found
//! in official Game Carts:
//!
//! * Battery-Backed: The simplest kind of memory, which acts as ordinary
//!   memory. You can have SRAM up to 32KiB, and while there exist a few
//!   variants this does not matter much for a game developer.
//! * EEPROM: A kind of SRAM based on very cheap chips and slow chips, which use
//!   a serial interface based on reading/write bit streams into IO registers.
//!   This memory comes in 8KiB and 512 byte versions, which unfortunately cannot
//!   be distinguished at runtime.
//! * Flash: A kind of memory based on flash memory. This memory can be read
//!   like ordinary memory, but writing requires sending commands using multiple
//!   IO register spread across the address space. This memory comes in 64KiB
//!   and 128KiB variants, which can thankfully be distinguished using a chip ID.
//!
//! As these various memory types cannot be distinguished at runtime, the kind
//! of SRAM in use must be set manually.
//!
//! ## Setting up SRAM
//!
//! To use SRAM, you must call one of the approprate functions to set up Game
//! Pak to use the given memory type. The available memory types are:
//!
//! * For 32 KiB battery-backed SRAM, call [`use_battery_backed_sram`].
//! * For 64 KiB flash memory, call [`use_flash_64k`].
//! * For 128 KiB flash memory, call [`use_flash_128k`].
//! * For 512 byte EEPROM, call [`use_eeprom_512b`].
//! * For 8 KiB EEPROM, call [`use_eeprom_8k`].
//!
//! Then, use [`set_timer_id`] to set the timer you intend to use to track the
//! timeout that prevents errors with SRAM media from hanging your game.
//!
//! ```rust
//! # use gba::sram;
//! sram::use_flash_128k();
//! sram::set_timer_for_timeout(3); // Uses timer 3 for SRAM timeouts.
//! ```

use crate::io::timers::*;
use crate::sync::{Static, RawMutex, RawMutexGuard};
use voladdress::VolAddress;

mod marker_strings;
mod raw_read;

pub mod battery_backed;
pub mod eeprom;
pub mod flash;

pub use raw_read::*;

/// The error used for
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    /// There is no SRAM media attached to this game cart.
    NoMedia,
    /// There was an error communicating with the SRAM chip in the game cart.
    ProtocolError,
    /// Failed to write the data to SRAM.
    WriteError,
    /// An attempt was made to access an offset outside the SRAM chip.
    OutOfRange,
    /// The media is already in use.
    ///
    /// This can generally only happen in an IRQ that happens during an ongoing
    /// SRAM operation.
    MediaInUse,
}

/// A trait allowing reading and writing memory in SRAM.
pub trait SramAccess : Sync {
    /// Returns the type of memory in use.
    fn sram_type(&self) -> Result<SramType, Error>;

    /// Returns the length of the memory type.
    fn len(&self) -> Result<usize, Error>;

    /// Reads a slice of memory from the SRAM chip.
    ///
    /// This will attempt to fill `buffer` entirely, and will error if this is
    /// not possible. The contents of `buffer` are unpredictable if an error is
    /// returned.
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error>;

    /// Verifies that SRAM has been successfully written, comparing it against
    /// the given buffer.
    fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error>;

    /// Writes a slice of memory to the SRAM chip.
    ///
    /// This will attempt to write `buffer` entirely to the SRAM chip and will
    /// error if this is not possible. The contents of SRAM are unpredictable
    /// if an error returns. If you want to avoid savegame corruption, it would
    /// be wise to keep two mirrors of the savegame.
    ///
    /// This will validate that the buffer was written correctly, and attempt
    /// to retry 3 times before returning an error.
    fn write(&self, offset: usize, buffer: &[u8]) -> Result<(), Error> {
        self.write_validating(offset, buffer, true)
    }

    /// Writes an aligned slice of memory to the SRAM chip.
    ///
    /// This will attempt to write `buffer` entirely to the SRAM chip and will
    /// error if this is not possible. The contents of SRAM are unpredictable
    /// if an error returns. If you want to avoid savegame corruption, it would
    /// be wise to keep two mirrors of the savegame.
    ///
    /// This is designed to write entire sectors of the SRAM at once. Any data
    /// that is in a sector written by this command, but outside the written
    /// range will be corrupted.
    ///
    /// This will validate that the buffer was written correctly, and attempt
    /// to retry 3 times before returning an error.
    fn write_aligned(&self, offset: usize, buffer: &[u8]) -> Result<(), Error> {
        self.write_validating(offset, buffer, false)
    }

    /// Writes a slice of memory to the SRAM chip.
    ///
    /// This will attempt to write `buffer` entirely to the SRAM chip and will
    /// error if this is not possible. The contents of SRAM are unpredictable
    /// if an error returns. If you want to avoid savegame corruption, it would
    /// be wise to keep two mirrors of the savegame.
    ///
    /// If `exact` is set to `false`, data falling in the same sector as any
    /// data written to SRAM will be corrupted
    ///
    /// Use [`SramType::get_sram_write_ranges`] to check the range that may be
    /// potentially corrupted. Currently this only occurs for some flash SRAM
    /// types which read/write data in blocks of 4 kilobytes.
    ///
    /// This will validate that the buffer was written correctly, and attempt
    /// to retry 3 times before returning an error.
    fn write_validating(
        &self, offset: usize, buffer: &[u8], exact: bool
    ) -> Result<(), Error> {
        for _ in 0..3 {
            self.write_raw(offset, buffer, exact)?;
            if self.verify(offset, buffer)? {
                return Ok(())
            }
        }
        Err(Error::WriteError)
    }

    /// Writes a slice of memory to the SRAM chip.
    ///
    /// This will attempt to write `buffer` entirely to the SRAM chip and will
    /// error if this is not possible. The contents of SRAM are unpredictable
    /// if an error returns. If you want to avoid savegame corruption, it would
    /// be wise to keep two mirrors of the savegame.
    ///
    /// If `exact` is set to `false`, data outside the range written by this
    /// function is allowed to be corrupted. This generally allows the write
    /// to proceed somewhat faster.
    ///
    /// Use [`SramType::get_sram_write_ranges`] to check the range that may be
    /// potentially corrupted. Currently this only occurs for some flash SRAM
    /// types which read/write data in blocks of 4 kilobytes.
    fn write_raw(
        &self, offset: usize, buffer: &[u8], exact: bool,
    ) -> Result<(), Error>;

    /// Returns the shift required such that `1 << shift` equals the sector
    /// size.
    fn sector_shift(&self) -> Result<usize, Error>;

    /// Returns the size of this media's scetors.
    fn sector_size(&self) -> Result<usize, Error> {
        Ok(1 << self.sector_shift()?)
    }
}

struct NoSram;
impl SramAccess for NoSram {
    fn sram_type(&self) -> Result<SramType, Error> {
        Ok(SramType::None)
    }
    fn len(&self) -> Result<usize, Error> {
        Ok(0)
    }
    fn read(&self, _offset: usize, _buffer: &mut [u8]) -> Result<(), Error> {
        Err(Error::NoMedia)
    }
    fn verify(&self, _offset: usize, _buffer: &[u8]) -> Result<bool, Error> {
        Err(Error::NoMedia)
    }
    fn write_raw(&self, _offset: usize, _buffer: &[u8], _exact: bool) -> Result<(), Error> {
        Err(Error::NoMedia)
    }
    fn sector_shift(&self) -> Result<usize, Error> {
        Err(Error::NoMedia)
    }
}

/// A constant containing a SramAccess that accesses no SRAM.
pub static NO_SRAM: &'static dyn SramAccess = &NoSram;

/// A list of basic SRAM types.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum SramType {
    /// No backup media.
    None,
    /// 32KiB Battery-Backed SRAM or FRAM
    Sram32K,
    /// 8KiB EEPROM
    Eeprom8K,
    /// 512B EEPROM
    Eeprom512B,
    /// 64KiB flash chip
    Flash64K,
    /// 128KiB flash chip
    Flash128K,
    /// A custom SRAM type defined by the user
    Custom(&'static str),
}

/// Contains the current SRAM accessor.
static SRAM_ACCESS: Static<&'static dyn SramAccess> = Static::new(NO_SRAM);

/// Sets the SRAM accessor in use, and returns the current one.
pub fn set_accessor(access: &'static dyn SramAccess) -> &'static dyn SramAccess {
    SRAM_ACCESS.replace(access)
}

/// Gets the SRAM accessor in use.
pub fn get_accessor() -> &'static dyn SramAccess {
    SRAM_ACCESS.read()
}

/// Declares that the ROM uses battery backed SRAM/FRAM.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// Battery Backed SRAM is generally very fast, but limited in size compared
/// to flash chips.
pub fn use_battery_backed_sram() {
    marker_strings::emit_sram_marker();
    set_accessor(&battery_backed::BatteryBackedAccess);
}

/// Declares that the ROM uses 64KiB flash memory.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// Flash SRAM is generally very slow to write to and relatively fast to read
/// from. It is the only real option if you need larger save data.
pub fn use_flash_64k() {
    marker_strings::emit_flash_512k_marker();
    set_accessor(flash::ACCESS);
}

/// Declares that the ROM uses 128KiB flash memory.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// Flash SRAM is generally very slow to write to and relatively fast to read
/// from. It is the only real option if you need larger save data.
pub fn use_flash_128k() {
    marker_strings::emit_flash_1m_marker();
    set_accessor(flash::ACCESS);
}

/// Declares that the ROM uses 512 bytes EEPROM memory.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// EEPROM is generally pretty slow and also very small. It's mainly used in
/// Game Paks because it's cheap.
pub fn use_eeprom_512b() {
    marker_strings::emit_eeprom_marker();
    set_accessor(eeprom::ACCESS_512B);
}

/// Declares that the ROM uses 8 KiB EEPROM memory.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// EEPROM is generally pretty slow and also very small. It's mainly used in
/// Game Paks because it's cheap.
pub fn use_eeprom_8k() {
    marker_strings::emit_eeprom_marker();
    set_accessor(eeprom::ACCESS_8K);
}

/// Internal representation for our active timer.
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum TimerId {
    None,
    T0,
    T1,
    T2,
    T3,
}

/// Stores the timer ID used for SRAM timeouts.
static TIMER_ID: Static<TimerId> = Static::new(TimerId::None);

/// Sets the timer to use to implement timeouts for operations that may hang.
///
/// This timer may be used by any SRAM operation.
pub fn set_timer_for_timeout(id: u8) {
    if id >= 4 {
        panic!("Timer ID must be 0-3.");
    } else {
        TIMER_ID.write([TimerId::T0, TimerId::T1, TimerId::T2, TimerId::T3][id as usize])
    }
}

/// Disables the timeout for operations that may hang.
pub fn disable_timeout() {
    TIMER_ID.write(TimerId::None);
}

/// A timeout type used to prevent errors with SRAM from hanging the game.
pub struct Timeout {
    _lock_guard: RawMutexGuard<'static>,
    active: bool,
    timer_l: VolAddress<u16>,
    timer_h: VolAddress<TimerControlSetting>,
}
impl Timeout {
    /// Creates a new timeout from the timer passed to [`set_timer_id`].
    ///
    /// ## Errors
    ///
    /// If another timeout has already been created.
    #[inline(never)]
    pub fn new() -> Result<Self, Error> {
        static TIMEOUT_LOCK: RawMutex = RawMutex::new();
        let _lock_guard = match TIMEOUT_LOCK.try_lock() {
            Some(x) => x,
            None => return Err(Error::MediaInUse),
        };
        let id = TIMER_ID.read();
        Ok(Timeout {
            _lock_guard,
            active: id != TimerId::None,
            timer_l: match id {
                TimerId::None => unsafe { VolAddress::new(0) },
                TimerId::T0 => TM0CNT_L,
                TimerId::T1 => TM1CNT_L,
                TimerId::T2 => TM2CNT_L,
                TimerId::T3 => TM3CNT_L,
            },
            timer_h: match id {
                TimerId::None => unsafe { VolAddress::new(0) },
                TimerId::T0 => TM0CNT_H,
                TimerId::T1 => TM1CNT_H,
                TimerId::T2 => TM2CNT_H,
                TimerId::T3 => TM3CNT_H,
            },
        })
    }

    /// Starts this timeout.
    pub fn start(&self) {
        if self.active {
            self.timer_l.write(0);
            let timer_ctl = TimerControlSetting::new()
                .with_tick_rate(TimerTickRate::CPU1024)
                .with_enabled(true);
            self.timer_h.write(TimerControlSetting::new());
            self.timer_h.write(timer_ctl);
        }
    }

    /// Returns whether a number of milliseconds has passed since the last call
    /// to [`start`].
    pub fn is_timeout_met(&self, check_ms: u16) -> bool {
        self.active && check_ms * 17 < self.timer_l.read()
    }
}

/// Tries to obtain a lock on the global lock for SRAM operations.
///
/// This is used to prevent operations on SRAM types that have complex state
/// from interfering with each other.
fn lock_sram() -> Result<RawMutexGuard<'static>, Error> {
    static LOCK: RawMutex = unsafe { RawMutex::new() };
    match LOCK.try_lock() {
        Some(x) => Ok(x),
        None => Err(Error::MediaInUse),
    }
}