#![no_std]
#![no_main]

use gba::{
  bios::VBlankIntrWait,
  mmio::{DISPCNT, DISPSTAT, IE, IME, KEYINPUT, MODE5_VRAM},
  video::{Color, DisplayControl, DisplayStatus},
  IrqBits,
};

gba::panic_handler!(empty_loop);

#[no_mangle]
pub extern "C" fn main() -> ! {
  IME.write(true);
  IE.write(IrqBits::new().with_vblank(true));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));

  DISPCNT.write(DisplayControl::new().with_bg_mode(5).with_bg2(true));

  let max_x = MODE5_VRAM.get_frame(0).unwrap().width() as i16 - 1;
  let max_y = MODE5_VRAM.get_frame(0).unwrap().height() as i16 - 1;
  let mut x: i16 = 5;
  let mut y: i16 = 15;
  let mut frame: usize = 0;
  let mut color = Color(0b0_11100_11110_11111);

  loop {
    VBlankIntrWait();
    color.0 = color.0.rotate_left(1);

    // keypad moves the update point
    let keys = KEYINPUT.read();
    x = (x + i16::from(keys.dx())).clamp(0, max_x);
    y = (y + i16::from(keys.dy())).clamp(0, max_y);

    // L and R pick the frame
    if keys.r() && frame == 0 {
      frame = 1;
    }
    if keys.l() && frame == 1 {
      frame = 0;
    }
    DISPCNT.write(
      DisplayControl::new()
        .with_bg_mode(5)
        .with_bg2(true)
        .with_frame1_active(frame > 0),
    );

    let addr = MODE5_VRAM
      .get_frame(frame)
      .unwrap()
      .index(x as usize, (max_y - y) as usize);
    addr.write(color);
  }
}
