//! Various panic handler functions that you might find useful.

/// Declares one of the functions in the
/// [`panic_handlers`](crate::panic_handlers) module to be the handler for your
/// program.
///
/// Valid inputs are the name of any of the functions in that module:
/// * [`empty_loop`][crate::panic_handlers::empty_loop]
///
/// There's no special magic here, it just saves you on typing it all out
/// yourself.
#[macro_export]
macro_rules! panic_handler {
  ($i:ident) => {
    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! {
      $crate::panic_handlers::$i(info)
    }
  };
}

/// Just performs an empty `loop`
#[inline]
pub fn empty_loop(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
