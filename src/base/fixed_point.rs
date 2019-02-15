#![allow(non_camel_case_types)]

//! Module for fixed point math types and operations.

use core::{
  marker::PhantomData,
  ops::{Add, Div, Mul, Neg, Shl, Shr, Sub},
};
use typenum::{consts::False, marker_traits::Unsigned, type_operators::IsEqual, U8};

/// Fixed point `T` value with `F` fractional bits.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Fx<T, F: Unsigned> {
  num: T,
  phantom: PhantomData<F>,
}

impl<T, F: Unsigned> Fx<T, F> {
  /// Uses the provided value directly.
  pub fn from_raw(r: T) -> Self {
    Fx { num: r, phantom: PhantomData }
  }

  /// Unwraps the inner value.
  pub fn into_raw(self) -> T {
    self.num
  }

  /// Casts the base type, keeping the fractional bit quantity the same.
  pub fn cast_inner<Z, C: Fn(T) -> Z>(self, op: C) -> Fx<Z, F> {
    Fx { num: op(self.num), phantom: PhantomData }
  }
}

impl<T: Add<Output = T>, F: Unsigned> Add for Fx<T, F> {
  type Output = Self;
  fn add(self, rhs: Fx<T, F>) -> Self::Output {
    Fx { num: self.num + rhs.num, phantom: PhantomData }
  }
}

impl<T: Sub<Output = T>, F: Unsigned> Sub for Fx<T, F> {
  type Output = Self;
  fn sub(self, rhs: Fx<T, F>) -> Self::Output {
    Fx { num: self.num - rhs.num, phantom: PhantomData }
  }
}

impl<T: Shl<u32, Output = T>, F: Unsigned> Shl<u32> for Fx<T, F> {
  type Output = Self;
  fn shl(self, rhs: u32) -> Self::Output {
    Fx { num: self.num << rhs, phantom: PhantomData }
  }
}

impl<T: Shr<u32, Output = T>, F: Unsigned> Shr<u32> for Fx<T, F> {
  type Output = Self;
  fn shr(self, rhs: u32) -> Self::Output {
    Fx { num: self.num >> rhs, phantom: PhantomData }
  }
}

impl<T: Neg<Output = T>, F: Unsigned> Neg for Fx<T, F> {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Fx { num: -self.num, phantom: PhantomData }
  }
}

macro_rules! fixed_point_methods {
  ($t:ident) => {
    impl<F: Unsigned> Fx<$t, F> {
      /// Gives the smallest positive non-zero value.
      pub fn precision() -> Self {
        Fx { num: 1, phantom: PhantomData }
      }

      /// Makes a value with the integer part shifted into place.
      pub fn from_int_part(i: $t) -> Self {
        Fx { num: i << F::U8, phantom: PhantomData }
      }

      /// Changes the fractional bit quantity, keeping the base type the same.
      pub fn adjust_fractional_bits<Y: Unsigned + IsEqual<F, Output = False>>(self) -> Fx<$t, Y> {
        let leftward_movement: i32 = Y::to_i32() - F::to_i32();
        Fx {
          num: if leftward_movement > 0 {
            self.num << leftward_movement
          } else {
            self.num >> (-leftward_movement)
          },
          phantom: PhantomData,
        }
      }
    }
  };
}

fixed_point_methods! {u8}
fixed_point_methods! {i8}
fixed_point_methods! {i16}
fixed_point_methods! {u16}
fixed_point_methods! {i32}
fixed_point_methods! {u32}

macro_rules! fixed_point_signed_multiply {
  ($t:ident) => {
    impl<F: Unsigned> Mul for Fx<$t, F> {
      type Output = Self;
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn mul(self, rhs: Fx<$t, F>) -> Self::Output {
        let pre_shift = (self.num as i32).wrapping_mul(rhs.num as i32);
        if pre_shift < 0 {
          if pre_shift == core::i32::MIN {
            Fx { num: core::$t::MIN, phantom: PhantomData }
          } else {
            Fx { num: (-((-pre_shift) >> F::U8)) as $t, phantom: PhantomData }
          }
        } else {
          Fx { num: (pre_shift >> F::U8) as $t, phantom: PhantomData }
        }
      }
    }
  };
}

fixed_point_signed_multiply! {i8}
fixed_point_signed_multiply! {i16}
fixed_point_signed_multiply! {i32}

macro_rules! fixed_point_unsigned_multiply {
  ($t:ident) => {
    impl<F: Unsigned> Mul for Fx<$t, F> {
      type Output = Self;
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn mul(self, rhs: Fx<$t, F>) -> Self::Output {
        Fx {
          num: ((self.num as u32).wrapping_mul(rhs.num as u32) >> F::U8) as $t,
          phantom: PhantomData,
        }
      }
    }
  };
}

fixed_point_unsigned_multiply! {u8}
fixed_point_unsigned_multiply! {u16}
fixed_point_unsigned_multiply! {u32}

macro_rules! fixed_point_signed_division {
  ($t:ident) => {
    impl<F: Unsigned> Div for Fx<$t, F> {
      type Output = Self;
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn div(self, rhs: Fx<$t, F>) -> Self::Output {
        let mul_output: i32 = (self.num as i32).wrapping_mul(1 << F::U8);
        let divide_result: i32 = crate::bios::div(mul_output, rhs.num as i32);
        Fx { num: divide_result as $t, phantom: PhantomData }
      }
    }
  };
}

fixed_point_signed_division! {i8}
fixed_point_signed_division! {i16}
fixed_point_signed_division! {i32}

macro_rules! fixed_point_unsigned_division {
  ($t:ident) => {
    impl<F: Unsigned> Div for Fx<$t, F> {
      type Output = Self;
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn div(self, rhs: Fx<$t, F>) -> Self::Output {
        let mul_output: i32 = (self.num as i32).wrapping_mul(1 << F::U8);
        let divide_result: i32 = crate::bios::div(mul_output, rhs.num as i32);
        Fx { num: divide_result as $t, phantom: PhantomData }
      }
    }
  };
}

fixed_point_unsigned_division! {u8}
fixed_point_unsigned_division! {u16}
fixed_point_unsigned_division! {u32}

/// Alias for an `i16` fixed point value with 8 fractional bits.
pub type fx8_8 = Fx<i16, U8>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    use typenum::U4;
    let one = Fx::<u16, U4>::from_int_part(1);
    let two = Fx::<u16, U4>::from_int_part(2);
    assert!(one + one == two)
  }

  #[test]
  fn test_sub() {
    use typenum::U4;
    let one = Fx::<u16, U4>::from_int_part(1);
    let two = Fx::<u16, U4>::from_int_part(2);
    assert!(two - one == one)
  }

  #[test]
  fn test_shl() {
    use typenum::U4;
    let one = Fx::<u16, U4>::from_int_part(1);
    let two = Fx::<u16, U4>::from_int_part(2);
    assert!(one << 1 == two)
  }

  #[test]
  fn test_shr() {
    use typenum::U4;
    let one = Fx::<u16, U4>::from_int_part(1);
    let two = Fx::<u16, U4>::from_int_part(2);
    assert!(two >> 1 == one)
  }

  #[test]
  fn test_neg() {
    use typenum::U4;
    let one = Fx::<i16, U4>::from_int_part(1);
    let neg_one = Fx::<i16, U4>::from_int_part(-1);
    assert!(-one == neg_one);
    assert!(-(-one) == one);
  }

  #[test]
  fn test_mul() {
    use typenum::U4;
    let half = Fx::<u16, U4>::from_int_part(1) >> 1;
    let two = Fx::<u16, U4>::from_int_part(2);
    let three = Fx::<u16, U4>::from_int_part(3);
    let twelve = Fx::<u16, U4>::from_int_part(12);
    assert!(two * three == twelve * half);
  }

  #[test]
  fn test_div() {
    use typenum::U4;
    let two = Fx::<u16, U4>::from_int_part(2);
    let six = Fx::<u16, U4>::from_int_part(6);
    let twelve = Fx::<u16, U4>::from_int_part(12);
    assert!(twelve / two == six);
  }
}
