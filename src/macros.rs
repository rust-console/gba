#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! on_gba_or_unimplemented {
  ($($token_tree:tt)*) => {
    #[cfg(feature="on_gba")]
    {
      $($token_tree)*
    }
    #[cfg(not(feature="on_gba"))]
    unimplemented!()
  }
}
pub(crate) use on_gba_or_unimplemented;

macro_rules! pub_const_fn_new_zeroed {
  () => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn new() -> Self {
      Self(0)
    }
  };
}
pub(crate) use pub_const_fn_new_zeroed;

macro_rules! u16_bool_field {
  ($bit:expr, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> bool {
      bitfrob::u16_get_bit($bit, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, b: bool) -> Self {
      Self(bitfrob::u16_with_bit($bit, self.0, b))
    }
  };
  (inverted $bit:expr, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> bool {
      !bitfrob::u16_get_bit($bit, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, b: bool) -> Self {
      Self(bitfrob::u16_with_bit($bit, self.0, !b))
    }
  };
}
pub(crate) use u16_bool_field;

macro_rules! u16_enum_field {
  ($low:literal - $high:literal : $t:ty, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> $t {
      unsafe {
        core::mem::transmute::<u16, $t>(bitfrob::u16_get_region(
          $low, $high, self.0,
        ))
      }
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: $t) -> Self {
      Self(bitfrob::u16_with_region($low, $high, self.0, val as u16))
    }
  };
}
pub(crate) use u16_enum_field;

macro_rules! u16_int_field {
  ($low:literal - $high:literal, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> u16 {
      bitfrob::u16_get_value($low, $high, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: u16) -> Self {
      Self(bitfrob::u16_with_value($low, $high, self.0, val))
    }
  };
}
pub(crate) use u16_int_field;

macro_rules! u8_bool_field {
  ($bit:expr, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> bool {
      bitfrob::u8_get_bit($bit, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, b: bool) -> Self {
      Self(bitfrob::u8_with_bit($bit, self.0, b))
    }
  };
}
pub(crate) use u8_bool_field;

macro_rules! u8_enum_field {
  ($low:literal - $high:literal : $t:ty, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> $t {
      unsafe {
        core::mem::transmute::<u8, $t>(bitfrob::u8_get_region(
          $low, $high, self.0,
        ))
      }
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: $t) -> Self {
      Self(bitfrob::u8_with_region($low, $high, self.0, val as u8))
    }
  };
}
pub(crate) use u8_enum_field;

macro_rules! u8_int_field {
  ($low:literal - $high:literal, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> u8 {
      bitfrob::u8_get_value($low, $high, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: u8) -> Self {
      Self(bitfrob::u8_with_value($low, $high, self.0, val))
    }
  };
}
pub(crate) use u8_int_field;
