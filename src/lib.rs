#![no_std]
#![feature(isa_attribute)]

//! This crate helps you write GBA ROMs.
//!
//! ## Safety
//!
//! This crate takes *minimal* precautions to avoid GBA specific code from being
//! run on a standard desktop by accident by using `#[cfg(target_arch = "arm")]`
//! in appropriate places. However, there are obviously many other ARM devices
//! in the world. If you actually run the GBA specific code on something that
//! isn't a GBA, then that's your fault.
//!
//! ## Docs.rs
//!
//! The docs on docs.rs are generated for the `thumbv6m-none-eabi` target
//! because the docs.rs docker image isn't currently able to use the
//! `-Zbuild-std=core` ability of cargo. Instead, we have it just build using a
//! "close enough" Tier 2 target.
//!
//! When building your actual GBA games you should of course use the
//! `thumbv4t-none-eabi` target.

pub mod prelude {
  pub use crate::mmio_types::*;

  #[cfg(target_arch = "arm")]
  pub use crate::bios::*;
  #[cfg(target_arch = "arm")]
  pub use crate::debugging::*;
  #[cfg(target_arch = "arm")]
  pub use crate::mmio_addresses::*;
  #[cfg(target_arch = "arm")]
  pub use crate::random::*;
  #[cfg(target_arch = "arm")]
  pub use crate::save::*;
  #[cfg(target_arch = "arm")]
  pub use crate::sync::*;
}

pub mod mmio_types;

#[cfg(target_arch = "arm")]
pub mod mmio_addresses;

#[cfg(target_arch = "arm")]
pub mod bios;

pub mod art;

#[cfg(target_arch = "arm")]
pub mod macros;

#[cfg(target_arch = "arm")]
pub mod sync;

#[cfg(target_arch = "arm")]
pub mod save;

#[cfg(target_arch = "arm")]
pub mod debugging;

#[cfg(target_arch = "arm")]
pub mod random;
/*
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

TODO: math module for math functions you probably want on the GBA

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
*/
