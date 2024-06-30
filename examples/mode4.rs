#![no_std]
#![no_main]

use gba::{
  bios::VBlankIntrWait,
  irq::{IrqBits, IE, IME},
  keys::KEYINPUT,
  video::{
    Color, DisplayControl, DisplayStatus, BG_PALRAM, DISPCNT, DISPSTAT,
    MODE4_VRAM,
  },
};

gba::panic_handler!(empty_loop);

#[no_mangle]
pub extern "C" fn main() -> ! {
  IME.write(true);
  IE.write(IrqBits::new().with_vblank(true));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));

  DISPCNT.write(DisplayControl::new().with_bg_mode(4).with_bg2(true));

  let max_x = 2 * MODE4_VRAM.get_frame(0).unwrap().width() as i16 - 1;
  let max_y = MODE4_VRAM.get_frame(0).unwrap().height() as i16 - 1;
  let mut x: i16 = 5;
  let mut y: i16 = 15;
  let mut frame: usize = 0;
  let mut pal_index: u8 = 0;

  {
    let mut red = 1;
    let mut green = 3;
    let mut blue = 5;
    BG_PALRAM.iter().for_each(|addr| {
      red = (red + 3) % 256;
      green = (green + 5) % 256;
      blue = (blue + 7) % 256;
      let color = Color::from_rgb(red, green, blue);
      addr.write(color)
    });
  }

  loop {
    VBlankIntrWait();
    pal_index = pal_index.wrapping_add(1);

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
        .with_bg_mode(4)
        .with_bg2(true)
        .with_frame1_active(frame > 0),
    );

    // We have to do some silly dancing here because we can't write just a `u8`
    // in VRAM. 8-bit writes in VRAM get duplicated across the aligned 16-bit
    // chunk. Instead, we have to read a `u8x2` combo, update part of it, then
    // write it back to VRAM. Normally, you probably wouldn't use Mode 4 at all
    // for a program like this, but it keeps the Mode 4 demo closer to how the
    // Mode 3 demo works.
    let addr = MODE4_VRAM
      .get_frame(frame)
      .unwrap()
      .index((x / 2) as usize, (max_y - y) as usize);
    let old = addr.read();
    addr.write(if (x & 1) != 0 {
      old.with_high(pal_index)
    } else {
      old.with_low(pal_index)
    });
  }
}
