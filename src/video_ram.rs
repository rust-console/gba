use super::*;

/// The width of the GBA screen.
pub const SCREEN_WIDTH: isize = 240;

/// The height of the GBA screen.
pub const SCREEN_HEIGHT: isize = 160;

/// The start of VRAM.
///
/// Depending on what display mode is currently set there's different ways that
/// your program should interpret the VRAM space. Accordingly, we give the raw
/// value as just being a usize.
pub const VRAM_BASE_ADDRESS: usize = 0x0600_0000;

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
pub unsafe fn mode3_plot(col: isize, row: isize, color: u16) {
  core::ptr::write_volatile((VRAM_BASE_ADDRESS as *mut u16).offset(col + row * SCREEN_WIDTH), color);
}
