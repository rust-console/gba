#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! const_new {
  () => {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
      Self(0)
    }
  };
}
pub(crate) use const_new;

macro_rules! u16_value_field {
  ($bit_start:literal - $bit_end:literal, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    pub const fn $get(self) -> u16 {
      $crate::u16_get_value::<$bit_start, $bit_end>(self.0)
    }
    #[inline]
    #[must_use]
    pub const fn $with(self, u: u16) -> Self {
      Self($crate::u16_with_value::<$bit_start, $bit_end>(self.0, u))
    }
  };
}
pub(crate) use u16_value_field;

macro_rules! u16_enum_field {
  ($bit_start:literal - $bit_end:literal : $enum_ty:ident, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    pub const fn $get(self) -> $enum_ty {
      unsafe {
        core::mem::transmute($crate::u16_get_region::<$bit_start, $bit_end>(
          self.0,
        ))
      }
    }
    #[inline]
    #[must_use]
    pub const fn $with(self, u: $enum_ty) -> Self {
      Self($crate::u16_with_region::<$bit_start, $bit_end>(self.0, unsafe {
        core::mem::transmute(u)
      }))
    }
  };
}
pub(crate) use u16_enum_field;

macro_rules! u16_bool_field {
  ($bit:literal, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    pub const fn $get(self) -> bool {
      $crate::u16_get_bit::<$bit>(self.0)
    }
    #[inline]
    #[must_use]
    pub const fn $with(self, b: bool) -> Self {
      Self($crate::u16_with_bit::<$bit>(self.0, b))
    }
  };
}
pub(crate) use u16_bool_field;
