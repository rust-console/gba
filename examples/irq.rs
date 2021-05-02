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
  // disable Interrupt Master Enable to prevent an interrupt during the handler
  unsafe { IME.write(false) };

  // read which interrupts are pending, and "filter" the selection by which are
  // supposed to be enabled.
  let which_interrupts_to_handle = IRQ_PENDING.read() & IE.read();

  // read the current IntrWait value. It sorta works like a running total, so
  // any interrupts we process we'll enable in this value, which we write back
  // at the end.
  let mut intr_wait_flags = INTR_WAIT_ACKNOWLEDGE.read();

  if which_interrupts_to_handle.vblank() {
    vblank_handler();
    intr_wait_flags.set_vblank(true);
  }
  if which_interrupts_to_handle.hblank() {
    hblank_handler();
    intr_wait_flags.set_hblank(true);
  }
  if which_interrupts_to_handle.vcount() {
    vcount_handler();
    intr_wait_flags.set_vcount(true);
  }
  if which_interrupts_to_handle.timer0() {
    timer0_handler();
    intr_wait_flags.set_timer0(true);
  }
  if which_interrupts_to_handle.timer1() {
    timer1_handler();
    intr_wait_flags.set_timer1(true);
  }

  // acknowledge that we did stuff.
  IRQ_ACKNOWLEDGE.write(which_interrupts_to_handle);

  // write out any IntrWait changes.
  unsafe { INTR_WAIT_ACKNOWLEDGE.write(intr_wait_flags) };

  // re-enable as we go out.
  unsafe { IME.write(true) };
}

fn vblank_handler() {
  write_pixel(BLUE);
}

fn hblank_handler() {
  write_pixel(GREEN);
}

fn vcount_handler() {
  write_pixel(RED);
}

fn timer0_handler() {
  write_pixel(YELLOW);
}

fn timer1_handler() {
  write_pixel(PINK);
}
