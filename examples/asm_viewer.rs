#![no_std]
#![no_main]

use gba::mem_fns::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
  let dest = unsafe { (0x0400_0000 as *const u16).read_volatile() };
  let src = unsafe { (0x0400_0000 as *const u16).read_volatile() };
  let count = unsafe { (0x0400_0000 as *const u16).read_volatile() };
  unsafe {
    forward_copy_u8(dest as *mut u8, src as *mut u8, count as usize);
    forward_copy_u16(dest as *mut u16, src as *mut u16, count as usize);
    forward_copy_u32(dest as *mut u32, src as *mut u32, count as usize);
    reverse_copy_u8(dest as *mut u8, src as *mut u8, count as usize);
    reverse_copy_u16(dest as *mut u16, src as *mut u16, count as usize);
    reverse_copy_u32(dest as *mut u32, src as *mut u32, count as usize);
  }
  loop {}
}
