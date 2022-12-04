#![no_std]
#![no_main]
#![allow(unused_imports)]

//! Scratch space for checking the asm output of stuff.

use gba::prelude::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
  loop {}
}
