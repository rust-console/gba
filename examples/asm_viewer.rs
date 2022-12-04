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
    __aeabi_memcpy1(dest as *mut u8, src as *mut u8, count as usize);
    __aeabi_memcpy2(dest as *mut u16, src as *mut u16, count as usize);
    __aeabi_memcpy4(dest as *mut u32, src as *mut u32, count as usize);
    __aeabi_memcpy8(dest as *mut u32, src as *mut u32, count as usize);
    __aeabi_memcpy(dest as *mut u8, src as *mut u8, count as usize);
    memcpy(dest as *mut u8, src as *mut u8, count as usize);
    __aeabi_memmove4(dest as *mut u32, src as *mut u32, count as usize);
    __aeabi_memmove8(dest as *mut u32, src as *mut u32, count as usize);
    __aeabi_memmove(dest as *mut u8, src as *mut u8, count as usize);
    memmove(dest as *mut u8, src as *mut u8, count as usize);
    __aeabi_memset4(dest as *mut u32, count as usize, count as i32);
    __aeabi_memset8(dest as *mut u32, count as usize, count as i32);
    __aeabi_memset(dest as *mut u8, count as usize, count as i32);
    memset(dest as *mut u8, count as i32, count as usize);
    __aeabi_memclr4(dest as *mut u32, count as usize);
    __aeabi_memclr8(dest as *mut u32, count as usize);
    __aeabi_memclr(dest as *mut u8, count as usize);
  }
  loop {}
}
