#![no_std]
#![no_main]

use gba::{
  asm_runtime::USER_IRQ_HANDLER,
  bios::VBlankIntrWait,
  mmio::{BACKDROP_COLOR, DISPCNT, DISPSTAT, IE, IME, KEYINPUT},
  video::{Color, DisplayControl, DisplayStatus},
  IrqBits,
};

gba::panic_handler!(empty_loop);

#[no_mangle]
pub extern "C" fn main() -> ! {
  USER_IRQ_HANDLER.write(Some(handler));
  IE.write(IrqBits::new().with_vblank(true));
  IME.write(true);
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  DISPCNT.write(DisplayControl::new());
  loop {
    VBlankIntrWait();
  }
}

extern "C" fn handler(_: IrqBits) {
  let keys = KEYINPUT.read();
  BACKDROP_COLOR.write(Color(keys.0));
}
