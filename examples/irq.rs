#![no_std]
#![feature(start)]

use gba::{
  io::{
    display::{DisplayControlSetting, DisplayMode, DisplayStatusSetting, DISPCNT, DISPSTAT},
    irq::{self, IrqEnableSetting, IrqFlags, BIOS_IF, IE, IME},
    keypad::read_key_input,
    timers::{TimerControlSetting, TimerTickRate, TM0CNT_H, TM0CNT_L, TM1CNT_H, TM1CNT_L},
  },
  vram::bitmap::Mode3,
  Color,
};

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
  const TIMER_SETTINGS: TimerControlSetting =
    TimerControlSetting::new().with_overflow_irq(true).with_enabled(true);

  TM0CNT_L.write(init_val);
  TM0CNT_H.write(TIMER_SETTINGS.with_tick_rate(TimerTickRate::CPU1024));
  TM1CNT_L.write(init_val);
  TM1CNT_H.write(TIMER_SETTINGS.with_tick_rate(TimerTickRate::CPU64));
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  DISPCNT.write(DisplayControlSetting::new().with_mode(DisplayMode::Mode3).with_bg2(true));
  Mode3::clear_to(BLACK);

  // Set the IRQ handler to use.
  irq::set_irq_handler(irq_handler);

  // Enable all interrupts that are set in the IE register.
  unsafe { IME.write(IrqEnableSetting::IRQ_YES) };

  // Request that VBlank, HBlank and VCount will generate IRQs.
  const DISPLAY_SETTINGS: DisplayStatusSetting = DisplayStatusSetting::new()
    .with_vblank_irq_enable(true)
    .with_hblank_irq_enable(true)
    .with_vcounter_irq_enable(true);
  DISPSTAT.write(DISPLAY_SETTINGS);

  // Start two timers with overflow IRQ generation.
  start_timers();

  loop {
    let this_frame_keys = read_key_input();

    // The VBlank IRQ must be enabled at minimum, or else the CPU will halt
    // at the call to vblank_interrupt_wait() as the VBlank IRQ will never
    // be triggered.
    let mut flags = IrqFlags::new().with_vblank(true);

    // Enable interrupts based on key input.
    if this_frame_keys.a() {
      flags = flags.with_hblank(true);
    }
    if this_frame_keys.b() {
      flags = flags.with_vcounter(true);
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
    gba::bios::vblank_interrupt_wait();
  }
}

static mut PIXEL: usize = 0;

fn write_pixel(color: Color) {
  unsafe {
    Mode3::write(PIXEL, 0, color);
    PIXEL = (PIXEL + 1) % (Mode3::WIDTH * Mode3::HEIGHT);
  }
}

extern "C" fn irq_handler(flags: IrqFlags) {
  if flags.vblank() {
    vblank_handler();
  }
  if flags.hblank() {
    hblank_handler();
  }
  if flags.vcounter() {
    vcounter_handler();
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
  unsafe { BIOS_IF.write(BIOS_IF.read().with_vblank(true)) };
}

fn hblank_handler() {
  write_pixel(GREEN);

  unsafe { BIOS_IF.write(BIOS_IF.read().with_hblank(true)) };
}

fn vcounter_handler() {
  write_pixel(RED);

  unsafe { BIOS_IF.write(BIOS_IF.read().with_vcounter(true)) };
}

fn timer0_handler() {
  write_pixel(YELLOW);

  unsafe { BIOS_IF.write(BIOS_IF.read().with_timer0(true)) };
}

fn timer1_handler() {
  write_pixel(PINK);

  unsafe { BIOS_IF.write(BIOS_IF.read().with_timer1(true)) };
}
