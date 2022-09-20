use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field};

/// Key input data.
///
/// Internally this uses a "low-active" convention: A bit is 0 when the key is
/// *pressed*, and 1 when a key is *released*. The accessor methods handle this
/// automatically, you only need to consider this fact if you want to use the
/// raw bit pattern for something (eg: as a randomness source).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeyInput(u16);
impl KeyInput {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(inverted 0, a, with_a);
  u16_bool_field!(inverted 1, b, with_b);
  u16_bool_field!(inverted 2, select, with_select);
  u16_bool_field!(inverted 3, start, with_start);
  u16_bool_field!(inverted 4, right, with_right);
  u16_bool_field!(inverted 5, left, with_left);
  u16_bool_field!(inverted 6, up, with_up);
  u16_bool_field!(inverted 7, down, with_down);
  u16_bool_field!(inverted 8, r, with_r);
  u16_bool_field!(inverted 9, l, with_l);

  #[inline]
  #[must_use]
  pub const fn to_u16(self) -> u16 {
    self.0
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeyControl(u16);
impl KeyControl {
  pub_const_fn_new_zeroed!();
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

  u16_bool_field!(14, irq_enabled, with_irq_enabled);
  u16_bool_field!(15, irq_all, with_irq_all);

  #[inline]
  #[must_use]
  pub const fn to_u16(self) -> u16 {
    self.0
  }
}
