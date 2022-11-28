#![no_std]
#![no_main]

use core::fmt::Write;
use gba::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
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

  let mut obj = ObjAttr::new();
  obj.set_x(13);
  obj.set_y(37);
  obj.set_tile_id(1);
  OBJ_ATTR_ALL.index(0).write(obj);

  DISPCNT.write(DisplayControl::new().with_show_obj(true));

  loop {
    VBlankIntrWait();
    let keys = KEYINPUT.read();
    if keys.a() {
      let t = TIMER0_COUNT.read();
      BACKDROP_COLOR.write(Color(t));
    }
  }
}
