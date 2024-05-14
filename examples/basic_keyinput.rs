#![no_std]
#![no_main]

use gba::{
  mmio::{BACKDROP_COLOR, DISPCNT, KEYINPUT},
  video::{Color, DisplayControl},
};

gba::panic_handler!(empty_loop);

#[no_mangle]
pub extern "C" fn main() -> ! {
  DISPCNT.write(DisplayControl::new());
  loop {
    let keys = KEYINPUT.read();
    BACKDROP_COLOR.write(Color(keys.0));
  }
}
