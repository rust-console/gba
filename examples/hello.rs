#![no_std]
#![no_main]
#![feature(isa_attribute)]

use gba::{bios::VBlankIntrWait, gba_cell::GbaCell, IrqFn, RUST_IRQ_HANDLER};

extern crate gba;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

static KEYS: GbaCell<u16> = GbaCell::new(0);

const KEYINPUT: *mut u16 = 0x0400_0130 as *mut u16;
const DISPSTAT: *mut u16 = 0x0400_0004 as *mut u16;
const IME: *mut bool = 0x0400_0208 as *mut bool;
const IE: *mut u16 = 0x0400_0200 as *mut u16;
const BACKDROP_COLOR: *mut u16 = 0x05000000 as *mut u16;
const DISPCNT: *mut u16 = 0x0400_0000 as *mut u16;

extern "C" fn irq_handler(_: u16) {
  KEYS.write(unsafe { KEYINPUT.read_volatile() });
}

#[no_mangle]
fn main() {
  RUST_IRQ_HANDLER.write(IrqFn(Some(irq_handler)));
  unsafe { DISPSTAT.write_volatile(1 << 3) };
  unsafe { IE.write_volatile(1) };
  unsafe { IME.write_volatile(true) };

  unsafe { DISPCNT.write_volatile(1 << 8) };

  loop {
    VBlankIntrWait();

    let k = KEYS.read();
    unsafe { BACKDROP_COLOR.write_volatile(k) };
  }
}
