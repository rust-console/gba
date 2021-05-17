//! A package containing useful utilities for writing save accessors. This is
//! mainly used internally, although the types inside are exposed publicly.

use super::Error;
use crate::{
  prelude::*,
  sync::{RawMutex, RawMutexGuard, Static},
};
use voladdress::*;

/// Internal representation for our active timer.
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum TimerId {
  None,
  T0,
  T1,
  T2,
  T3,
}

/// Stores the timer ID used for timeouts created by save accessors.
static TIMER_ID: Static<TimerId> = Static::new(TimerId::None);

/// Sets the timer to use to implement timeouts for operations that may hang.
///
/// At any point where you call functions in a save accessor, this timer may be
/// reset to a different value.
pub fn set_timer_for_timeout(id: u8) {
  if id >= 4 {
    panic!("Timer ID must be 0-3.");
  } else {
    TIMER_ID.write([TimerId::T0, TimerId::T1, TimerId::T2, TimerId::T3][id as usize])
  }
}

/// Disables the timeout for operations that may hang.
pub fn disable_timeout() {
  TIMER_ID.write(TimerId::None);
}

/// A timeout type used to prevent hardware errors in save media from hanging
/// the game.
pub struct Timeout {
  _lock_guard: RawMutexGuard<'static>,
  active: bool,
  timer_l: VolAddress<u16, Safe, Safe>,
  timer_h: VolAddress<TimerControl, Safe, Safe>,
}
impl Timeout {
  /// Creates a new timeout from the timer passed to [`set_timer_for_timeout`].
  ///
  /// ## Errors
  ///
  /// If another timeout has already been created.
  #[inline(never)]
  pub fn new() -> Result<Self, Error> {
    static TIMEOUT_LOCK: RawMutex = RawMutex::new();
    let _lock_guard = match TIMEOUT_LOCK.try_lock() {
      Some(x) => x,
      None => return Err(Error::MediaInUse),
    };
    let id = TIMER_ID.read();
    Ok(Timeout {
      _lock_guard,
      active: id != TimerId::None,
      timer_l: match id {
        TimerId::None => unsafe { VolAddress::new(0) },
        TimerId::T0 => unsafe { VolAddress::new(TIMER0_COUNTER.as_usize()) },
        TimerId::T1 => unsafe { VolAddress::new(TIMER1_COUNTER.as_usize()) },
        TimerId::T2 => unsafe { VolAddress::new(TIMER2_COUNTER.as_usize()) },
        TimerId::T3 => unsafe { VolAddress::new(TIMER3_COUNTER.as_usize()) },
      },
      timer_h: match id {
        TimerId::None => unsafe { VolAddress::new(0) },
        TimerId::T0 => TIMER0_CONTROL,
        TimerId::T1 => TIMER1_CONTROL,
        TimerId::T2 => TIMER2_CONTROL,
        TimerId::T3 => TIMER3_CONTROL,
      },
    })
  }

  /// Starts this timeout.
  pub fn start(&self) {
    if self.active {
      self.timer_l.write(0);
      let timer_ctl = TimerControl::new().with_prescaler_selection(3).with_enabled(true);
      self.timer_h.write(TimerControl::new());
      self.timer_h.write(timer_ctl);
    }
  }

  /// Returns whether a number of milliseconds has passed since the last call
  /// to [`Timeout::start()`].
  pub fn is_timeout_met(&self, check_ms: u16) -> bool {
    self.active && check_ms * 17 < self.timer_l.read()
  }
}

/// Tries to obtain a lock on the global lock for save operations.
///
/// This is used to prevent problems with stateful save media.
pub fn lock_media() -> Result<RawMutexGuard<'static>, Error> {
  static LOCK: RawMutex = RawMutex::new();
  match LOCK.try_lock() {
    Some(x) => Ok(x),
    None => Err(Error::MediaInUse),
  }
}
