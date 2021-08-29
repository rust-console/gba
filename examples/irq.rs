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

    // Set the IRQ handlers to use.
    irq::set_handler(IrqKind::VBlank, Some(vblank_handler));
    irq::set_handler(IrqKind::HBlank, Some(hblank_handler));
    irq::set_handler(IrqKind::VCount, Some(vcount_handler));
    irq::set_handler(IrqKind::Timer0, Some(timer0_handler));
    irq::set_handler(IrqKind::Timer1, Some(timer1_handler));

    // Set the flags needed to enable each IRQ.
    irq::enable(IrqKind::VBlank);
    irq::enable(IrqKind::HBlank);
    irq::enable(IrqKind::VCount);
    irq::enable(IrqKind::Timer0);
    irq::enable(IrqKind::Timer1);

    // Enable all interrupts that were set above.
    irq::set_master_enabled(true);

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

// All interrupt handlers *must* be compiled as Thumb code by annotating them
// with `#[instruction_set(arm::t32)]`.
//
// To handle nested interrupts, write the appropriate `InterruptFlags` to the
// `IE` register from the appropriate handler function. `IE` will be cleared
// before each user-defined interrupt handler is jumped to from the main
// "switchboard" interrupt handler, and will be restored afterwards.

#[instruction_set(arm::t32)]
extern "C" fn vblank_handler() {
    write_pixel(BLUE);
}

#[instruction_set(arm::t32)]
extern "C" fn hblank_handler() {
    write_pixel(GREEN);
}

#[instruction_set(arm::t32)]
extern "C" fn vcount_handler() {
    write_pixel(RED);
}

#[instruction_set(arm::t32)]
extern "C" fn timer0_handler() {
    write_pixel(YELLOW);
}

#[instruction_set(arm::t32)]
extern "C" fn timer1_handler() {
    write_pixel(PINK);
}
