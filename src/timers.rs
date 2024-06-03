//! Timer related data types.

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
  pub const fn with_cpus_per_tick(self, cpus: CpusPerTick) -> Self {
    Self(u8_with_region(0, 1, self.0, cpus as u8))
  }
  /// If the timer should *only* tick when the lower-number timer overflows.
  ///
  /// * When set, this **overrides** the `cpus_per_tick` value and Timer N will
  ///   instead tick once per overflow of Timer (N-1).
  /// * This has no effect for Timer 0, since it has no lower numbered timer.
  #[inline]
  pub const fn cascade_ticks(self, cascade: bool) -> Self {
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
