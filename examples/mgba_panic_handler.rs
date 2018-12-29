#![no_std]
#![feature(start)]
#![forbid(unsafe_code)]

use gba::{
  io::display::{DisplayControlSetting, DisplayMode, DISPCNT},
  vram::bitmap::Mode3,
  Color,
};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  use core::fmt::Write;
  use gba::mgba::{MGBADebug, MGBADebugLevel};

  if let Some(mut mgba) = MGBADebug::new() {
    let _ = write!(mgba, "{}", info);
    mgba.send(MGBADebugLevel::Fatal);
  }
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  const SETTING: DisplayControlSetting = DisplayControlSetting::new().with_mode(DisplayMode::Mode3).with_bg2(true);
  DISPCNT.write(SETTING);
  Mode3::write_pixel(120, 80, Color::from_rgb(31, 0, 0));
  Mode3::write_pixel(136, 80, Color::from_rgb(0, 31, 0));
  Mode3::write_pixel(120, 96, Color::from_rgb(0, 0, 31));
  panic!("fumoffu!");
}
