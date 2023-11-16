/*
 * Made by Evan Goemer
 * Discord: @evangoemer
 */

#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
fn main() -> ! {
  DISPCNT.write(
    DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true),
  );

  let mut red = 0;
  let mut green = 255;
  let mut blue = 0;

  for y in 0..160 {
    for x in 0..240 {
      let color = Color::from_rgb(red, green, blue);
      VIDEO3_VRAM.index(x, y).write(color);

      red = (red + 1) % 256;
      green = (green + 3) % 256;
      blue = (blue + 5) % 256;
    }
  }
  loop {}
}
