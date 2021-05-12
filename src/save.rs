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
//! * For 32 KiB battery-backed SRAM, call [`use_sram`].
//! * For 64 KiB flash memory, call [`use_flash_64k`].
//! * For 128 KiB flash memory, call [`use_flash_128k`].
//! * For 512 byte EEPROM, call [`use_eeprom_512b`].
//! * For 8 KiB EEPROM, call [`use_eeprom_8k`].
//!
//! Then, call [`set_timer_for_timeout`] to set the timer you intend to use to
//! track the timeout that prevents errors with the save media from hanging your
//! game. For more information on GBA timers, see the
//! [`timers`](`crate::mmio_types::TimerControl`) module's documentation.
//!
//! ```rust
//! # use gba::save;
//! save::use_flash_128k();
//! save::set_timer_for_timeout(3); // Uses timer 3 for save media timeouts.
//! ```
//!
//! ## Using save media
//!
//! To access save media, use the [`SaveAccess::new`] method to create a new
//! [`SaveAccess`] object. Its methods are used to read or write save media.
//!
//! Reading data from the savegame is simple. Use [`read`](`SaveAccess::read`)
//! to copy data from an offset in the savegame into a buffer in memory.
//!
//! ```rust
//! # use gba::{info, save::SaveAccess};
//! let mut buf = [0; 1000];
//! SaveAccess::new()?.read(1000, &mut buf)?;
//! info!("Memory result: {:?}", buf);
//! ```
//!
//! Writing to save media requires you to prepare the area for writing by calling
//! the [`prepare_write`](`SaveAccess::prepare_write`) method before doing the
//! actual write commands with the [`write`](`SaveAccess::write`) method.
//!
//! ```rust
//! # use gba::{info, save::SaveAccess};
//! let access = SaveAccess::new()?;
//! access.prepare_write(500..600)?;
//! access.write(500, &[10; 25])?;
//! access.write(525, &[20; 25])?;
//! access.write(550, &[30; 25])?;
//! access.write(575, &[40; 25])?;
//! ```
//!
//! The `prepare_write` method leaves everything in a sector that overlaps the
//! range passed to it in an implementation defined state. On some devices it may
//! do nothing, and on others, it may clear the entire range to `0xFF`.
//!
//! Because writes can only be prepared on a per-sector basis, a clear on a range
//! of `4000..5000` on a device with 4096 byte sectors will actually clear a range
//! of `0..8192`. Use [`sector_size`](`SaveAccess::sector_size`) to find the
//! sector size, or [`align_range`](`SaveAccess::align_range`) to directly
//! calculate the range of memory that will be affected by the clear.
//!
//! ## Performance and Other Details
//!
//! Because `prepare_write` does nothing on non-flash chips, it would not cause
//! correctness issues to ignore it. Even so, it is recommend to write code to
//! use the `prepare_write` function regardless of the save media, as it has
//! minimal runtime cost on other save media types. If needed, you can check if
//! `prepare_write` is required by calling the
//! (`requires_prepare_write`)(`SaveAccess::requires_prepare_write`) method.
//!
//! Some memory types have a `sector_size` above `1`, but do not use
//! `prepare_write`. This indicates that the media type has sectors that must
//! be rewritten all at once, instead of supporting the separate erase/write
//! cycles that flash media does. Writing non-sector aligned memory will be
//! slower on such save media, as the implementation needs to read the old
//! contents into a buffer before writing to avoid data loss.
//!
//! To summarize, for all supported media types:
//!
//! * SRAM does not require `prepare_write` and has no sectors to align to. Reads
//!   and writes at any alignment are efficient. Furthermore, it does not require
//!   a timer to be set with [`set_timer_for_timeout`].
//! * Non-Atmel flash chips requires `prepare_write`, and have sectors of 4096
//!   bytes. Atmel flash chips instead do not require `prepare_write`, and instead
//!   have sectors of 128 bytes. You should generally try to use `prepare_write`
//!   regardless, and write in blocks of 128 bytes if at all possible.
//! * EEPROM does not require `prepare_write` and has sectors of 8 bytes.

use crate::sync::Static;
use core::ops::Range;

mod asm_utils;
mod setup;
mod utils;

pub use asm_utils::*;
pub use setup::*;
pub use utils::*;

pub mod eeprom;
pub mod flash;
pub mod sram;

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
  /// An operation on save media timed out.
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
  /// Whether the save media type requires the use of the
  /// [`prepare_write`](`SaveAccess::prepare_write`) function before a block of
  /// memory can be overwritten.
  pub requires_prepare_write: bool,
}

/// A trait allowing low-level saving and writing to save media.
///
/// It exposes an interface mostly based around the requirements of reading and
/// writing flash memory, as those are the most restrictive.
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

/// Allows reading and writing of save media.
#[derive(Copy, Clone)]
pub struct SaveAccess {
  access: &'static dyn RawSaveAccess,
  info: &'static MediaInfo,
}
impl SaveAccess {
  /// Creates a new save accessor around the current save implementaiton.
  pub fn new() -> Result<SaveAccess, Error> {
    match get_save_implementation() {
      Some(access) => Ok(SaveAccess { access, info: access.info()? }),
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

  /// Returns whether this save media requires the use of [`SaveAccess::prepare_write`].
  pub fn requires_prepare_write(&self) -> bool {
    self.info.requires_prepare_write
  }

  /// Returns a range that contains all sectors the input range overlaps.
  ///
  /// This can be used to calculate which blocks would be erased by a call
  /// to [`prepare_write`](`SaveAccess::prepare_write`)
  pub fn align_range(&self, range: Range<usize>) -> Range<usize> {
    let shift = self.info.sector_shift;
    let mask = (1 << shift) - 1;
    (range.start & !mask)..((range.end + mask) & !mask)
  }

  /// Prepares a given span of offsets for writing.
  ///
  /// This will erase any data in any sector overlapping the input range. To
  /// calculate which offset ranges would be affected, use the
  /// [`align_range`](`SaveAccess::align_range`) function.
  pub fn prepare_write(&self, range: Range<usize>) -> Result<(), Error> {
    if self.info.requires_prepare_write {
      let range = self.align_range(range);
      let shift = self.info.sector_shift;
      self.access.prepare_write(range.start >> shift, range.len() >> shift)
    } else {
      Ok(())
    }
  }

  /// Writes a given buffer into the save media.
  ///
  /// If [`requires_prepare_write`](`SaveAccess::requires_prepare_write`) returns
  /// `true`, you must call [`prepare_write`](`SaveAccess::prepare_write`) on the
  /// range you intend to write for this to function correctly. The contents of
  /// the save media are unpredictable if you do not.
  pub fn write(&self, offset: usize, buffer: &[u8]) -> Result<(), Error> {
    self.access.write(offset, buffer)
  }

  /// Writes and validates a given buffer into the save media.
  ///
  /// If [`requires_prepare_write`](`SaveAccess::requires_prepare_write`) returns
  /// `true`, you must call [`prepare_write`](`SaveAccess::prepare_write`) on the
  /// range you intend to write for this to function correctly. The contents of
  /// the save media will be unpredictable if you do not.
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
