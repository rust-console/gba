#![no_std]
#![no_main]

use gba::{
  bios::VBlankIntrWait,
  irq::{IrqBits, IE, IME},
  keys::KEYINPUT,
  video::{
    Color, DisplayControl, DisplayStatus, DISPCNT, DISPSTAT, MODE3_VRAM,
  },
};

gba::panic_handler!(empty_loop);

#[no_mangle]
pub extern "C" fn main() -> ! {
  IME.write(true);
  IE.write(IrqBits::new().with_vblank(true));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));

  DISPCNT.write(DisplayControl::new().with_bg_mode(3).with_bg2(true));

  let max_x = MODE3_VRAM.width() as i16 - 1;
  let max_y = MODE3_VRAM.height() as i16 - 1;
  let mut x: i16 = 5;
  let mut y: i16 = 15;
  let mut color = Color(0b0_11100_11110_11111);

  loop {
    VBlankIntrWait();
    color.0 = color.0.rotate_left(1);

    let keys = KEYINPUT.read();
    x = (x + i16::from(keys.dx())).clamp(0, max_x);
    y = (y + i16::from(keys.dy())).clamp(0, max_y);

    let addr = MODE3_VRAM.index(x as usize, (max_y - y) as usize);
    addr.write(color);
  }
}
