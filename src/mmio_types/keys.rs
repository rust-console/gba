use core::ops::BitOr;
use crate::mmio_addresses::KEYINPUT;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
  A      = 1u16 << 0,
  B      = 1u16 << 1,
  SELECT = 1u16 << 2,
  START  = 1u16 << 3,
  RIGHT  = 1u16 << 4,
  LEFT   = 1u16 << 5,
  UP     = 1u16 << 6,
  DOWN   = 1u16 << 7,
  R      = 1u16 << 8,
  L      = 1u16 << 9,
}

impl BitOr<Key> for Key {
  type Output = u16;
  fn bitor(self, rhs: Key) -> u16 {
    (self as u16) | (rhs as u16)
  }
}

impl BitOr<Key> for u16 {
  type Output = u16;
  fn bitor(self, rhs: Key) -> u16 {
    self | (rhs as u16)
  }
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

  pub fn read() -> Self {
    KEYINPUT.read().into()
  }

  pub fn update(&mut self) {
    *self = Keys::read()
  }

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
  
  pub fn any_pressed(self, key_mask: u16) -> bool {
    self.0 & key_mask != 0
  }

  pub fn pressed(self, key: Key) -> bool {
    self.any_pressed(key as u16)
  }

  pub fn released(self, key: Key) -> bool {
    self.0 & (key as u16) == 0
  }
}

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KeyMonitor {
  current: Keys,
  previous: Keys,
}

impl KeyMonitor {
  pub fn new() -> KeyMonitor {
    KeyMonitor {
      current: Keys::read(),
      previous: Keys::default(),
    }
  }

  pub fn update(&mut self) {
    self.previous = self.current;
    self.current = Keys::read();
  }

  pub fn is_pressed(&self, key: Key) -> bool {
    self.current.pressed(key)
  }

  pub fn was_pressed(&self, key: Key) -> bool {
    self.previous.pressed(key)
  }

  pub fn is_released(&self, key: Key) -> bool {
    self.current.released(key)
  }

  pub fn was_released(&self, key: Key) -> bool {
    self.previous.released(key)
  }

  pub fn just_pressed(&self, key: Key) -> bool {
    self.current.pressed(key) && self.previous.released(key)
  }

  pub fn just_released(&self, key: Key) -> bool {
    self.current.released(key) && self.previous.pressed(key)
  }

  pub fn being_held(&self, key: Key) -> bool {
    self.current.pressed(key) && self.previous.pressed(key)
  }
}