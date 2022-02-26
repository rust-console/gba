#![no_std]
#![no_main]

//! The `minimum` example puts a single red pixel on the screen without any
//! outside files beyond the standard linker script.

core::arch::global_asm!(
  ".arm
  .section .text.rom_header
  .global __start
  __start:
    b asm_init
    @ Blank all the way out to the end of the multi-boot header.
    @ This prevents some emulators from mis-detecting the ROM type.
    .space 0xE0

  asm_init:
    @ todo

  call_to_rust_main:
    ldr lr, =1f
    ldr r0, =main
    bx r0
    @ `main` should never return,
    @ but putting this safety loop costs us so little we'll just do it.
    1: b 1b
  .previous
  .thumb"
);

#[no_mangle]
extern "C" fn main() -> ! {
  unsafe { (0x0600_0000 as *mut u16).write(0b00000_00000_11111) };
  unsafe { (0x0400_0000 as *mut u16).write(3 | (1 << 10)) };
  panic!("nothing to do.");
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
  // TODO: log the info to debug.
  loop {}
}
