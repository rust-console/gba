#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![feature(const_int_wrapping)]
#![feature(min_const_unsafe_fn)]
#![warn(missing_docs)]
#![allow(clippy::cast_lossless)]
#![deny(clippy::float_arithmetic)]

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

/// Assists in defining a newtype wrapper over some base type.
///
/// Note that rustdoc and derives are all the "meta" stuff, so you can write all
/// of your docs and derives in front of your newtype in the same way you would
/// for a normal struct. Then the inner type to be wrapped it name.
///
/// The macro _assumes_ that you'll be using it to wrap zero safe numeric types,
/// so it automatically provides a `const fn` method for `new` that just wraps
/// `0`. If this is not desired you can add `, no frills` to the invocation.
///
/// Example:
/// ```
/// newtype! {
///   /// Records a particular key press combination.
///   #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
///   KeyInput, u16
/// }
/// ```
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $old_name:ident) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
    impl $new_name {
      /// A `const` "zero value" constructor
      pub const fn new() -> Self {
        $new_name(0)
      }
    }
  };
  ($(#[$attr:meta])* $new_name:ident, $old_name:ident, no frills) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}

pub mod base;
pub(crate) use self::base::*;
pub mod bios;
pub mod io;

pub mod video_ram;

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

use gba_proc_macro::register_bit;

/// LCD Control. Read/Write.
///
/// * [gbatek entry](http://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol)
pub const DISPCNT: VolAddress<DisplayControlSetting> = unsafe { VolAddress::new_unchecked(0x400_0000) };

newtype!(
  /// A newtype over the various display control options that you have on a GBA.
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  DisplayControlSetting,
  u16
);

#[allow(missing_docs)]
impl DisplayControlSetting {
  pub const BG_MODE_MASK: u16 = 0b111;

  pub fn mode(self) -> DisplayControlMode {
    match self.0 & Self::BG_MODE_MASK {
      0 => DisplayControlMode::Tiled0,
      1 => DisplayControlMode::Tiled1,
      2 => DisplayControlMode::Tiled2,
      3 => DisplayControlMode::Bitmap3,
      4 => DisplayControlMode::Bitmap4,
      5 => DisplayControlMode::Bitmap5,
      _ => unreachable!(),
    }
  }
  pub fn set_mode(&mut self, new_mode: DisplayControlMode) {
    self.0 &= !Self::BG_MODE_MASK;
    self.0 |= match new_mode {
      DisplayControlMode::Tiled0 => 0,
      DisplayControlMode::Tiled1 => 1,
      DisplayControlMode::Tiled2 => 2,
      DisplayControlMode::Bitmap3 => 3,
      DisplayControlMode::Bitmap4 => 4,
      DisplayControlMode::Bitmap5 => 5,
    };
  }

  register_bit!(CGB_MODE_BIT, u16, 0b1000, cgb_mode);
  register_bit!(PAGE_SELECT_BIT, u16, 0b1_0000, page1_enabled);
  register_bit!(HBLANK_INTERVAL_FREE_BIT, u16, 0b10_0000, hblank_interval_free);
  register_bit!(OBJECT_MEMORY_1D, u16, 0b100_0000, object_memory_1d);
  register_bit!(FORCE_BLANK_BIT, u16, 0b1000_0000, force_blank);
  register_bit!(DISPLAY_BG0_BIT, u16, 0b1_0000_0000, display_bg0);
  register_bit!(DISPLAY_BG1_BIT, u16, 0b10_0000_0000, display_bg1);
  register_bit!(DISPLAY_BG2_BIT, u16, 0b100_0000_0000, display_bg2);
  register_bit!(DISPLAY_BG3_BIT, u16, 0b1000_0000_0000, display_bg3);
  register_bit!(DISPLAY_OBJECT_BIT, u16, 0b1_0000_0000_0000, display_object);
  register_bit!(DISPLAY_WINDOW0_BIT, u16, 0b10_0000_0000_0000, display_window0);
  register_bit!(DISPLAY_WINDOW1_BIT, u16, 0b100_0000_0000_0000, display_window1);
  register_bit!(OBJECT_WINDOW_BIT, u16, 0b1000_0000_0000_0000, display_object_window);
}

/// The six display modes available on the GBA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayControlMode {
  /// This basically allows for the most different things at once (all layers,
  /// 1024 tiles, two palette modes, etc), but you can't do affine
  /// transformations.
  Tiled0,
  /// This is a mix of `Tile0` and `Tile2`: BG0 and BG1 run as if in `Tiled0`,
  /// and BG2 runs as if in `Tiled2`.
  Tiled1,
  /// This allows affine transformations, but only uses BG2 and BG3.
  Tiled2,
  /// This is the basic bitmap draw mode. The whole screen is a single bitmap.
  /// Uses BG2 only.
  Bitmap3,
  /// This uses _paletted color_ so that there's enough space to have two pages
  /// at _full resolution_, allowing page flipping. Uses BG2 only.
  Bitmap4,
  /// This uses _reduced resolution_ so that there's enough space to have two
  /// pages with _full color_, allowing page flipping. Uses BG2 only.
  Bitmap5,
}

/// Assigns the given display control setting.
pub fn set_display_control(setting: DisplayControlSetting) {
  DISPCNT.write(setting);
}
/// Obtains the current display control setting.
pub fn display_control() -> DisplayControlSetting {
  DISPCNT.read()
}

/// Vertical Counter (LY)
pub const VCOUNT: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0006) };

/// Obtains the current VCount value.
pub fn vcount() -> u16 {
  VCOUNT.read()
}

/// Performs a busy loop until VBlank starts.
pub fn wait_until_vblank() {
  // TODO: make this the better version with BIOS and interrupts and such.
  while vcount() < crate::video_ram::SCREEN_HEIGHT as u16 {}
}

/// Performs a busy loop until VDraw starts.
pub fn wait_until_vdraw() {
  // TODO: make this the better version with BIOS and interrupts and such.
  while vcount() >= crate::video_ram::SCREEN_HEIGHT as u16 {}
}
