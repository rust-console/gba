#![no_std]
#![no_main]
#![allow(unused)]

use core::ptr::{addr_of, addr_of_mut};

use gba::{
  asm_runtime::USER_IRQ_HANDLER,
  bios::VBlankIntrWait,
  irq::{IrqBits, IE, IME},
  keys::KEYINPUT,
  mgba::{MgbaLogLevel, MgbaLogger},
  video::{
    Color, DisplayControl, DisplayStatus, BACKDROP_COLOR, DISPCNT, DISPSTAT,
  },
};

gba::panic_handler!(mgba_log_err);

#[no_mangle]
pub extern "C" fn main() -> ! {
  // Set a handler, and then configure interrupts on (IME), vblank interrupts
  // enabled for receiving (IE), and vblank interrupts being sent (DISPSTAT).
  // All these steps can be done in any order.
  USER_IRQ_HANDLER.write(Some(handler));
  IME.write(true);
  IE.write(IrqBits::new().with_vblank(true));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));

  // Once the program is ready, we turn off the forced blank bit in the display
  // to begin showing things, and it will trigger a vblank interrupt after each
  // draw cycle.
  DISPCNT.write(DisplayControl::new());

  // The body of the game is to just sleep until each vblank (this saves a lot
  // of battery power), then immediately upon waking we just go back to sleep.
  // The handler is effectively run "during" this wait call (after the GBA wakes
  // up, but before the BIOS function call returns).
  loop {
    VBlankIntrWait();
  }
}

extern "C" fn handler(_: IrqBits) {
  // As an example of what we can do with the per-frame key data, we use it to
  // set a color to the backdrop. When keys are pressed/released, the color of
  // the backdrop will change.
  let keys = KEYINPUT.read();
  BACKDROP_COLOR.write(Color(keys.0));
}
