//! Module to interface with the GBA's four timer units.
//!
//! Similar to the background layers and DMA units, there are four timer units
//! and they're numbered 0 through 3.
//!
//! There's two hardware addresses that control each timer.
//! * The timer's high address is the [`TimerControl`] bits.
//! * The timer's low address is a `u16` which *reads* the timer's "count"
//!   value, but *writes* the timer's "reload" value. In this crate we actually
//!   represent that as two separate MMIO controls for improved code clarity.
//!   Just be aware that in mGBA's debugger and in other documentation you'll
//!   see it as a single address.
//!
//! When a timer is disabled, it will continue to read the count value that it
//! stopped at.
//!
//! ## Reloading
//!
//! When the timer goes from disabled to enabled, or when the timer overflows,
//! the last set reload value is copied to the counter value.
//!
//! ## Ticking
//!
//! When a timer is enabled, the timer will tick every so often. Each tick
//! increases the counter value by 1. The rate at which the timer ticks depends
//! on the timer's configuration:
//!
//! * If the `cascade` bit is set the timer will tick once per overflow of the
//!   next lower timer. For example, if timer 3 is set to cascade, it will tick
//!   once per overflow of timer 2. Note that timer 0 ignores the cascade bit,
//!   since it doesn't have a "next lower" timer.
//! * Otherwise, the timer ticks every one or more CPU cycles, according to the
//!   [`TimerScale`] set in the `scale` field.
//!
//! ## Overflows
//!
//! When a timer would tick *above* `u16::MAX` then an overflow occurs. This can
//! trigger an interrupt, and will also cause the timer to copy its reload value
//! into its counter.
//!
//! If you want a timer to overflow every `x` ticks (where `x` is non-zero),
//! then use the [`wrapping_neg`](u16::wrapping_neg) method to easily get the
//! right reload value to set:
//!
//! ```
//! # use gba::prelude::*;
//! let x = 7_u16;
//! TIMER0_RELOAD.write(x.wrapping_neg());
//! ```
//!
//! ## Using Cascade To Pause A Timer
//!
//! When a timer goes from disabled to enabled it will reset the counter value
//! to the reload value. If you want to temporarily pause a timer *without*
//! having the counter value get reset when you resume the timer you can instead
//! set the `cascade` bit of the timer while the next lower timer is
//! **disabled**. This keeps the timer "active" but prevents it from ticking.
//! When you turn off cascade mode the timer will resume ticking from the
//! current counter value.
//!
//! Note that this doesn't work for timer 0, because that timer ignores the
//! cascade bit.

use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field};

/// A number of CPU cycles per timer tick.
///
/// * The GBA's CPU runs at 16,777,216 cycles per second (16.78 Mhz).
/// * The GBA's PPU outputs one pixel per 4 CPU cycles.
/// * It takes 280,896 cycles for one full frame (when you add up all the draw
///   and blank periods).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum TimerScale {
  /// Approximately 59.6 nanoseconds
  #[default]
  _1 = 0,
  /// Approximately 3.815 microseconds
  _64 = 1,
  /// Approximately 15.26 microseconds
  ///
  /// **Hint:** With a reload value of 0, this timer scale will overflow
  /// exactly once per second.
  _256 = 2,
  /// Approximately 61.04 microseconds
  ///
  /// **Hint:** With a reload value of `0x4000_u16.wrapping_neg()`,
  /// this timer scale will overflow exactly once per second.
  _1024 = 3,
}

/// Timer configuration bits.
///
/// * `scale` is how many CPU cycles per tick
/// * `cascade` will override the prescale value and instead tick the timer once
///   per overflow of the next lower timer. Timer 0 ignores the cascade bit.
/// * `overflow_irq` will cause an IRQ to be sent each overflow.
/// * `enabled` makes the timer tick.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TimerControl(u16);
impl TimerControl {
  pub_const_fn_new_zeroed!();
  u16_enum_field!(0 - 1: TimerScale, scale, with_scale);
  u16_bool_field!(2, cascade, with_cascade);
  u16_bool_field!(6, overflow_irq, with_overflow_irq);
  u16_bool_field!(7, enabled, with_enabled);
}
