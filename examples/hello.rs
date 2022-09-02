#![no_std]
#![no_main]
#![feature(isa_attribute)]

use core::sync::atomic::{AtomicU16, Ordering};

use gba::set_rust_irq_handler;

extern crate gba;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

static KEYS: AtomicU16 = AtomicU16::new(0);

const KEYINPUT: *mut u16 = 0x0400_0130 as *mut u16;
const DISPSTAT: *mut u16 = 0x0400_0004 as *mut u16;
const IME: *mut bool = 0x0400_0208 as *mut bool;
const IE: *mut u16 = 0x0400_0200 as *mut u16;
const BACKDROP_COLOR: *mut u16 = 0x05000000 as *mut u16;
const DISPCNT: *mut u16 = 0x0400_0000 as *mut u16;

extern "C" fn irq_handler(_: u16) {
  KEYS.store(unsafe { KEYINPUT.read_volatile() }, Ordering::SeqCst);
}

#[no_mangle]
fn main() {
  set_rust_irq_handler(irq_handler);
  unsafe { DISPSTAT.write_volatile(1 << 3) };
  unsafe { IE.write_volatile(1) };
  unsafe { IME.write_volatile(true) };

  unsafe { DISPCNT.write_volatile(1 << 8) };

  loop {
    VBlankIntrWait();

    let k = KEYS.load(Ordering::SeqCst);
    unsafe { BACKDROP_COLOR.write_volatile(k) };
  }
}

#[inline]
#[instruction_set(arm::t32)]
#[allow(non_snake_case)]
pub fn VBlankIntrWait() {
  unsafe {
    core::arch::asm! {
      "swi #0x05",
      out("r0") _,
      out("r1") _,
      out("r3") _,
      options(preserves_flags),
    }
  };
}

#[no_mangle]
unsafe extern "C" fn __sync_lock_test_and_set_4(
  ptr: *mut i32, val: i32,
) -> i32 {
  let out = unsafe { ptr.read() };
  unsafe { ptr.write(val) };
  out
}
#[no_mangle]
unsafe extern "C" fn __sync_lock_test_and_set_2(
  ptr: *mut i16, val: i16,
) -> i16 {
  let out = unsafe { ptr.read() };
  unsafe { ptr.write(val) };
  out
}
#[no_mangle]
unsafe extern "C" fn __sync_val_compare_and_swap_2(
  ptr: *mut i16, val: i16,
) -> i16 {
  let out = unsafe { ptr.read() };
  unsafe { ptr.write(val) };
  out
}
