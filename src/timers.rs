//! Timer related data types.

use super::*;
use bitfrob::{u8_with_bit, u8_with_region};

/// Control bits for one of the GBA's four timers.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct TimerControl(u8);
impl TimerControl {
  /// A new, zeroed value.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// The number of CPU cycles per timer tick.
  #[inline]
  pub const fn with_cycles_per_tick(self, cpus: CpusPerTick) -> Self {
    Self(u8_with_region(0, 1, self.0, cpus as u8))
  }
  /// If the timer should *only* tick when the lower-number timer overflows.
  ///
  /// * When set, this **overrides** the `cpus_per_tick` value and Timer N will
  ///   instead tick once per overflow of Timer (N-1).
  /// * This has no effect for Timer 0, since it has no lower numbered timer.
  #[inline]
  pub const fn with_cascade_ticks(self, cascade: bool) -> Self {
    Self(u8_with_bit(2, self.0, cascade))
  }
  /// If an overflow of this timer should send an interrupt.
  #[inline]
  pub const fn with_send_irq(self, irq: bool) -> Self {
    Self(u8_with_bit(6, self.0, irq))
  }
  /// If this timer is enabled.
  #[inline]
  pub const fn with_enabled(self, enabled: bool) -> Self {
    Self(u8_with_bit(7, self.0, enabled))
  }
}

/// How many CPU cycles per timer tick.
#[repr(u8)]
#[allow(missing_docs)]
pub enum CpusPerTick {
  _1 = 0,
  _64 = 1,
  _256 = 2,
  _1024 = 3,
}

/// Timer0's current counter value.
pub const TIMER0_COUNTER: RoAddr<u16> = unsafe { VolAddress::new(0x0400_0100) };
/// Timer1's current counter value.
pub const TIMER1_COUNTER: RoAddr<u16> = unsafe { VolAddress::new(0x0400_0104) };
/// Timer2's current counter value.
pub const TIMER2_COUNTER: RoAddr<u16> = unsafe { VolAddress::new(0x0400_0108) };
/// Timer3's current counter value.
pub const TIMER3_COUNTER: RoAddr<u16> = unsafe { VolAddress::new(0x0400_010C) };

/// The value for Timer0 to reload on overflow on when the `start` bit is newly
/// set.
pub const TIMER0_RELOAD: WoAddr<u16> = unsafe { VolAddress::new(0x0400_0100) };
/// The value for Timer1 to reload on overflow on when the `start` bit is newly
/// set.
pub const TIMER1_RELOAD: WoAddr<u16> = unsafe { VolAddress::new(0x0400_0104) };
/// The value for Timer2 to reload on overflow on when the `start` bit is newly
/// set.
pub const TIMER2_RELOAD: WoAddr<u16> = unsafe { VolAddress::new(0x0400_0108) };
/// The value for Timer3 to reload on overflow on when the `start` bit is newly
/// set.
pub const TIMER3_RELOAD: WoAddr<u16> = unsafe { VolAddress::new(0x0400_010C) };

/// Control bits for Timer 0.
pub const TIMER0_CONTROL: PlainAddr<TimerControl> =
  unsafe { VolAddress::new(0x0400_0102) };

/// Control bits for Timer 1.
pub const TIMER1_CONTROL: PlainAddr<TimerControl> =
  unsafe { VolAddress::new(0x0400_0106) };

/// Control bits for Timer 2.
pub const TIMER2_CONTROL: PlainAddr<TimerControl> =
  unsafe { VolAddress::new(0x0400_010A) };

/// Control bits for Timer 3.
pub const TIMER3_CONTROL: PlainAddr<TimerControl> =
  unsafe { VolAddress::new(0x0400_010E) };
