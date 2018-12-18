#![allow(non_camel_case_types)]

//! Module for fixed point math types and operations.

use core::{convert::From, marker::PhantomData};
use typenum::{marker_traits::Unsigned, U8};

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
    Fx {
      num: r,
      phantom: PhantomData,
    }
  }
  /// Unwraps the inner value.
  pub fn into_raw(self) -> T {
    self.num
  }

  /// Casts the base type, keeping the fractional bit quantity the same.
  pub fn cast_inner<Z, C: Fn(T) -> Z>(self, op: C) -> Fx<Z, F> {
    Fx {
      num: op(self.num),
      phantom: PhantomData,
    }
  }
}

macro_rules! fixed_point_methods {
  ($t:ident) => {
    impl<F: Unsigned> Fx<$t, F> {
      /// Gives 0 for this type.
      pub fn zero() -> Self {
        Fx {
          num: 0,
          phantom: PhantomData,
        }
      }

      /// Gives the smallest positive non-zero value.
      pub fn precision() -> Self {
        Fx {
          num: 1,
          phantom: PhantomData,
        }
      }

      /// Makes a value with the integer part shifted into place.
      pub fn from_int_part(i: $t) -> Self {
        Fx {
          num: i << F::to_u8(),
          phantom: PhantomData,
        }
      }

      /// Gives the raw inner value.
      pub fn into_inner(&self) -> $t {
        self.num
      }

      /// Changes the fractional bit quantity, keeping the base type the same.
      pub fn change_bit_quantity<N: Unsigned>(&self) -> Fx<$t, N> {
        unimplemented!()
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

/// Alias for an `i16` fixed point value with 8 fractional bits.
pub type fx8_8 = Fx<i16, U8>;
