//! Various panic handler functions that you might find useful.

/// Just performs an empty `loop`
#[inline]
pub fn empty_loop(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
