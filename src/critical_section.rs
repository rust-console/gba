//! Support for the [critical-section](https://docs.rs/critical-section) crate.

use critical_section::{set_impl, Impl, RawRestoreState};

use crate::mmio::IME;

struct GbaCriticalSection;
set_impl!(GbaCriticalSection);

#[cfg(feature = "on_gba")]
unsafe impl Impl for GbaCriticalSection {
  /// # Safety
  /// This function has no pre-conditions.
  unsafe fn acquire() -> RawRestoreState {
    let restore = IME.read();
    IME.write(false);
    restore
  }

  /// # Safety
  /// This function has no pre-conditions.
  unsafe fn release(restore: RawRestoreState) {
    IME.write(restore);
  }
}

#[cfg(not(feature = "on_gba"))]
unsafe impl Impl for GbaCriticalSection {
  /// # Safety
  /// This function will always panic, so I guess you could say it's always
  /// safe.
  unsafe fn acquire() -> RawRestoreState {
    panic!()
  }

  /// # Safety
  /// This function will always panic, so I guess you could say it's always
  /// safe.
  unsafe fn release(restore: RawRestoreState) {
    panic!()
  }
}
