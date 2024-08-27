use core::ops::*;

/// `i16` with 8 bits of fixed-point fraction.
///
/// This is used by the affine matrix entries.
///
/// * This build of the docs does not use the `fixed` feature and uses the
///   crate's internal fixed point type.
#[allow(non_camel_case_types)]
#[cfg(not(feature = "fixed"))]
pub type i16fx8 = Fixed<i16, 8>;

/// `i16` with 14 bits of fixed-point fraction.
///
/// This is used by the [`ArcTan`](crate::bios::ArcTan) and
/// [`ArcTan2`](crate::bios::ArcTan2) BIOS functions.
///
/// * This build of the docs does not use the `fixed` feature and uses the
///   crate's internal fixed point type.
#[allow(non_camel_case_types)]
#[cfg(not(feature = "fixed"))]
pub type i16fx14 = Fixed<i16, 14>;

/// `i32` with 8 bits of fixed-point fraction.
///
/// This is used by the background reference point entries.
///
/// * This build of the docs does not use the `fixed` feature and uses the
///   crate's internal fixed point type.
#[allow(non_camel_case_types)]
#[cfg(not(feature = "fixed"))]
pub type i32fx8 = Fixed<i32, 8>;

/// `i16` with 8 bits of fixed-point fraction.
///
/// This is used by the affine matrix entries.
///
/// * This build of the docs uses the `fixed` feature and uses the fixed point
///   type from the `fixed` crate.
#[allow(non_camel_case_types)]
#[cfg(feature = "fixed")]
pub type i16fx8 = ::fixed::FixedI32<::fixed::types::extra::U8>;

/// `i16` with 14 bits of fixed-point fraction.
///
/// This is used by the [`ArcTan`](crate::bios::ArcTan) and
/// [`ArcTan2`](crate::bios::ArcTan2) BIOS functions.
///
/// * This build of the docs uses the `fixed` feature and uses the fixed point
///   type from the `fixed` crate.
#[allow(non_camel_case_types)]
#[cfg(feature = "fixed")]
pub type i16fx14 = ::fixed::FixedI32<::fixed::types::extra::U14>;

/// `i32` with 8 bits of fixed-point fraction.
///
/// This is used by the background reference point entries.
///
/// * This build of the docs uses the `fixed` feature and uses the fixed point
///   type from the `fixed` crate.
#[allow(non_camel_case_types)]
#[cfg(feature = "fixed")]
pub type i32fx8 = ::fixed::FixedI32<::fixed::types::extra::U8>;

/// A [fixed-point][wp-fp] number. This transparently wraps an integer with a
/// const generic for how many bits are fractional.
///
/// [wp-fp]: https://en.wikipedia.org/wiki/Fixed-point_arithmetic
///
/// * This type is generic, but the `I` type is intended to be a signed or
///   unsigned integer of a fixed bit size: `i8`, `i16`, `i32`, `u8`, `u16`, or
///   `u32`. This type is *not* semver supported to work with any other `I`
///   type. If it does work for other types of `I`, that's on accident.
/// * The `B` value is the number of bits that form the fractional part. It
///   should be *less than* the number of bits in the integer's type. Multiply
///   and divide ops need to shift the value by `B`, and so if `B` is greater
///   than or equal to the integer's size the op will panic.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Fixed<I, const B: u32>(I);

macro_rules! impl_trait_op_unit {
  ($t:ty, $trait:ident, $op:ident) => {
    impl<const B: u32> $trait for Fixed<$t, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      fn $op(self) -> Self::Output {
        Self::$op(self)
      }
    }
  };
}
macro_rules! impl_trait_op_self_rhs {
  ($t:ty, $trait:ident, $op:ident) => {
    impl<const B: u32> $trait for Fixed<$t, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      fn $op(self, rhs: Self) -> Self::Output {
        Self::$op(self, rhs)
      }
    }
  };
}
macro_rules! impl_trait_op_assign_self_rhs {
  ($t:ty, $trait:ident, $op:ident, $op_assign:ident) => {
    impl<const B: u32> $trait for Fixed<$t, B> {
      #[inline]
      #[cfg_attr(feature = "track_caller", track_caller)]
      fn $op_assign(&mut self, rhs: Self) {
        *self = self.$op(rhs);
      }
    }
  };
}
macro_rules! impl_shift_self_u32 {
  ($t:ty, $trait:ident, $op:ident) => {
    impl<const B: u32> $trait<u32> for Fixed<$t, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      fn $op(self, rhs: u32) -> Self::Output {
        Self::$op(self, rhs)
      }
    }
  };
}
macro_rules! impl_shift_assign_self_u32 {
  ($t:ty, $trait:ident, $op:ident, $op_assign:ident) => {
    impl<const B: u32> $trait<u32> for Fixed<$t, B> {
      #[inline]
      #[cfg_attr(feature = "track_caller", track_caller)]
      fn $op_assign(&mut self, rhs: u32) {
        *self = self.$op(rhs);
      }
    }
  };
}

macro_rules! impl_common_fixed_ops {
  ($t:ty) => {
    impl<const B: u32> Fixed<$t, B> {
      /// Shifts the value left by `B`, wrapping it into the range of this Fixed
      /// type.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn wrapping_from(i: $t) -> Self {
        Self(i << B)
      }

      /// Makes a `Fixed` directly from a raw inner value (no shift).
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn from_bits(i: $t) -> Self {
        Self(i)
      }

      /// Unwraps the inner value directly into the base type (no shift).
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn to_bits(self) -> $t {
        self.0
      }

      /// Bitwise Not.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn not(self) -> Self {
        Self(!self.0)
      }

      /// Addition.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
      }

      /// Subtraction.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
      }

      /// Remainder.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
      }

      /// Bitwise AND.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
      }

      /// Bitwise OR.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
      }

      /// Bitwise XOR.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
      }

      /// Bit-shift Left.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn shl(self, rhs: u32) -> Self {
        Self(self.0 << rhs)
      }

      /// Bit-shift Right.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
      }
    }
    impl_trait_op_unit!($t, Not, not);
    impl_trait_op_self_rhs!($t, Add, add);
    impl_trait_op_self_rhs!($t, Sub, sub);
    impl_trait_op_self_rhs!($t, Mul, mul);
    impl_trait_op_self_rhs!($t, Div, div);
    impl_trait_op_self_rhs!($t, Rem, rem);
    impl_trait_op_self_rhs!($t, BitAnd, bitand);
    impl_trait_op_self_rhs!($t, BitOr, bitor);
    impl_trait_op_self_rhs!($t, BitXor, bitxor);
    impl_shift_self_u32!($t, Shl, shl);
    impl_shift_self_u32!($t, Shr, shr);
    impl_trait_op_assign_self_rhs!($t, AddAssign, add, add_assign);
    impl_trait_op_assign_self_rhs!($t, SubAssign, sub, sub_assign);
    impl_trait_op_assign_self_rhs!($t, MulAssign, mul, mul_assign);
    impl_trait_op_assign_self_rhs!($t, DivAssign, div, div_assign);
    impl_trait_op_assign_self_rhs!($t, RemAssign, rem, rem_assign);
    impl_trait_op_assign_self_rhs!($t, BitAndAssign, bitand, bitand_assign);
    impl_trait_op_assign_self_rhs!($t, BitOrAssign, bitor, bitor_assign);
    impl_trait_op_assign_self_rhs!($t, BitXorAssign, bitxor, bitxor_assign);
    impl_shift_assign_self_u32!($t, ShlAssign, shl, shl_assign);
    impl_shift_assign_self_u32!($t, ShrAssign, shr, shr_assign);
  };
}
impl_common_fixed_ops!(i8);
impl_common_fixed_ops!(i16);
impl_common_fixed_ops!(i32);
impl_common_fixed_ops!(u8);
impl_common_fixed_ops!(u16);
impl_common_fixed_ops!(u32);

macro_rules! impl_signed_fixed_ops {
  ($t:ty, $unsigned:ty) => {
    impl<const B: u32> Fixed<$t, B> {
      /// Negate.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn neg(self) -> Self {
        Self(-self.0)
      }

      /// If the number is negative or not.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn is_negative(self) -> bool {
        self.0 < 0
      }

      /// Multiply.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn mul(self, rhs: Self) -> Self {
        let raw = (self.0 as i32) * (rhs.0 as i32);
        Self((raw >> B) as $t)
      }

      /// Divide.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn div(self, rhs: Self) -> Self {
        let m = (self.0 as i32) * (1 << B);
        let d = m / (rhs.0 as i32);
        Self(d as $t)
      }

      /// Fractional part of the value.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn fract(self) -> Self {
        let frac_mask = (<$unsigned>::MAX >> (<$t>::BITS - B));
        Self((self.0.unsigned_abs() & frac_mask) as $t)
      }

      /// Whole part of the value.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn trunc(self) -> Self {
        Self(((self.0.unsigned_abs() >> B) << B) as $t)
      }
    }
    impl_trait_op_unit!($t, Neg, neg);
    impl<const B: u32> core::fmt::Debug for Fixed<$t, B> {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let whole: $t = self.trunc().to_bits() >> B;
        let fract: $t = self.fract().to_bits();
        let divisor: $t = 1 << B;
        if self.is_negative() {
          let whole = whole.unsigned_abs();
          write!(f, "-({whole}+{fract}/{divisor})")
        } else {
          write!(f, "{whole}+{fract}/{divisor}")
        }
      }
    }
  };
}
impl_signed_fixed_ops!(i8, u8);
impl_signed_fixed_ops!(i16, u16);
impl_signed_fixed_ops!(i32, u32);

macro_rules! impl_unsigned_fixed_ops {
  ($t:ty) => {
    impl<const B: u32> Fixed<$t, B> {
      /// Multiply.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn mul(self, rhs: Self) -> Self {
        let raw = (self.0 as u32) * (rhs.0 as u32);
        Self((raw >> B) as $t)
      }

      /// Divide.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn div(self, rhs: Self) -> Self {
        let m = (self.0 as u32) * (1 << B);
        let d = m / (rhs.0 as u32);
        Self(d as $t)
      }

      /// Fractional part of the value.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn fract(self) -> Self {
        Self(self.0 & (<$t>::MAX >> (<$t>::BITS - B)))
      }

      /// Whole part of the value.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn trunc(self) -> Self {
        Self(self.0 & (<$t>::MAX << B))
      }
    }
    impl<const B: u32> core::fmt::Debug for Fixed<$t, B> {
      #[inline]
      fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let whole: $t = self.trunc().to_bits() >> B;
        let fract: $t = self.fract().to_bits();
        let divisor: $t = 1 << B;
        write!(f, "{whole}+{fract}/{divisor}")
      }
    }
  };
}
impl_unsigned_fixed_ops!(u8);
impl_unsigned_fixed_ops!(u16);
impl_unsigned_fixed_ops!(u32);
