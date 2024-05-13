//! Support for the [critical-section](https://docs.rs/critical-section) crate.

use critical_section::{set_impl, Impl, RawRestoreState};

use crate::mmio::IME;

struct GbaCriticalSection;
#[cfg(feature = "on_gba")]
set_impl!(GbaCriticalSection);

#[cfg(feature = "on_gba")]
unsafe impl Impl for GbaCriticalSection {
  /// ## Safety
  /// * This function has no pre-conditions.
  /// * This uses `IME` to disable interrupts. Technically there's a 2 CPU cycle
  ///   delay between `IME` being disabled and interrupts actually being unable
  ///   to run. This function is marked `inline(never)`, so just the time it
  ///   takes the CPU to return from the function should be enough to prevent
  ///   any problems. Technically that's "only a hint" though. Even then, any
  ///   code running in ROM will generally be slow enough for is to not matter
  ///   just because of ROM access speeds. If your code is running in IWRAM and
  ///   you wanted to be absolutely paranoid you could insert two calls to
  ///   [`nop`][crate::asm_runtime::nop] after calling this function.
  ///   Personally, even then I wouldn't bother.
  #[inline(never)]
  unsafe fn acquire() -> RawRestoreState {
    let restore = IME.read();
    IME.write(false);
    restore
  }

  /// ## Safety
  /// * This function has no pre-conditions.
  #[inline]
  unsafe fn release(restore: RawRestoreState) {
    IME.write(restore);
  }
}

#[cfg(not(feature = "on_gba"))]
unsafe impl Impl for GbaCriticalSection {
  /// ## Safety
  /// * This function will always panic.
  #[track_caller]
  unsafe fn acquire() -> RawRestoreState {
    unimplemented!()
  }

  /// ## Safety
  /// * This function will always panic.
  #[track_caller]
  unsafe fn release(restore: RawRestoreState) {
    unimplemented!()
  }
}
