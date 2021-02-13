//! A module containing functions and utilities useful for synchronizing state.

#![cfg_attr(not(all(target_vendor = "nintendo", target_env = "agb")), allow(unused_variables))]

use crate::io::irq::{IrqEnableSetting, IME};

mod locks;
mod statics;

pub use locks::*;
pub use statics::*;

/// Marks that a given pointer is read by volatile means without actually
/// reading it.
#[inline(always)]
pub fn volatile_mark_ro<T>(val: *const T) {
  unsafe { asm!("/* {0} */", in(reg) val, options(readonly, nostack)) }
}

/// Marks that a given pointer is read or written by volatile means without
/// actually reading or writing it.
#[inline(always)]
pub fn volatile_mark_rw<T>(val: *mut T) {
  unsafe { asm!("/* {0} */", in(reg) val, options(nostack)) }
}

/// An internal function used as a temporary hack to get `compiler_fence`
/// working. While this call is not properly inlined, working is better than
/// not working at all.
///
/// Ideally we should figure out why this causes any code to be emitted at all
/// (probably some problem with our target JSON?) and fix it.
///
/// Not public API, obviously.
#[doc(hidden)]
#[deprecated]
#[allow(dead_code)]
#[no_mangle]
#[inline(always)]
pub unsafe extern "C" fn __sync_synchronize() {}

/// Runs a function with IRQs disabled.
///
/// This should not be done without good reason, as IRQs are usually important
/// for game functionality.
pub fn disable_irqs<T>(mut func: impl FnOnce() -> T) -> T {
  let current_ime = IME.read();
  IME.write(IrqEnableSetting::IRQ_NO);
  // prevents the contents of the function from being reordered before IME is disabled.
  volatile_mark_rw(&mut func);

  let mut result = func();

  // prevents the contents of the function from being reordered after IME is reenabled.
  volatile_mark_rw(&mut result);
  IME.write(current_ime);

  result
}
