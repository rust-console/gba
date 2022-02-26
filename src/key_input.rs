use voladdress::{Safe, VolAddress};

use crate::macros::{const_new, u16_bool_field};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeysLowActive(u16);
pub const KEYINPUT: VolAddress<KeysLowActive, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0130) };

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
#[inline]
#[must_use]
pub fn get_keys() -> Keys {
  KEYINPUT.read().into()
}
