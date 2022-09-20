#![no_std]
#![no_main]
#![feature(isa_attribute)]

use gba::{
  asm_runtime::RUST_IRQ_HANDLER,
  bios::VBlankIntrWait,
  gba_cell::GbaCell,
  interrupts::IrqBits,
  keys::KeyInput,
  mmio::{BACKDROP_COLOR, DISPCNT, DISPSTAT, IE, IME, KEYINPUT},
  video::{Color, DisplayControl, DisplayStatus},
};

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

static KEYS: GbaCell<KeyInput> = GbaCell::new(KeyInput::new());

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

  DISPCNT.write(DisplayControl::new().with_show_bg0(true));

  loop {
    VBlankIntrWait();

    let k = KEYS.read();
    BACKDROP_COLOR.write(Color(k.to_u16()));
  }
}
