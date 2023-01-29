//! Module for interfacing with the device's button inputs.
//!
//! The GBA has two primary face buttons (A and B), two secondary face buttons
//! (Select and Start), a 4-way directional pad ("D-pad"), and two shoulder
//! buttons (L and R).
//!
//! To get the state of all the buttons just read from
//! [`KEYINPUT`](crate::mmio::KEYINPUT). For consistency, you should usually
//! read the buttons only once per frame. Then use that same data for all user
//! input considerations across that entire frame. Otherwise, small fluctuations
//! in pressure can cause inconsistencies in the reading during a frame.
//!
//! In addition to simply providing inputs, the buttons can also trigger a
//! hardware interrupt. Set the desired set of buttons that will trigger a key
//! interrupt with [`KEYCNT`](crate::mmio::KEYCNT), and when that button
//! combination is pressed the key interrupt will be fired. Key interrupts
//! aren't a good fit for standard inputs, but as a way to provide a single
//! extra special input it works okay. For example, this is generally how games
//! with a "soft reset" button combination do that. The key interrupt handler
//! sets a "reset requested" flag when the key interrupt occurs, and then the
//! main game loop checks the flag each frame and performs a soft reset instead
//! of the normal game simulation when the flag is set.

use core::ops;
use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field};

/// [`KEYINPUT`](crate::prelude::KEYINPUT): Key input data.
///
/// Each key on the GBA is reprisented by a single bit within this value, so all
/// button state can be captured in the same moment. The `input.getter()` method
/// for each button will return `true` if that button was held down at the time
/// of reading this input data.
///
/// Generally you should read `KEYINPUT` just once per frame, and then use that
/// data for the entire frame's computation. If you read `KEYINPUT` each time
/// you need to check a particular button for input then small variations in
/// button contact can cause confusing frame inputs. For example, a particular
/// button reads as pressed at one part of a frame and then released later
/// during the same frame. Reading the input only once per frame is a simple
/// form of "debouncing" that will help maintain consistency in the program.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeyInput(u16);
impl KeyInput {
  pub const fn new() -> Self {
    Self(0xFFFF)
  }
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

  /// Unwraps the input into its raw `u16` form.
  ///
  /// Internally this type uses a "low-active" convention: A bit is 0 when the
  /// key is *pressed*, and 1 when a key is *released*. The accessor methods
  /// handle this automatically, but when unwrapping the value into a `u16` you
  /// may need to consider this yourself.
  #[inline]
  #[must_use]
  pub const fn to_u16(self) -> u16 {
    self.0
  }
}
impl From<KeyInput> for u16 {
  #[inline]
  #[must_use]
  fn from(value: KeyInput) -> Self {
    value.to_u16()
  }
}
impl From<u16> for KeyInput {
  #[inline]
  #[must_use]
  fn from(value: u16) -> Self {
    Self(value)
  }
}
impl ops::BitAnd for KeyInput {
  type Output = Self;

  fn bitand(self, other: Self) -> Self {
    Self(!(!self.to_u16() & !other.to_u16()))
  }
}
impl ops::BitAndAssign for KeyInput {
  fn bitand_assign(&mut self, other: Self) {
    *self = *self & other;
  }
}
impl ops::BitOr for KeyInput {
  type Output = Self;

  fn bitor(self, other: Self) -> Self {
    Self(!(!self.to_u16() | !other.to_u16()))
  }
}
impl ops::BitOrAssign for KeyInput {
  fn bitor_assign(&mut self, other: Self) {
    *self = *self | other;
  }
}
impl ops::BitXor for KeyInput {
  type Output = Self;

  fn bitxor(self, other: Self) -> Self {
    Self(!(!self.to_u16() ^ !other.to_u16()))
  }
}
impl ops::BitXorAssign for KeyInput {
  fn bitxor_assign(&mut self, other: Self) {
    *self = *self ^ other;
  }
}
impl ops::Not for KeyInput {
  type Output = Self;

  fn not(self) -> Self {
    Self(!self.to_u16())
  }
}

/// [`KEYCNT`](crate::prelude::KEYCNT): Determines when a key interrupt will be
/// sent.
///
/// A key interrupt can be sent if `irq_enabled` is set and the correct buttons
/// are pushed. This *should not* be used for normal user input. For normal
/// input you should just read [`KEYINPUT`](crate::prelude::KEYINPUT). Instead,
/// the key interrupts are primarily used for breaking the CPU out of Halt
/// state.
///
/// * If `irq_all` is `true` then *all* buttons enabled must be pressed at once.
/// * Otherwise *any* buttons enabled can be pressed to trigger the interrupt.
///
/// As with all interrupts, the key interrupt must also be set to be received in
/// the [`IE`](crate::prelude::IE) control, and interrupts must be enabled with
/// the [`IME`](crate::prelude::IME) control, or the key interrupt won't
/// actually be triggered even when `irq_enabled` is set.
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
}
