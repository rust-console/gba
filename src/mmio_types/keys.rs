use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeysLowActive(u16);
impl KeysLowActive {
  const_new!();
  bitfield_bool!(u16; 0, a_released, with_a_released, set_a_released);
  bitfield_bool!(u16; 1, b_released, with_b_released, set_b_released);
  bitfield_bool!(u16; 2, select_released, with_select_released, set_select_released);
  bitfield_bool!(u16; 3, start_released, with_start_released, set_start_released);
  bitfield_bool!(u16; 4, right_released, with_right_released, set_right_released);
  bitfield_bool!(u16; 5, left_released, with_left_released, set_left_released);
  bitfield_bool!(u16; 6, up_released, with_up_released, set_up_released);
  bitfield_bool!(u16; 7, down_released, with_down_released, set_down_released);
  bitfield_bool!(u16; 8, r_released, with_r_released, set_r_released);
  bitfield_bool!(u16; 9, l_released, with_l_released, set_l_released);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Keys(u16);
impl Keys {
  const_new!();
  bitfield_bool!(u16; 0, a, with_a, set_a);
  bitfield_bool!(u16; 1, b, with_b, set_b);
  bitfield_bool!(u16; 2, select, with_select, set_select);
  bitfield_bool!(u16; 3, start, with_start, set_start);
  bitfield_bool!(u16; 4, right, with_right, set_right);
  bitfield_bool!(u16; 5, left, with_left, set_left);
  bitfield_bool!(u16; 6, up, with_up, set_up);
  bitfield_bool!(u16; 7, down, with_down, set_down);
  bitfield_bool!(u16; 8, r, with_r, set_r);
  bitfield_bool!(u16; 9, l, with_l, set_l);

  pub const fn x_signum(self) -> i32 {
    if self.right() {
      1
    } else if self.left() {
      -1
    } else {
      0
    }
  }

  pub const fn y_signum(self) -> i32 {
    if self.down() {
      1
    } else if self.up() {
      -1
    } else {
      0
    }
  }
}
// TODO: bit ops for keys

impl From<KeysLowActive> for Keys {
  fn from(low_active: KeysLowActive) -> Self {
    Self(low_active.0 ^ 0b11_1111_1111)
  }
}

impl From<Keys> for KeysLowActive {
  fn from(keys: Keys) -> Self {
    Self(keys.0 ^ 0b11_1111_1111)
  }
}
