#![no_std]
#![feature(start)]
#![forbid(unsafe_code)]

use gba::{
  fatal,
  io::display::{DisplayControlSetting, DisplayMode, DISPCNT},
  vram::bitmap::{Mode3, Mode4, Mode5, Page},
  warn, Color,
};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  fatal!("{}", info);
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  const SETTING: DisplayControlSetting =
    DisplayControlSetting::new().with_mode(DisplayMode::Mode5).with_bg2(true);
  DISPCNT.write(SETTING);

  use gba::io::timers::*;
  let tcs0 = TimerControlSetting::new().with_enabled(true);
  let tcs1 = TimerControlSetting::new().with_tick_rate(TimerTickRate::Cascade).with_enabled(true);

  TM1CNT_H.write(tcs1);
  TM0CNT_H.write(tcs0);
  let start = TM0CNT_L.read();
  Mode5::clear_to(Page::Zero, Color(0));
  let end0 = TM0CNT_L.read();
  let end1 = TM1CNT_L.read();
  warn!("CLEAR_TO: start:{}, end0:{}, end1:{}", start, end0, end1);

  // reset
  TM1CNT_H.write(TimerControlSetting::new());
  TM0CNT_H.write(TimerControlSetting::new());

  TM1CNT_H.write(tcs1);
  TM0CNT_H.write(tcs0);
  let start = TM0CNT_L.read();
  Mode5::dma_clear_to(Page::Zero, Color(0));
  let end0 = TM0CNT_L.read();
  let end1 = TM1CNT_L.read();
  warn!("DMA_CLEAR_TO: start:{}, end0:{}, end1:{}", start, end0, end1);

  DISPCNT.write(DisplayControlSetting::new().with_mode(DisplayMode::Mode3).with_bg2(true));
  loop {
    let this_frame_keys = gba::io::keypad::read_key_input();
    gba::io::display::spin_until_vblank();
    Mode3::dma_clear_to(Color(111));
    Mode3::draw_line(5, 5, 240, 160, Color(0b0_11111_11111_11111));
    gba::io::display::spin_until_vdraw();
  }
}
