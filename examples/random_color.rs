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
  // Create default RNG
  let mut rng = RNG::default();

  let mut px: usize = 0;
  let mut py: usize = 0;
  let mut color;

  loop {
    // Generate color from RNG
    color = rng.next_color();
    // now we wait
    spin_until_vblank();
    // Draw pixels to screen
    mode3::bitmap_xy(px, py).write(color);
    mode3::bitmap_xy(px, py + 1).write(color);
    mode3::bitmap_xy(px + 1, py).write(color);
    mode3::bitmap_xy(px + 1, py + 1).write(color);
    // Increment x and y, wrap as needed
    px += 2;
    if px >= mode3::WIDTH {
      px = 0;
      py += 2;
      if py >= mode3::HEIGHT {
        py = 0;
      }
    }

    // now we wait again
    spin_until_vdraw();
  }
}
