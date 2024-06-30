//! Keypad (button) reading.

use super::*;

/// Keypad input state.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct KeyInput(pub u16);
impl KeyInput {
  /// If `a` is pressed (left primary button)
  #[inline]
  #[must_use]
  pub const fn a(self) -> bool {
    !bitfrob::u16_get_bit(0, self.0)
  }
  /// If `b` is pressed (right primary button)
  #[inline]
  #[must_use]
  pub const fn b(self) -> bool {
    !bitfrob::u16_get_bit(1, self.0)
  }
  /// If `select` is pressed (lower/left secondary button)
  #[inline]
  #[must_use]
  pub const fn select(self) -> bool {
    !bitfrob::u16_get_bit(2, self.0)
  }
  /// If `start` is pressed (upper/right secondary button)
  #[inline]
  #[must_use]
  pub const fn start(self) -> bool {
    !bitfrob::u16_get_bit(3, self.0)
  }
  /// If `right` is pressed (d-pad)
  #[inline]
  #[must_use]
  pub const fn right(self) -> bool {
    !bitfrob::u16_get_bit(4, self.0)
  }
  /// If `left` is pressed (d-pad)
  #[inline]
  #[must_use]
  pub const fn left(self) -> bool {
    !bitfrob::u16_get_bit(5, self.0)
  }
  /// If `up` is pressed (d-pad)
  #[inline]
  #[must_use]
  pub const fn up(self) -> bool {
    !bitfrob::u16_get_bit(6, self.0)
  }
  /// If `down` is pressed (d-pad)
  #[inline]
  #[must_use]
  pub const fn down(self) -> bool {
    !bitfrob::u16_get_bit(7, self.0)
  }
  /// If `r` is pressed (right shoulder button)
  #[inline]
  #[must_use]
  pub const fn r(self) -> bool {
    !bitfrob::u16_get_bit(8, self.0)
  }
  /// If `l` is pressed (left shoulder button)
  #[inline]
  #[must_use]
  pub const fn l(self) -> bool {
    !bitfrob::u16_get_bit(9, self.0)
  }
  /// Delta X of the d-pad. right +1, left -1.
  #[inline]
  #[must_use]
  pub const fn dx(self) -> i8 {
    if self.right() {
      1
    } else if self.left() {
      -1
    } else {
      0
    }
  }
  /// Delta Y of the d-pad. up +1, down -1.
  #[inline]
  #[must_use]
  pub const fn dy(self) -> i8 {
    if self.up() {
      1
    } else if self.down() {
      -1
    } else {
      0
    }
  }
}

/// Key Input (read-only).
///
/// Gives the low-active button state of all system buttons.
pub const KEYINPUT: RoAddr<KeyInput> = unsafe { VolAddress::new(0x0400_0130) };
