#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[instruction_set(arm::a32)]
fn make_3rd_bg_pal_entry_black() {
  BG_PALETTE.index(3).write(Color::new());
}

#[no_mangle]
extern "C" fn main() -> ! {
  make_3rd_bg_pal_entry_black();
  loop {}
}
