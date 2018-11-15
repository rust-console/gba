#![feature(start)]
#![no_std]

extern crate gba;
use gba::{io_registers::*, video_ram::*};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  let mode3bg2 = {
    let mut setting = DisplayControlSetting::default();
    setting.set_display_bg2(true);
    setting.set_mode(DisplayControlMode::Bitmap3);
    setting
  };
  set_display_control(mode3bg2);

  let red = rgb16(31, 0, 0);
  let green = rgb16(0, 31, 0);

  loop {
    let this_frame_keys = key_input();

    let this_frame_key_raw: u16 = unsafe { core::mem::transmute(this_frame_keys) };

    wait_until_vblank();

    for i in 0..16 {
      let key = ((this_frame_key_raw >> i) & 1) > 0;
      mode3_draw_pixel(15 - i, 0, if key { green } else { red });
      mode3_draw_pixel(15 - i, 1, if key { green } else { red });
    }

    wait_until_vdraw();
  }
}
