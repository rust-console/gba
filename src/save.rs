//! Module for reading and writing to save media.
//!
//! This module provides both specific interfaces that directly access particular
//! types of save media, and an abstraction layer that allows access to all kinds
//! of save media using a shared interface.
//!
//! ## Save media types
//!
//! There are, broadly speaking, three different kinds of save media that can be
//! found in official Game Carts:
//!
//! * Battery-Backed SRAM: The simplest kind of save media, which can be accessed
//!   like normal memory. You can have SRAM up to 32KiB, and while there exist a
//!   few variants this does not matter much for a game developer.
//! * EEPROM: A kind of save media based on very cheap chips and slow chips.
//!   These are accessed using a serial interface based on reading/writing bit
//!   streams into IO registers. This memory comes in 8KiB and 512 byte versions,
//!   which unfortunately cannot be distinguished at runtime.
//! * Flash: A kind of save media based on flash memory. Flash memory can be read
//!   like ordinary memory, but writing requires sending commands using multiple
//!   IO register spread across the address space. This memory comes in 64KiB
//!   and 128KiB variants, which can thankfully be distinguished using a chip ID.
//!
//! As these various types of save media cannot be easily distinguished at
//! runtime, the kind of media in use should be set manually.
//!
//! ## Setting save media type
//!
//! To use save media in your game, you must set which type to use. This is done
//! by calling one of the following functions at startup:
//!
//! * For 32 KiB battery-backed SRAM, call [`use_battery_backed_sram`].
//! * For 64 KiB flash memory, call [`use_flash_64k`].
//! * For 128 KiB flash memory, call [`use_flash_128k`].
//! * For 512 byte EEPROM, call [`use_eeprom_512b`].
//! * For 8 KiB EEPROM, call [`use_eeprom_8k`].
//!
//! Then, call [`set_timer_for_timeout`] to set the timer you intend to use to
//! track the timeout that prevents errors with the save media from hanging your
//! game. For more information on GBA timers, see the
//! [`timer`](`crate::io::timer`) module's documentation.
//!
//! ```rust
//! # use gba::save;
//! save::use_flash_128k();
//! save::set_timer_for_timeout(3); // Uses timer 3 for save media timeouts.
//! ```
//!
//! ## Using save media
//!
//!

use core::ops::Range;
use crate::sync::Static;

mod setup;
mod asm_utils;
mod utils;

pub use setup::*;
pub use asm_utils::*;
pub use utils::*;

pub mod sram;
pub mod eeprom;
pub mod flash;

/// A list of save media types.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum MediaType {
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
    /// A user-defined save media type
    Custom,
}

/// The type used for errors encountered while reading or writing save media.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    /// There is no save media attached to this game cart.
    NoMedia,
    /// Failed to write the data to save media.
    WriteError,
    /// An operation on SRAM timed out.
    OperationTimedOut,
    /// An attempt was made to access save media at an invalid offset.
    OutOfBounds,
    /// The media is already in use.
    ///
    /// This can generally only happen in an IRQ that happens during an ongoing
    /// save media operation.
    MediaInUse,
    /// This command cannot be used with the save media in use.
    IncompatibleCommand,
}

/// Information about the save media used.
#[derive(Clone, Debug)]
pub struct MediaInfo {
    /// The type of save media installed.
    pub media_type: MediaType,
    /// The power-of-two size of each sector. Zero represents a sector size of
    /// 0, implying sectors are not in use.
    ///
    /// (For example, 512 byte sectors would return 9 here.)
    pub sector_shift: usize,
    /// The size of the save media, in sectors.
    pub sector_count: usize,
}

/// A trait allowing low-level saving and writing to save media.
///
/// It exposes an interface mostly based around the requirements of reading and
/// writing Flash memory, as those are the most restrictive.
///
/// This interface treats memory as a continuous block of bytes for purposes of
/// reading, and as an array of sectors .
pub trait RawSaveAccess: Sync {
    /// Returns information about the save media used.
    fn info(&self) -> Result<&'static MediaInfo, Error>;

    /// Reads a slice of memory from save media.
    ///
    /// This will attempt to fill `buffer` entirely, and will error if this is
    /// not possible. The contents of `buffer` are unpredictable if an error is
    /// returned.
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error>;

    /// Verifies that the save media has been successfully written, comparing
    /// it against the given buffer.
    fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error>;

    /// Prepares a given span of sectors for writing. This may permanently erase
    /// the current contents of the sector on some save media.
    fn prepare_write(&self, sector: usize, count: usize) -> Result<(), Error>;

    /// Writes a buffer to the save media.
    ///
    /// The sectors you are writing to must be prepared with a call to the
    /// `prepare_write` function beforehand, or else the contents of the save
    /// media may be unpredictable after writing.
    fn write(&self, offset: usize, buffer: &[u8]) -> Result<(), Error>;
}

/// Contains the current save media implementation.
static CURRENT_SAVE_ACCESS: Static<Option<&'static dyn RawSaveAccess>> = Static::new(None);

/// Sets the save media implementation in use.
pub fn set_save_implementation(access: Option<&'static dyn RawSaveAccess>) {
    CURRENT_SAVE_ACCESS.write(access)
}

/// Gets the save media implementation in use.
pub fn get_save_implementation() -> Option<&'static dyn RawSaveAccess> {
    CURRENT_SAVE_ACCESS.read()
}

/// A wrapper around [`RawSaveAccess`] allowing high-level saving and writing to
/// save media.
#[derive(Copy, Clone)]
pub struct SaveAccess {
    access: &'static dyn RawSaveAccess,
    info: &'static MediaInfo,
}
impl SaveAccess {
    /// Creates a new save accessor around the current save implementaiton.
    pub fn new() -> Result<SaveAccess, Error> {
        match get_save_implementation() {
            Some(access) => Ok(SaveAccess {
                access,
                info: access.info()?,
            }),
            None => Err(Error::NoMedia),
        }
    }

    /// Returns the media info underlying this accessor.
    pub fn media_info(&self) -> &'static MediaInfo {
        self.info
    }

    /// Returns the save media type being used.
    pub fn media_type(&self) -> MediaType {
        self.info.media_type
    }

    /// Returns the sector size of the save media. It is generally optimal to
    /// write data in blocks that are aligned to the sector size.
    pub fn sector_size(&self) -> usize {
        1 << self.info.sector_shift
    }

    /// Returns the total length of this save media.
    pub fn len(&self) -> usize {
        self.info.sector_count << self.info.sector_shift
    }

    /// Copies data from the save media to a buffer.
    pub fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error> {
        self.access.read(offset, buffer)
    }

    /// Verifies that a given block of memory matches the save media.
    pub fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error> {
        self.access.verify(offset, buffer)
    }

    /// Returns a range that contains all sectors the input range overlaps.
    ///
    /// This can be used to calculate which blocks would be erased by a call
    /// to [`prepare_write`](`SaveAccess::prepare_write`)
    pub fn align_range(&self, range: Range<usize>) -> Range<usize> {
        let shift = self.info.sector_shift;
        let mask = (1 << shift) - 1;
        (range.start & !mask) .. ((range.end + mask) & !mask)
    }

    /// Prepares a given span of offsets for writing.
    ///
    /// This will erase any data in any sector overlapping the input range. To
    /// calculate which offset ranges would be affected, use the
    /// [`align_range`](`SaveAccess::align_range`) function.
    pub fn prepare_write(&self, range: Range<usize>) -> Result<(), Error> {
        let range = self.align_range(range);
        let shift = self.info.sector_shift;
        self.access.prepare_write(range.start >> shift, range.len() >> shift)
    }

    /// Writes a given buffer into the save media.
    ///
    /// You must call [`prepare_write`] on the range you intend to write for this
    /// to function correctly.
    pub fn write(&self, offset: usize, buffer: &[u8]) -> Result<(), Error> {
        self.access.write(offset, buffer)
    }

    /// Writes and validates a given buffer into the save media.
    ///
    /// You must call [`prepare_write`] on the range you intend to write for this
    /// to function correctly.
    ///
    /// This function will verify that the write has completed successfully, and
    /// return an error if it has not done so.
    pub fn write_and_verify(&self, offset: usize, buffer: &[u8]) -> Result<(), Error> {
        self.write(offset, buffer)?;
        if !self.verify(offset, buffer)? {
            Err(Error::WriteError)
        } else {
            Ok(())
        }
    }
}
