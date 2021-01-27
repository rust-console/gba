#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![allow(clippy::cast_lossless)]
#![deny(clippy::float_arithmetic)]
#![warn(missing_docs)]

//! This crate helps you write GBA ROMs.
//!
//! ## SAFETY POLICY
//!
//! Some parts of this crate are safe wrappers around unsafe operations. This is
//! good, and what you'd expect from a Rust crate.
//!
//! However, the safe wrappers all assume that you will _only_ attempt to
//! execute this crate on a GBA or in a GBA Emulator.
//!
//! **Do not** use this crate in programs that aren't running on the GBA. If you
//! do, it's a giant bag of Undefined Behavior.

use core::{cell::Cell, fmt, cmp::Ordering};

pub(crate) use gba_proc_macro::phantom_fields;
pub(crate) use voladdress::{read_only::ROVolAddress, VolAddress, VolBlock};

pub mod macros;

pub mod base;

pub mod bios;

pub mod iwram;

pub mod ewram;

pub mod io;

pub mod palram;

pub mod vram;

pub mod oam;

pub mod rom;

pub mod sram;

pub mod mgba;

extern "C" {
  /// This marks the end of the `.data` and `.bss` sections in IWRAM.
  ///
  /// Memory in IWRAM _before_ this location is not free to use, you'll trash
  /// your globals and stuff. Memory here or after is freely available for use
  /// (careful that you don't run into your own stack of course).
  ///
  /// The actual value is unimportant, you just want to use the _address of_
  /// this location as the start of your IWRAM usage.
  pub static __bss_end: u8;
}

newtype! {
  /// A color on the GBA is an RGB 5.5.5 within a `u16`
  #[derive(PartialOrd, Ord, Hash)]
  Color, pub u16
}

impl Color {
  /// Constructs a color from the channel values provided (should be 0..=31).
  ///
  /// No actual checks are performed, so illegal channel values can overflow
  /// into each other and produce an unintended color.
  pub const fn from_rgb(r: u16, g: u16, b: u16) -> Color {
    Color(b << 10 | g << 5 | r)
  }
}

//
// After here is totally unsorted nonsense
//

/// Performs unsigned divide and remainder, gives None if dividing by 0.
pub fn divrem_u32(numer: u32, denom: u32) -> Option<(u32, u32)> {
  // TODO: const this? Requires const if
  if denom == 0 {
    None
  } else {
    Some(unsafe { divrem_u32_unchecked(numer, denom) })
  }
}

/// Performs divide and remainder, no check for 0 division.
///
/// # Safety
///
/// If you call this with a denominator of 0 the result is implementation
/// defined (not literal UB) including but not limited to: an infinite loop,
/// panic on overflow, or incorrect output.
pub unsafe fn divrem_u32_unchecked(numer: u32, denom: u32) -> (u32, u32) {
  // TODO: const this? Requires const if
  if (numer >> 5) < denom {
    divrem_u32_simple(numer, denom)
  } else {
    divrem_u32_non_restoring(numer, denom)
  }
}

/// The simplest form of division. If N is too much larger than D this will be
/// extremely slow. If N is close enough to D then it will likely be faster than
/// the non_restoring form.
fn divrem_u32_simple(mut numer: u32, denom: u32) -> (u32, u32) {
  // TODO: const this? Requires const if
  let mut quot = 0;
  while numer >= denom {
    numer -= denom;
    quot += 1;
  }
  (quot, numer)
}

/// Takes a fixed quantity of time based on the bit width of the number (in this
/// case 32).
fn divrem_u32_non_restoring(numer: u32, denom: u32) -> (u32, u32) {
  // TODO: const this? Requires const if
  let mut r: i64 = numer as i64;
  let d: i64 = (denom as i64) << 32;
  let mut q: u32 = 0;
  let mut i = 1 << 31;
  while i > 0 {
    if r >= 0 {
      q |= i;
      r = 2 * r - d;
    } else {
      r = 2 * r + d;
    }
    i >>= 1;
  }
  q -= !q;
  if r < 0 {
    q -= 1;
    r += d;
  }
  r >>= 32;
  // TODO: remove this once we've done more checks here.
  debug_assert!(r >= 0);
  debug_assert!(r <= core::u32::MAX as i64);
  (q, r as u32)
}

/// Performs signed divide and remainder, gives None if dividing by 0 or
/// computing `MIN/-1`
pub fn divrem_i32(numer: i32, denom: i32) -> Option<(i32, i32)> {
  if denom == 0 || (numer == core::i32::MIN && denom == -1) {
    None
  } else {
    Some(unsafe { divrem_i32_unchecked(numer, denom) })
  }
}

/// Performs signed divide and remainder, no check for 0 division or `MIN/-1`.
///
/// # Safety
///
/// * If you call this with a denominator of 0 the result is implementation
///   defined (not literal UB) including but not limited to: an infinite loop,
///   panic on overflow, or incorrect output.
/// * If you call this with `MIN/-1` you'll get a panic in debug or just `MIN`
///   in release (which is incorrect), because of how twos-compliment works.
pub unsafe fn divrem_i32_unchecked(numer: i32, denom: i32) -> (i32, i32) {
  // TODO: const this? Requires const if
  let unsigned_numer = numer.abs() as u32;
  let unsigned_denom = denom.abs() as u32;
  let opposite_sign = (numer ^ denom) < 0;
  let (udiv, urem) = if (numer >> 5) < denom {
    divrem_u32_simple(unsigned_numer, unsigned_denom)
  } else {
    divrem_u32_non_restoring(unsigned_numer, unsigned_denom)
  };
  match (opposite_sign, numer < 0) {
    (true, true) => (-(udiv as i32), -(urem as i32)),
    (true, false) => (-(udiv as i32), urem as i32),
    (false, true) => (udiv as i32, -(urem as i32)),
    (false, false) => (udiv as i32, urem as i32),
  }
}

/// A memory safe type to allow for global mutable static variables.
/// 
/// The way it achieves memory safety is similar to [`Cell<T>`]:
/// It never allows you to transform a `&Static<T>` to a `&T`. That way
/// you can never have reference invalidation because you can only ever get
/// a reference to the inner value if you have exclusive access to the `Static<T>`.
/// Additionally, because the GBA doesn't have hardware threads, data races are
/// impossible, so we can safely implement [`Sync`](core::sync::Sync) for this type, whereas
/// [`Cell<T>`] cannot.
///
/// Under the hood this type uses [`Cell<T>`] so it can't have implementation errors
/// that lead to unsoundness (unless the core library implementation is wrong).
///
/// # Examples
///
/// ```ignore
/// static IRQ_COUNTER: Static<usize> = Static::new(0);
/// 
/// extern "C" fn irq_handler(_: IrqFlags) {
///   IRQ_COUNTER.set(COUNTER.get() + 1);   
/// }
///```
/// 
/// This example just counts the number of interrupt requests which normally
/// you wouldn't be able to without using `unsafe` code.
/// 
/// [`Cell<T>`]: core::cell::Cell
#[repr(transparent)]
pub struct Static<T: ?Sized> {
  inner: Cell<T>,
}

impl<T> Static<T> {
  /// Constructs a new `Static<T>` with a given value.
  pub const fn new(value: T) -> Self {
    Self { inner: Cell::new(value) }
  }

  /// Replaces the current value with the given one.
  pub fn set(&self, value: T) {
    self.inner.set(value);
  }

  /// Swaps the two inner values. The advantage over simply using
  /// [`core::mem::swap`] is that you don't need exclusive access to the values.
  pub fn swap(&self, other: &Static<T>) {
    self.inner.swap(&other.inner);
  }

  /// Replaces the current value with the given one and returns it.
  pub fn replace(&self, value: T) -> T {
    self.inner.replace(value)
  }

  /// Consumes the `Static<T>` and returns the current inner value.
  pub fn into_inner(self) -> T {
    self.inner.into_inner()
  }
}

impl<T: Copy> Static<T> {
  /// Returns a copy of the current inner value.
  pub fn get(&self) -> T {
    self.inner.get()
  }

  /// Updates the inner value using the given function
  /// and returns the new inner value.
  pub fn update<F: FnOnce(T) -> T>(&self, f: F) -> T {
    let old = self.get();
    let new = f(old);
    self.set(new);
    new
  }
}

impl<T: ?Sized> Static<T> {
  /// Returns a pointer to the inner value.
  pub const fn as_ptr(&self) -> *mut T {
    self.inner.as_ptr()
  }

  /// Returns a mutable reference to the inner value.
  /// This is safe, as it requires exclusive access to the `Static<T>`,
  /// so noone else can set the inner value which could lead
  /// to reference invalidation
  pub fn get_mut(&mut self) -> &mut T {
    self.inner.get_mut()
  }

  /// Returns a reference to `Static<T>` from a `&mut T`.
  /// This is safe because during the lifetime of the `&Static<T>`
  /// it has exclusive access to the `&mut T`
  pub fn from_mut(t: &mut T) -> &Static<T> {
    unsafe { &*(t as *mut T as *const Static<T>) }
  }
}

impl<T: Default> Static<T> {
  /// Returns the inner value and replaces it with [`<T as Default>::default()`](core::default::Default::default).
  pub fn take(&self) -> T {
    self.replace(Default::default())
  }
}

impl<T: fmt::Debug + Copy> fmt::Debug for Static<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("Static").field(&self.get()).finish()
  }
}

impl<T: Default> Default for Static<T> {
  fn default() -> Self {
    Self::new(Default::default())
  }
}

impl<T: PartialEq + Copy> PartialEq for Static<T> {
  fn eq(&self, other: &Self) -> bool {
    self.get().eq(&other.get())
  }

  fn ne(&self, other: &Self) -> bool {
    self.get().ne(&other.get())
  }
}

impl<T: Eq + Copy> Eq for Static<T> {
  fn assert_receiver_is_total_eq(&self) {}
}

impl<T> From<T> for Static<T> {
  fn from(value: T) -> Self {
    Self::new(value)
  }
}

impl<T: PartialOrd + Copy> PartialOrd for Static<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.get().partial_cmp(&other.get())
  }

  fn lt(&self, other: &Self) -> bool {
    self.get().lt(&other.get())
  }
  
  fn le(&self, other: &Self) -> bool {
    self.get().le(&other.get())
  }
  
  fn gt(&self, other: &Self) -> bool {
    self.get().gt(&other.get())
  }
  
  fn ge(&self, other: &Self) -> bool {
    self.get().ge(&other.get())
  }
}

// SAFETY: Because the GBA doesn't have hardware threads, data races are impossible, so this implementation is sound
// (Note that it's only sound if it's actually run on a GBA)
unsafe impl<T> Sync for Static<T> {}

/*
#[cfg(test)]
mod tests {
  use super::*;
  use quickcheck::quickcheck;

  // We have an explicit property on the non_restoring division
  quickcheck! {
    fn divrem_u32_non_restoring_prop(num: u32, denom: u32) -> bool {
      if denom > 0 {
        divrem_u32_non_restoring(num, denom) == (num / denom, num % denom)
      } else {
        true
      }
    }
  }

  // We have an explicit property on the simple division
  quickcheck! {
    fn divrem_u32_simple_prop(num: u32, denom: u32) -> bool {
      if denom > 0 {
        divrem_u32_simple(num, denom) == (num / denom, num % denom)
      } else {
        true
      }
    }
  }

  // Test the u32 wrapper
  quickcheck! {
    fn divrem_u32_prop(num: u32, denom: u32) -> bool {
      if denom > 0 {
        divrem_u32(num, denom).unwrap() == (num / denom, num % denom)
      } else {
        divrem_u32(num, denom).is_none()
      }
    }
  }

  // test the i32 wrapper
  quickcheck! {
    fn divrem_i32_prop(num: i32, denom: i32) -> bool {
      if denom == 0 || num == core::i32::MIN && denom == -1 {
        divrem_i32(num, denom).is_none()
      } else {
        divrem_i32(num, denom).unwrap() == (num / denom, num % denom)
      }
    }
  }
}
*/
