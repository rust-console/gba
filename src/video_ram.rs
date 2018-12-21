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

// TODO: kill all this too

/// The physical width in pixels of the GBA screen.
pub const SCREEN_WIDTH: isize = 240;

/// The physical height in pixels of the GBA screen.
pub const SCREEN_HEIGHT: isize = 160;

/// The start of VRAM.
///
/// Depending on what display mode is currently set there's different ways that
/// your program should interpret the VRAM space. Accordingly, we give the raw
/// value as just being a `usize`.
pub const VRAM_BASE_ADDRESS: usize = 0x0600_0000;

const MODE3_VRAM: VolAddress<u16> = unsafe { VolAddress::new_unchecked(VRAM_BASE_ADDRESS) };

/// Draws a pixel to the screen while in Display Mode 3, with bounds checks.
///
/// # Panics
///
/// If `col` or `row` are out of bounds this will panic.
pub fn mode3_draw_pixel(col: isize, row: isize, color: u16) {
  assert!(col >= 0 && col < SCREEN_WIDTH);
  assert!(row >= 0 && row < SCREEN_HEIGHT);
  unsafe { mode3_draw_pixel_unchecked(col, row, color) }
}

/// Draws a pixel to the screen while in Display Mode 3.
///
/// Coordinates are relative to the top left corner.
///
/// If you're in another mode you'll get something weird drawn, but it's memory
/// safe in the rust sense as long as you stay in bounds.
///
/// # Safety
///
/// * `col` must be in `0..SCREEN_WIDTH`
/// * `row` must be in `0..SCREEN_HEIGHT`
pub unsafe fn mode3_draw_pixel_unchecked(col: isize, row: isize, color: u16) {
  MODE3_VRAM.offset(col + row * SCREEN_WIDTH).write(color);
}

/// Reads the given pixel of video memory according to Mode 3 placement.
///
/// # Failure
///
/// If the location is out of bounds you get `None`.
pub fn mode3_read_pixel(col: isize, row: isize) -> Option<u16> {
  if col >= 0 && col < SCREEN_WIDTH && row >= 0 && row < SCREEN_HEIGHT {
    unsafe { Some(MODE3_VRAM.offset(col + row * SCREEN_WIDTH).read()) }
  } else {
    None
  }
}

/// Clears the entire screen to the color specified.
pub unsafe fn mode3_clear_screen(color: u16) {
  // TODO: use DMA?
  let color = color as u32;
  let bulk_color = color << 16 | color;
  let block: VolAddressBlock<u32> = VolAddressBlock::new_unchecked(MODE3_VRAM.cast::<u32>(), (SCREEN_HEIGHT * SCREEN_WIDTH / 2) as usize);
  for b in block.iter() {
    b.write(bulk_color);
  }
}
