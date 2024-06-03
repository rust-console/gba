#![no_std]
#![no_main]

use gba::{
  asm_runtime::USER_IRQ_HANDLER,
  bios::VBlankIntrWait,
  gba_cell::GbaCell,
  mmio::{BACKDROP_COLOR, DISPCNT, DISPSTAT, IE, IME, TIMER0_CONTROL},
  timers::{CpusPerTick, TimerControl},
  video::{Color, DisplayControl, DisplayStatus},
  IrqBits,
};

static SECONDS: GbaCell<u32> = GbaCell::new(0);

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
      // .with_cpus_per_tick(CpusPerTick::_64),
  );

  DISPCNT.write(DisplayControl::new().with_bg_mode(3));
  loop {
    VBlankIntrWait();
    BACKDROP_COLOR.write(Color(SECONDS.read() as u16));
  }
}

extern "C" fn irq_handler(bits: IrqBits) {
  if bits.timer0() {
    SECONDS.write(SECONDS.read().wrapping_add(1));
  }
}
