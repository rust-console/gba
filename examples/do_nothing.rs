#![no_std]
#![no_main]

gba::panic_handler!(empty_loop);

#[no_mangle]
pub extern "C" fn main() -> ! {
  loop {}
}
