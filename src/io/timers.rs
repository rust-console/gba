//! Module for timers.
//!
//! The timers are slightly funny in that reading and writing from them works
//! somewhat differently than with basically any other part of memory.
//!
//! When you read a timer's counter you read the current value.
//!
//! When you write a timer's counter you write _the counter's reload value_.
//! This is used whenever you enable the timer or any time the timer overflows.
//! You cannot set a timer to a given counter value, but you can set a timer to
//! start at some particular value every time it reloads.
//!
//! The timer counters are `u16`, so if you want to set them to run for a
//! certain number of ticks before overflow you would write something like
//!
//! ```rust
//! let init_val: u16 = u32::wrapping_sub(0x1_0000, ticks) as u16;
//! ```
//!
//! A timer reloads any time it overflows _or_ goes from disabled to enabled. If
//! you want to "pause" a timer _without_ making it reload when resumed then you
//! should not disable it. Instead, you should set its `TimerTickRate` to
//! `Cascade` and disable _the next lower timer_ so that it won't overflow into
//! the timer you have on hold.

use super::*;

// TODO: striding blocks?

/// Timer 0 Counter/Reload. Special (see module).
pub const TM0CNT_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0100) };

/// Timer 1 Counter/Reload. Special (see module).
pub const TM1CNT_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0104) };

/// Timer 2 Counter/Reload. Special (see module).
pub const TM2CNT_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0108) };

/// Timer 3 Counter/Reload. Special (see module).
pub const TM3CNT_L: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_010C) };

/// Timer 0 Control. Read/Write.
pub const TM0CNT_H: VolAddress<TimerControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0102) };

/// Timer 1 Control. Read/Write.
pub const TM1CNT_H: VolAddress<TimerControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0106) };

/// Timer 2 Control. Read/Write.
pub const TM2CNT_H: VolAddress<TimerControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_010A) };

/// Timer 3 Control. Read/Write.
pub const TM3CNT_H: VolAddress<TimerControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_010E) };

newtype! {
  /// Allows control of a timer unit.
  ///
  /// * Bits 0-2: How often the timer should tick up one unit. You can either
  ///   specify a number of CPU cycles or "cascade" mode, where there's a single
  ///   tick per overflow of the next lower timer. For example, Timer 1 would
  ///   tick up once per overflow of Timer 0 if it were in cascade mode. Cascade
  ///   mode naturally does nothing when used with Timer 0.
  /// * Bit 6: Raise a timer interrupt upon overflow.
  /// * Bit 7: Enable the timer.
  TimerControlSetting, u16
}
impl TimerControlSetting {
  phantom_fields! {
    self.0: u16,
    tick_rate: 0-2=TimerTickRate<CPU1, CPU64, CPU256, CPU1024, Cascade>,
    overflow_irq: 6,
    enabled: 7,
  }
}

/// Controls how often an enabled timer ticks upward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum TimerTickRate {
  /// Once every CPU cycle
  CPU1 = 0,
  /// Once per 64 CPU cycles
  CPU64 = 1,
  /// Once per 256 CPU cycles
  CPU256 = 2,
  /// Once per 1,024 CPU cycles
  CPU1024 = 3,
  /// Once per overflow of the next lower timer. (Useless with Timer 0)
  Cascade = 4,
}
