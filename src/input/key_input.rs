use voladdress::{Safe, VolAddress};

use crate::macros::{const_new, u16_bool_field};

/// Holds data for GBA keys using a "low-active" convention.
///
/// If a bit is 0 the key is "pressed", if a bit is 1 the key is "released".
///
/// This is how the hardware exposes the value, though it's often not so easy to
/// work with because usually we want 1 to be the "pressed" value.
///
/// You're expected to convert this into a [Keys] struct, which uses the more
/// usual high-active convention. Or you can call [get_keys], which will do it
/// for you.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeysLowActive(u16);

/// The current state of all the system's keys (buttons).
pub const KEYINPUT: VolAddress<KeysLowActive, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0130) };

/// Holds data for GBA keys using a "high-active" convention.
///
/// If a bit is 1 the key is "pressed", if a bit is 0 the key is "released".
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Keys(u16);
impl Keys {
  const_new!();
  u16_bool_field!(0, a, with_a);
  u16_bool_field!(1, b, with_b);
  u16_bool_field!(2, select, with_select);
  u16_bool_field!(3, start, with_start);
  u16_bool_field!(4, right, with_right);
  u16_bool_field!(5, left, with_left);
  u16_bool_field!(6, up, with_up);
  u16_bool_field!(7, down, with_down);
  u16_bool_field!(8, r, with_r);
  u16_bool_field!(9, l, with_l);
}
impl From<KeysLowActive> for Keys {
  #[inline]
  #[must_use]
  fn from(low: KeysLowActive) -> Self {
    Self(low.0 ^ 0b11_1111_1111)
  }
}
impl From<Keys> for u16 {
  #[inline]
  #[must_use]
  fn from(k: Keys) -> Self {
    k.0
  }
}

/// Reads [`KEYINPUT`] and converts to a [Keys] value.
#[inline]
#[must_use]
pub fn get_keys() -> Keys {
  KEYINPUT.read().into()
}
