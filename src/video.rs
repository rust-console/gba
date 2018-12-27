//! Module for all things relating to the Video RAM.
//!
//! Note that the GBA has six different display modes available, and the
//! _meaning_ of Video RAM depends on which display mode is active. In all
//! cases, Video RAM is **96kb** from `0x0600_0000` to `0x0601_7FFF`.
//!
//! # Safety
//!
//! Note that all possible bit patterns are technically allowed within Video
//! Memory. If you write the "wrong" thing into video memory you don't crash the
//! GBA, instead you just get graphical glitches (or perhaps nothing at all).
//! Accordingly, the "safe" functions here will check that you're in bounds, but
//! they won't bother to check that you've set the video mode they're designed
//! for.

pub use super::*;

/// The start of VRAM.
///
/// Depending on what display mode is currently set there's different ways that
/// your program should interpret the VRAM space. Accordingly, we give the raw
/// value as just being a `usize`. Specific video mode types then wrap this as
/// being the correct thing.
pub const VRAM_BASE_USIZE: usize = 0x600_0000;

/// Mode 3 is a bitmap mode with full color and full resolution.
///
/// * **Width:** 240
/// * **Height:** 160
///
/// Because the memory requirements are so large, there's only a single page
/// available instead of two pages like the other video modes have.
///
/// As with all bitmap modes, the bitmap itself utilizes BG2 for display, so you
/// must have that BG enabled in addition to being within Mode 3.
pub struct Mode3;
impl Mode3 {
  /// The physical width in pixels of the GBA screen.
  pub const SCREEN_WIDTH: usize = 240;

  /// The physical height in pixels of the GBA screen.
  pub const SCREEN_HEIGHT: usize = 160;

  /// The Mode 3 VRAM.
  ///
  /// Use `col + row * SCREEN_WIDTH` to get the address of an individual pixel,
  /// or use the helpers provided in this module.
  pub const VRAM: VolAddressBlock<Color> =
    unsafe { VolAddressBlock::new_unchecked(VolAddress::new_unchecked(VRAM_BASE_USIZE), Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT) };

  const MODE3_U32_COUNT: u16 = (Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT / 2) as u16;

  /// private iterator over the pixels, two at a time
  const BULK_ITER: VolAddressIter<u32> =
    unsafe { VolAddressBlock::new_unchecked(VolAddress::new_unchecked(VRAM_BASE_USIZE), Self::MODE3_U32_COUNT as usize).iter() };

  /// Reads the pixel at the given (col,row).
  ///
  /// # Failure
  ///
  /// Gives `None` if out of bounds.
  pub fn read_pixel(col: usize, row: usize) -> Option<Color> {
    Self::VRAM.get(col + row * Self::SCREEN_WIDTH).map(VolAddress::read)
  }

  /// Writes the pixel at the given (col,row).
  ///
  /// # Failure
  ///
  /// Gives `None` if out of bounds.
  pub fn write_pixel(col: usize, row: usize, color: Color) -> Option<()> {
    Self::VRAM.get(col + row * Self::SCREEN_WIDTH).map(|va| va.write(color))
  }

  /// Clears the whole screen to the desired color.
  pub fn clear_to(color: Color) {
    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    for va in Self::BULK_ITER {
      va.write(bulk_color)
    }
  }

  /// Clears the whole screen to the desired color using DMA3.
  pub fn dma_clear_to(color: Color) {
    use crate::io::dma::DMA3;

    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    unsafe { DMA3::fill32(&bulk_color, VRAM_BASE_USIZE as *mut u32, Self::MODE3_U32_COUNT) };
  }
}
