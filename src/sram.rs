//! Module for reading and writing to SRAM.
//!
//! This module provides both an interface that directly accesses various kinds
//! of SRAM and also an interface that abstracts over the various kinds of SRAM
//! available to GBA games.
//!
//! SRAM Types
//! ==========
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
//! of SRAM in use must be set manually at either compile-time or runtime.

mod marker_strings;
mod raw_read;

pub mod battery_backed;

pub use raw_read::*;

/// The error used for
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    /// There is no SRAM media attached to this game cart.
    NoMedia,
    /// There was an error communicating with the SRAM chip in the game cart.
    ProtocolError,
    /// An attempt was made to access an offset outside the SRAM chip.
    OutOfRange,
}

/// A trait allowing reading and writing memory in SRAM.
pub trait SramAccess : Sync {
    /// Returns the length of the memory type.
    fn len(&self) -> usize;

    /// Reads a slice of memory from the SRAM chip.
    ///
    /// This will attempt to fill `buffer` entirely, and will error if this is
    /// not possible. The contents of `buffer` are unpredictable if an error is
    /// returned.
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error>;

    /// Writes a slice of memory to the SRAM chip.
    ///
    /// This will attempt to write `buffer` entirely to the SRAM chip and will
    /// error if this is not possible. The contents of SRAM are unpredictable
    /// if an error returns. If you want to avoid savegame corruption, it would
    /// be wise to keep two mirrors of the savegame.
    ///
    /// If `allow_clears` is set to `true`, data outside the range written by
    /// this function is allowed to be corrupted. This generally allows the
    /// write to proceed somewhat faster.
    ///
    /// Use [`SramType::get_sram_write_ranges`] to check the range that may be
    /// potentially corrupted. Currently this only occurs for some flash SRAM
    /// types which read/write data in blocks of 4 kilobytes.
    fn write(&self, offset: usize, buffer: &[u8], exact: bool) -> Result<(), Error>;

    /// Returns the range of offsets (end value is exclusive) that
    /// [`SramType::write_sram`] with the `allow_clears` argument set to `true`
    /// may corrupt.
    fn get_sram_write_ranges(&self, offset: usize, len: usize) -> Result<(usize, usize), Error>;
}

struct NoSram;
impl SramAccess for NoSram {
    fn len(&self) -> usize {
        0
    }
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error> {
        Err(Error::NoMedia)
    }
    fn write(&self, offset: usize, buffer: &[u8], exact: bool) -> Result<(), Error> {
        Err(Error::NoMedia)
    }
    fn get_sram_write_ranges(&self, offset: usize, len: usize) -> Result<(usize, usize), Error> {
        Err(Error::NoMedia)
    }
}

/// A constant containing a SramAccess that contains no SRAM.
pub static NO_SRAM: &'static dyn SramAccess = &NoSram;

/// A list of basic SRAM types.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum SramType {
    /// 32KiB Battery-Backed SRAM or FRAM
    BatteryBacked,
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
static mut SRAM_ACCESS: &'static dyn SramAccess = NO_SRAM;

/// Sets the SRAM accessor in use, and returns the current one.
pub fn set_accessor(access: &'static dyn SramAccess) -> &'static dyn SramAccess {
    // TODO: Use a proper setter that prevents tearing
    unsafe {
        let current = SRAM_ACCESS;
        SRAM_ACCESS = access;
        current
    }
}

/// Gets the SRAM accessor in use.
pub fn get_accessor() -> &'static dyn SramAccess {
    unsafe { SRAM_ACCESS }
}

/// Declares that the ROM uses battery backed SRAM/FRAM.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one approprate for
/// memory type.
pub fn use_battery_backed_sram() {
    marker_strings::emit_sram_marker();
    set_accessor(battery_backed::ACCESS);
}