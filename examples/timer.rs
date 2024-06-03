#![no_std]
#![no_main]

use gba::{
  bios::VBlankIntrWait,
  gba_cell::GbaCell,
  irq::{IrqBits, IE, IME, USER_IRQ_HANDLER},
  timers::{CpusPerTick, TimerControl, TIMER0_CONTROL},
  video::{
    Color, DisplayControl, DisplayStatus, BACKDROP_COLOR, DISPCNT, DISPSTAT,
  },
};

static OVERFLOWS: GbaCell<u32> = GbaCell::new(0);

gba::panic_handler!(mgba_log_err);

#[no_mangle]
extern "C" fn main() -> ! {
  USER_IRQ_HANDLER.write(Some(irq_handler));
  IE.write(IrqBits::new().with_vblank(true).with_timer0(true));
  IME.write(true);
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));

  TIMER0_CONTROL.write(
    TimerControl::new()
      .with_enabled(true)
      .with_send_irq(true)
      .with_cpus_per_tick(CpusPerTick::_64),
  );

  DISPCNT.write(DisplayControl::new().with_bg_mode(3));
  loop {
    VBlankIntrWait();
    BACKDROP_COLOR.write(Color(OVERFLOWS.read() as u16));
  }
}

extern "C" fn irq_handler(bits: IrqBits) {
  if bits.timer0() {
    OVERFLOWS.write(OVERFLOWS.read().wrapping_add(1));
  }
}
