#![allow(non_camel_case_types)]

//! Module for fixed point math types and operations.

use core::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};

/// Fixed point `T` value with `F` fractional bits.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Fx<T, const F: usize> {
  num: T,
}

impl<T, const F: usize> Fx<T, F> {
  /// Uses the provided value directly.
  pub fn from_raw(r: T) -> Self {
    Fx { num: r }
  }

  /// Unwraps the inner value.
  pub fn into_raw(self) -> T {
    self.num
  }

  /// Casts the base type, keeping the fractional bit quantity the same.
  pub fn cast_inner<Z, C: Fn(T) -> Z>(self, op: C) -> Fx<Z, F> {
    Fx { num: op(self.num) }
  }
}

impl<T: Add<Output = T>, const F: usize> Add for Fx<T, F> {
  type Output = Self;
  fn add(self, rhs: Fx<T, F>) -> Self::Output {
    Fx { num: self.num + rhs.num }
  }
}

impl<T: Sub<Output = T>, const F: usize> Sub for Fx<T, F> {
  type Output = Self;
  fn sub(self, rhs: Fx<T, F>) -> Self::Output {
    Fx { num: self.num - rhs.num }
  }
}

impl<T: Shl<u32, Output = T>, const F: usize> Shl<u32> for Fx<T, F> {
  type Output = Self;
  fn shl(self, rhs: u32) -> Self::Output {
    Fx { num: self.num << rhs }
  }
}

impl<T: Shr<u32, Output = T>, const F: usize> Shr<u32> for Fx<T, F> {
  type Output = Self;
  fn shr(self, rhs: u32) -> Self::Output {
    Fx { num: self.num >> rhs }
  }
}

impl<T: Neg<Output = T>, const F: usize> Neg for Fx<T, F> {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Fx { num: -self.num }
  }
}

impl<T, const F: usize> Fx<T, F>
where
  T: Shl<u8, Output = T> + Shr<i32, Output = T> + Shl<i32, Output = T> + From<u8>,
{
  /// Gives the smallest positive non-zero value.
  pub fn precision() -> Self {
    Fx { num: 1.into() }
  }

  /// Makes a value with the integer part shifted into place.
  pub fn from_int_part(i: T) -> Self {
    Fx { num: i << F as u8 }
  }

  /// Changes the fractional bit quantity, keeping the base type the same.
  pub fn adjust_fractional_bits<const Y: usize>(self) -> Fx<T, Y> {
    let leftward_movement: i32 = Y as i32 - F as i32;
    Fx {
      num: if leftward_movement > 0 {
        self.num << leftward_movement
      } else {
        self.num >> (-leftward_movement)
      },
    }
  }
}

macro_rules! fixed_point_signed_multiply {
  ($t:ident) => {
    impl<const F: usize> Mul for Fx<$t, F> {
      type Output = Self;
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn mul(self, rhs: Fx<$t, F>) -> Self::Output {
        let pre_shift = (self.num as i32).wrapping_mul(rhs.num as i32);
        if pre_shift < 0 {
          if pre_shift == core::i32::MIN {
            Fx { num: core::$t::MIN }
          } else {
            Fx { num: (-((-pre_shift) >> F as u8)) as $t }
          }
        } else {
          Fx { num: (pre_shift >> F as u8) as $t }
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
    impl<const F: usize> Mul for Fx<$t, F> {
      type Output = Self;
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn mul(self, rhs: Fx<$t, F>) -> Self::Output {
        Fx { num: ((self.num as u32).wrapping_mul(rhs.num as u32) >> F as u8) as $t }
      }
    }
  };
}

fixed_point_unsigned_multiply! {u8}
fixed_point_unsigned_multiply! {u16}
fixed_point_unsigned_multiply! {u32}

macro_rules! fixed_point_signed_division {
  ($t:ident) => {
    impl<const F: usize> Div for Fx<$t, F> {
      type Output = Self;
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn div(self, rhs: Fx<$t, F>) -> Self::Output {
        let mul_output: i32 = (self.num as i32).wrapping_mul(1 << F as u8);
        let divide_result: i32 = crate::bios::div(mul_output, rhs.num as i32);
        Fx { num: divide_result as $t }
      }
    }
  };
}

fixed_point_signed_division! {i8}
fixed_point_signed_division! {i16}
fixed_point_signed_division! {i32}

macro_rules! fixed_point_unsigned_division {
  ($t:ident) => {
    impl<const F: usize> Div for Fx<$t, F> {
      type Output = Self;
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn div(self, rhs: Fx<$t, F>) -> Self::Output {
        let mul_output: i32 = (self.num as i32).wrapping_mul(1 << F as u8);
        let divide_result: i32 = crate::bios::div(mul_output, rhs.num as i32);
        Fx { num: divide_result as $t }
      }
    }
  };
}

fixed_point_unsigned_division! {u8}
fixed_point_unsigned_division! {u16}
fixed_point_unsigned_division! {u32}

/// Alias for an `i16` fixed point value with 8 fractional bits.
pub type fx8_8 = Fx<i16, 8>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let one = Fx::<u16, 4>::from_int_part(1);
    let two = Fx::<u16, 4>::from_int_part(2);
    assert!(one + one == two)
  }

  #[test]
  fn test_sub() {
    let one = Fx::<u16, 4>::from_int_part(1);
    let two = Fx::<u16, 4>::from_int_part(2);
    assert!(two - one == one)
  }

  #[test]
  fn test_shl() {
    let one = Fx::<u16, 4>::from_int_part(1);
    let two = Fx::<u16, 4>::from_int_part(2);
    assert!(one << 1 == two)
  }

  #[test]
  fn test_shr() {
    let one = Fx::<u16, 4>::from_int_part(1);
    let two = Fx::<u16, 4>::from_int_part(2);
    assert!(two >> 1 == one)
  }

  #[test]
  fn test_neg() {
    let one = Fx::<i16, 4>::from_int_part(1);
    let neg_one = Fx::<i16, 4>::from_int_part(-1);
    assert!(-one == neg_one);
    assert!(-(-one) == one);
  }

  #[test]
  fn test_mul() {
    let half = Fx::<u16, 4>::from_int_part(1) >> 1;
    let two = Fx::<u16, 4>::from_int_part(2);
    let three = Fx::<u16, 4>::from_int_part(3);
    let twelve = Fx::<u16, 4>::from_int_part(12);
    assert!(two * three == twelve * half);
  }

  #[test]
  fn test_div() {
    let two = Fx::<u16, 4>::from_int_part(2);
    let six = Fx::<u16, 4>::from_int_part(6);
    let twelve = Fx::<u16, 4>::from_int_part(12);
    assert!(twelve / two == six);
  }
}
