#![no_std]
#![no_main]
#![feature(isa_attribute)]

use core::fmt::Write;
use gba::{
  mgba::{MgbaBufferedLogger, MgbaMessageLevel},
  prelude::*,
};

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    write!(logger, "{info}").ok();
  }
  loop {}
}

static KEYS: GbaCell<KeyInput> = GbaCell::new(KeyInput::new());

#[link_section = ".ewram"]
static VALUE: GbaCell<u16> = GbaCell::new(0);

extern "C" fn irq_handler(_: u16) {
  // just as a demo, we'll read the keys during vblank.
  KEYS.write(KEYINPUT.read());
}

#[no_mangle]
extern "C" fn main() -> ! {
  RUST_IRQ_HANDLER.write(Some(irq_handler));
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Debug) {
    writeln!(logger, "hello!").ok();
  }

  DISPCNT.write(DisplayControl::new().with_show_bg0(true));

  loop {
    VBlankIntrWait();

    let k = KEYS.read();
    VALUE.write((k.to_u16() + 3) / k.to_u16()); // force a runtime division
    BACKDROP_COLOR.write(Color(k.to_u16()));
  }
}
