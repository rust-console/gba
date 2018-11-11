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

/// Draws a pixel to the screen while in Display Mode 3, with bounds checks.
pub fn mode3_pixel(col: isize, row: isize, color: u16) {
  assert!(col >= 0 && col < SCREEN_WIDTH);
  assert!(row >= 0 && row < SCREEN_HEIGHT);
  unsafe { mode3_pixel_unchecked(col, row, color) }
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
pub unsafe fn mode3_pixel_unchecked(col: isize, row: isize, color: u16) {
  core::ptr::write_volatile((VRAM_BASE_ADDRESS as *mut u16).offset(col + row * SCREEN_WIDTH), color);
}
