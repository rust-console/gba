#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
#[allow(unused)]
fn panic(info: &core::panic::PanicInfo) -> ! {
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

  const RED: Color = Color::from_rgb(31, 0, 0);
  const GREEN: Color = Color::from_rgb(0, 31, 0);

  let mut keys = Keys::read();

  fn draw_square(x: usize, y: usize, color: Color) {
    mode3::bitmap_xy(x, y).write(color);
    mode3::bitmap_xy(x, y + 1).write(color);
    mode3::bitmap_xy(x, y + 2).write(color);
    mode3::bitmap_xy(x + 1, y).write(color);
    mode3::bitmap_xy(x + 1, y + 1).write(color);
    mode3::bitmap_xy(x + 1, y + 2).write(color);
    mode3::bitmap_xy(x + 2, y).write(color);
    mode3::bitmap_xy(x + 2, y + 1).write(color);
    mode3::bitmap_xy(x + 2, y + 2).write(color);
  }

  loop {
    // wait until we're into the vblank period
    spin_until_vdraw();
    spin_until_vblank();

    draw_square(1, 1, if keys.l() { GREEN } else { RED });
    draw_square(mode3::WIDTH - 4, 1, if keys.r() { GREEN } else { RED });
    //
    draw_square(mode3::WIDTH - 4, mode3::HEIGHT / 2, if keys.a() { GREEN } else { RED });
    draw_square(mode3::WIDTH - 4 - 20, mode3::HEIGHT / 2 + 20, if keys.b() { GREEN } else { RED });
    //
    draw_square(mode3::WIDTH / 2 + 10, mode3::HEIGHT - 5, if keys.start() { GREEN } else { RED });
    draw_square(mode3::WIDTH / 2 - 10, mode3::HEIGHT - 5, if keys.select() { GREEN } else { RED });
    //
    draw_square(30, mode3::HEIGHT / 2 - 20, if keys.up() { GREEN } else { RED });
    draw_square(30, mode3::HEIGHT / 2 + 20, if keys.down() { GREEN } else { RED });
    draw_square(10, mode3::HEIGHT / 2, if keys.left() { GREEN } else { RED });
    draw_square(50, mode3::HEIGHT / 2, if keys.right() { GREEN } else { RED });

    // read our keys for next frame
    keys.update();
  }
}
