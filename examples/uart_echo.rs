#![no_std]
#![no_main]

//      _      Link Cable Pinout
//  ___/ \___  1: VCC - 3.3V
// /         \ 2: SO - TX
// |  1 3 5  | 3: SI - RX
// |  2 4 6  | 4: SD
// |_________| 5: SC
//             6: GND

use embedded_hal::prelude::*;
use nb::block;

use gba::serial::{BaudRate, SioSerial};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
fn main() -> ! {
  let mut serial = SioSerial::init(BaudRate::Bps115200);

  loop {
    if let Ok(c) = block!(serial.read()) {
      block!(serial.write(c)).ok();
    }
  }
}
