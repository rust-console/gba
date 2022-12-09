#![no_std]
#![no_main]

use core::num::Wrapping;

use gba::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  use core::fmt::Write;
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    writeln!(logger, "{info}").ok();
  }
  loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  TIMER0_CONTROL.write(TimerControl::new().with_enabled(true));

  Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);
  Cga8x8Thick.bitunpack_4bpp(OBJ_TILES.as_region(), 0);
  BG_PALETTE.index(1).write(Color::MAGENTA);
  OBJ_PALETTE.index(1).write(Color::CYAN);

  let no_display = ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed);
  OBJ_ATTR0.iter().for_each(|va| va.write(no_display));

  let mut x = Wrapping(13);
  let mut y = Wrapping(37);

  let mut obj = ObjAttr::new();
  obj.set_x(x.0);
  obj.set_y(y.0);
  obj.set_tile_id(1);
  OBJ_ATTR_ALL.index(0).write(obj);

  DISPCNT.write(DisplayControl::new().with_show_obj(true));

  loop {
    // wait for vblank
    VBlankIntrWait();

    // update graphics
    OBJ_ATTR_ALL.index(0).write(obj);

    // get input and prepare next frame
    let keys = KEYINPUT.read();
    if keys.up() {
      y -= 1;
    }
    if keys.down() {
      y += 1;
    }
    if keys.left() {
      x -= 1;
    }
    if keys.right() {
      x += 1;
    }
    obj.set_x(x.0);
    obj.set_y(y.0);
  }
}
