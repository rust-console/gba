#![no_std]
#![feature(start)]
#![forbid(unsafe_code)]

use gba::{
  io::{
    display::{spin_until_vblank, spin_until_vdraw, DisplayControlMode, DisplayControlSetting, DISPCNT},
    keypad::read_key_input,
  },
  video::Mode3,
  Color,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  const SETTING: DisplayControlSetting = DisplayControlSetting::new().with_mode(DisplayControlMode::Bitmap3).with_display_bg2(true);
  DISPCNT.write(SETTING);

  let mut px = Mode3::SCREEN_WIDTH / 2;
  let mut py = Mode3::SCREEN_HEIGHT / 2;
  let mut color = Color::from_rgb(31, 0, 0);

  loop {
    // read the input for this frame
    let this_frame_keys = read_key_input();

    // adjust game state and wait for vblank
    px = px.wrapping_add(2 * this_frame_keys.column_direction() as usize);
    py = py.wrapping_add(2 * this_frame_keys.row_direction() as usize);
    spin_until_vblank();

    // draw the new game and wait until the next frame starts.
    const BLACK: Color = Color::from_rgb(0, 0, 0);
    if px >= Mode3::SCREEN_WIDTH || py >= Mode3::SCREEN_HEIGHT {
      // out of bounds, reset the screen and position.
      Mode3::clear_to(BLACK);
      color = color.rotate_left(5);
      px = Mode3::SCREEN_WIDTH / 2;
      py = Mode3::SCREEN_HEIGHT / 2;
    } else {
      let color_here = Mode3::read_pixel(px, py);
      if color_here != Some(BLACK) {
        // crashed into our own line, reset the screen
        Mode3::clear_to(BLACK);
        color = color.rotate_left(5);
      } else {
        // draw the new part of the line
        Mode3::write_pixel(px, py, color);
        Mode3::write_pixel(px, py + 1, color);
        Mode3::write_pixel(px + 1, py, color);
        Mode3::write_pixel(px + 1, py + 1, color);
      }
    }
    spin_until_vdraw();
  }
}
