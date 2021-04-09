#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
#[allow(unused)]
fn panic(info: &core::panic::PanicInfo) -> ! {
  // This kills the emulation with a message if we're running inside an
  // emulator we support (mGBA or NO$GBA), or just crashes the game if we
  // aren't.
  //fatal!("{}", info);

  loop {
    DISPCNT.read();
  }
}

/// Performs a busy loop until VBlank starts.
///
/// This is very inefficient, and please keep following the lessons until we
/// cover how interrupts work!
pub fn spin_until_vblank() {
  while VCOUNT.read() < 160 {}
}

/// Performs a busy loop until VDraw starts.
///
/// This is very inefficient, and please keep following the lessons until we
/// cover how interrupts work!
pub fn spin_until_vdraw() {
  while VCOUNT.read() >= 160 {}
}

#[no_mangle]
pub fn main() -> ! {
  const SETTING: DisplayControl = DisplayControl::new().with_display_mode(3).with_display_bg2(true);
  DISPCNT.write(SETTING);

  let mut px = mode3::WIDTH / 2;
  let mut py = mode3::HEIGHT / 2;
  let mut color = Color::from_rgb(31, 3, 1);

  loop {
    // read our keys for this frame
    let keys: Keys = KEYINPUT.read().into();

    // adjust game state and wait for vblank
    px = px.wrapping_add((2 * keys.x_signum()) as usize);
    py = py.wrapping_add((2 * keys.y_signum()) as usize);
    if keys.l() {
      color = Color(color.0.rotate_left(5));
    }
    if keys.r() {
      color = Color(color.0.rotate_right(5));
    }

    // now we wait
    spin_until_vblank();

    // draw the new game and wait until the next frame starts.
    if (px + 1) >= mode3::WIDTH || (py + 1) >= mode3::HEIGHT {
      // out of bounds, reset the screen and position.
      mode3::dma3_clear_to(Color::from_rgb(0, 0, 0));
      px = mode3::WIDTH / 2;
      py = mode3::HEIGHT / 2;
      color = Color(color.0.rotate_left(7));
    } else {
      // draw the new part of the line
      mode3::bitmap_xy(px, py).write(color);
      mode3::bitmap_xy(px, py + 1).write(color);
      mode3::bitmap_xy(px + 1, py).write(color);
      mode3::bitmap_xy(px + 1, py + 1).write(color);
    }

    // now we wait again
    spin_until_vdraw();
  }
}
