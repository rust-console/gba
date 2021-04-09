#![no_std]
#![no_main]
#![feature(isa_attribute)]

use gba::prelude::*;

const BLACK: Color = Color::from_rgb(0, 0, 0);
const RED: Color = Color::from_rgb(31, 0, 0);
const GREEN: Color = Color::from_rgb(0, 31, 0);
const BLUE: Color = Color::from_rgb(0, 0, 31);
const YELLOW: Color = Color::from_rgb(31, 31, 0);
const PINK: Color = Color::from_rgb(31, 0, 31);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

fn start_timers() {
  let init_val: u16 = u32::wrapping_sub(0x1_0000, 64) as u16;
  const TIMER_SETTINGS: TimerControl =
    TimerControl::new().with_irq_on_overflow(true).with_enabled(true);

  TIMER0_RELOAD.write(init_val);
  TIMER0_CONTROL.write(TIMER_SETTINGS.with_prescaler_selection(3));
  TIMER1_RELOAD.write(init_val);
  TIMER1_CONTROL.write(TIMER_SETTINGS.with_prescaler_selection(1));
}

#[no_mangle]
fn main() -> ! {
  DISPCNT.write(DisplayControl::new().with_display_mode(3).with_display_bg2(true));
  mode3::dma3_clear_to(BLACK);

  // Set the IRQ handler to use.
  unsafe { USER_IRQ_HANDLER.write(Some(irq_handler_a32)) };

  // Enable all interrupts that are set in the IE register.
  unsafe { IME.write(true) };

  // Request that VBlank, HBlank and VCount will generate IRQs.
  const DISPLAY_SETTINGS: DisplayStatus = DisplayStatus::new()
    .with_vblank_irq_enabled(true)
    .with_hblank_irq_enabled(true)
    .with_vcount_irq_enabled(true);
  DISPSTAT.write(DISPLAY_SETTINGS);

  // Start two timers with overflow IRQ generation.
  start_timers();

  loop {
    let this_frame_keys: Keys = KEYINPUT.read().into();

    // The VBlank IRQ must be enabled at minimum, or else the CPU will halt
    // at the call to vblank_interrupt_wait() as the VBlank IRQ will never
    // be triggered.
    let mut flags = InterruptFlags::new().with_vblank(true);

    // Enable interrupts based on key input.
    if this_frame_keys.a() {
      flags = flags.with_hblank(true);
    }
    if this_frame_keys.b() {
      flags = flags.with_vcount(true);
    }
    if this_frame_keys.l() {
      flags = flags.with_timer0(true);
    }
    if this_frame_keys.r() {
      flags = flags.with_timer1(true);
    }

    unsafe { IE.write(flags) };

    // Puts the CPU into low power mode until a VBlank IRQ is received. This
    // will yield considerably better power efficiency as opposed to spin
    // waiting.
    unsafe { VBlankIntrWait() };
  }
}

static mut PIXEL: usize = 0;

fn write_pixel(color: Color) {
  unsafe {
    (0x0600_0000 as *mut Color).wrapping_offset(PIXEL as isize).write_volatile(color);
    PIXEL += 1;
    if PIXEL == (mode3::WIDTH * mode3::HEIGHT) {
      PIXEL = 0;
    }
  }
}

#[instruction_set(arm::a32)]
extern "C" fn irq_handler_a32() {
  // we just use this a32 function to jump over back to t32 code.
  irq_handler_t32()
}

fn irq_handler_t32() {
  let flags = IRQ_PENDING.read();

  if flags.vblank() {
    vblank_handler();
  }
  if flags.hblank() {
    hblank_handler();
  }
  if flags.vcount() {
    vcount_handler();
  }
  if flags.timer0() {
    timer0_handler();
  }
  if flags.timer1() {
    timer1_handler();
  }
}

fn vblank_handler() {
  write_pixel(BLUE);

  // When using `interrupt_wait()` or `vblank_interrupt_wait()`, IRQ handlers must acknowledge
  // the IRQ on the BIOS Interrupt Flags register.
  unsafe { INTR_WAIT_ACKNOWLEDGE.write(INTR_WAIT_ACKNOWLEDGE.read().with_vblank(true)) };
}

fn hblank_handler() {
  write_pixel(GREEN);

  unsafe { INTR_WAIT_ACKNOWLEDGE.write(INTR_WAIT_ACKNOWLEDGE.read().with_hblank(true)) };
}

fn vcount_handler() {
  write_pixel(RED);

  unsafe { INTR_WAIT_ACKNOWLEDGE.write(INTR_WAIT_ACKNOWLEDGE.read().with_vcount(true)) };
}

fn timer0_handler() {
  write_pixel(YELLOW);

  unsafe { INTR_WAIT_ACKNOWLEDGE.write(INTR_WAIT_ACKNOWLEDGE.read().with_timer0(true)) };
}

fn timer1_handler() {
  write_pixel(PINK);

  unsafe { INTR_WAIT_ACKNOWLEDGE.write(INTR_WAIT_ACKNOWLEDGE.read().with_timer1(true)) };
}
