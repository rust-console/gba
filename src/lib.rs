#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![feature(cfg_target_vendor)]
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
