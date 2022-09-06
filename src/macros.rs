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
  ($bit:expr, $name: ident) => {
    ::paste::paste!(u16_bool_field!($bit, $name, [<with_ $name>]););
  };
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
}
pub(crate) use u16_bool_field;

macro_rules! u16_enum_field {
  ($low:literal - $high:literal : $t:ty, $name:ident) => {
    ::paste::paste!(u16_enum_field!($low - $high : $t, $name, [<with_ $name>]););
  };
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
  ($low:literal - $high:literal, $name:ident) => {
    ::paste::paste!(u16_int_field!($low - $high, $name, [<with_ $name>]););
  };
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
