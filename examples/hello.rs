#![no_std]
#![no_main]
#![feature(isa_attribute)]

use gba::{
  bios::VBlankIntrWait,
  gba_cell::GbaCell,
  interrupts::{IrqBits, IE, IME},
  keys::KEYINPUT,
  runtime::RUST_IRQ_HANDLER,
  video::{
    Color, DisplayControl, DisplayStatus, BACKDROP_COLOR, DISPCNT, DISPSTAT,
  },
};

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

static KEYS: GbaCell<u16> = GbaCell::new(0);

extern "C" fn irq_handler(_: u16) {
  KEYS.write(KEYINPUT.read());
}

#[no_mangle]
extern "C" fn main() -> ! {
  RUST_IRQ_HANDLER.write(Some(irq_handler));
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::new().with_vblank(true));
  IME.write(true);

  DISPCNT.write(DisplayControl::new().with_show_bg0(true));

  loop {
    VBlankIntrWait();

    let k = KEYS.read();
    BACKDROP_COLOR.write(Color(k));
  }
}
