#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

//#[inline(always)]
#[instruction_set(arm::a32)]
fn make_3rd_bg_pal_entry_black() {
  let x: u16;
  unsafe { core::arch::asm!("movs {}, #0", out(reg) x) };
  BG_PALETTE.index(3).write(Color(x));
}

#[no_mangle]
#[instruction_set(arm::t32)]
extern "C" fn main() -> ! {
  make_3rd_bg_pal_entry_black();
  loop {}
}
