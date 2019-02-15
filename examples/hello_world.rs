#![no_std]
#![feature(start)]
#![forbid(unsafe_code)]

use gba::{
  fatal,
  io::{
    display::{DisplayControlSetting, DisplayMode, DISPCNT, VBLANK_SCANLINE, VCOUNT},
    keypad::read_key_input,
  },
  vram::bitmap::Mode3,
  Color,
};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  // This kills the emulation with a message if we're running within mGBA.
  fatal!("{}", info);
  // If we're _not_ running within mGBA then we still need to not return, so
  // loop forever doing nothing.
  loop {}
}

/// Performs a busy loop until VBlank starts.
///
/// This is very inefficient, and please keep following the lessons until we
/// cover how interrupts work!
pub fn spin_until_vblank() {
  while VCOUNT.read() < VBLANK_SCANLINE {}
}

/// Performs a busy loop until VDraw starts.
///
/// This is very inefficient, and please keep following the lessons until we
/// cover how interrupts work!
pub fn spin_until_vdraw() {
  while VCOUNT.read() >= VBLANK_SCANLINE {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  const SETTING: DisplayControlSetting =
    DisplayControlSetting::new().with_mode(DisplayMode::Mode3).with_bg2(true);
  DISPCNT.write(SETTING);

  let mut px = Mode3::WIDTH / 2;
  let mut py = Mode3::HEIGHT / 2;
  let mut color = Color::from_rgb(31, 0, 0);

  loop {
    // read our keys for this frame
    let this_frame_keys = read_key_input();

    // adjust game state and wait for vblank
    px = px.wrapping_add(2 * this_frame_keys.x_tribool() as usize);
    py = py.wrapping_add(2 * this_frame_keys.y_tribool() as usize);
    if this_frame_keys.l() {
      color = Color(color.0.rotate_left(5));
    }
    if this_frame_keys.r() {
      color = Color(color.0.rotate_right(5));
    }

    // now we wait
    spin_until_vblank();

    // draw the new game and wait until the next frame starts.
    if px >= Mode3::WIDTH || py >= Mode3::HEIGHT {
      // out of bounds, reset the screen and position.
      Mode3::dma_clear_to(Color::from_rgb(0, 0, 0));
      px = Mode3::WIDTH / 2;
      py = Mode3::HEIGHT / 2;
    } else {
      // draw the new part of the line
      Mode3::write(px, py, color);
      Mode3::write(px, py + 1, color);
      Mode3::write(px + 1, py, color);
      Mode3::write(px + 1, py + 1, color);
    }

    // now we wait again
    spin_until_vdraw();
  }
}
