#![no_std]
#![feature(start)]
#![forbid(unsafe_code)]

use gba::{
  fatal,
  io::display::{DisplayControlSetting, DisplayMode, DISPCNT},
  vram::bitmap::Mode3,
  Color,
};
use gba::io::keypad::read_key_input;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  // This kills the emulation with a message if we're running within mGBA.
  fatal!("{}", info);
  // If we're _not_ running within mGBA then we still need to not return, so
  // loop forever doing nothing.
  loop {}
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

    gba::io::display::spin_until_vblank();
    Mode5::dma_clear_to(Page::Zero, Color(111));
    Mode5::draw_line(Page::Zero, 5, 5, 100, 100, Color(0b0_11111_11111_11111));
    gba::io::display::spin_until_vdraw();
  }
}
