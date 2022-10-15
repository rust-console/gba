use core::ops::*;

// TODO: this derived Debug impl prints the wrong thing, but it's fine for now.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Fixed<I, const B: u32>(I);

macro_rules! impl_passthrough_self_rhs {
  ($trait_name:ident, $method_name:ident) => {
    impl<I: $trait_name<Output = I>, const B: u32> $trait_name for Fixed<I, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      fn $method_name(self, rhs: Self) -> Self::Output {
        Self(self.0.$method_name(rhs.0))
      }
    }
  };
}
impl_passthrough_self_rhs!(Add, add);
impl_passthrough_self_rhs!(Sub, sub);
impl_passthrough_self_rhs!(Rem, rem);
impl_passthrough_self_rhs!(BitAnd, bitand);
impl_passthrough_self_rhs!(BitOr, bitor);
impl_passthrough_self_rhs!(BitXor, bitxor);

macro_rules! impl_passthrough_self_assign {
  ($trait_name:ident, $method_name:ident) => {
    impl<I: $trait_name, const B: u32> $trait_name for Fixed<I, B> {
      #[inline]
      fn $method_name(&mut self, rhs: Self) {
        self.0.$method_name(rhs.0);
      }
    }
  };
}
impl_passthrough_self_assign!(AddAssign, add_assign);
impl_passthrough_self_assign!(SubAssign, sub_assign);
impl_passthrough_self_assign!(RemAssign, rem_assign);
impl_passthrough_self_assign!(BitAndAssign, bitand_assign);
impl_passthrough_self_assign!(BitOrAssign, bitor_assign);
impl_passthrough_self_assign!(BitXorAssign, bitxor_assign);

macro_rules! impl_self_unit {
  ($trait_name:ident, $method_name:ident) => {
    impl<I: $trait_name<Output = I>, const B: u32> $trait_name for Fixed<I, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      fn $method_name(self) -> Self::Output {
        Self(self.0.$method_name())
      }
    }
  };
}
impl_self_unit!(Neg, neg);
impl_self_unit!(Not, not);

macro_rules! impl_shift {
  ($trait_name:ident, $method_name:ident) => {
    impl<S, I: $trait_name<S, Output = I>, const B: u32> $trait_name<S>
      for Fixed<I, B>
    {
      type Output = Self;
      #[inline]
      #[must_use]
      fn $method_name(self, rhs: S) -> Self::Output {
        Self(self.0.$method_name(rhs))
      }
    }
  };
}
impl_shift!(Shl, shl);
impl_shift!(Shr, shr);

macro_rules! impl_shift_assign {
  ($trait_name:ident, $method_name:ident) => {
    impl<S, I: $trait_name<S>, const B: u32> $trait_name<S> for Fixed<I, B> {
      #[inline]
      fn $method_name(&mut self, rhs: S) {
        self.0.$method_name(rhs)
      }
    }
  };
}
impl_shift_assign!(ShlAssign, shl_assign);
impl_shift_assign!(ShrAssign, shr_assign);

macro_rules! impl_signed_mul {
  ($i:ty) => {
    impl<const B: u32> Mul for Fixed<$i, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn mul(self, rhs: Self) -> Self::Output {
        Self(((self.0 as i32).mul(rhs.0 as i32) >> B) as $i)
      }
    }
  };
}
impl_signed_mul!(i8);
impl_signed_mul!(i16);
impl_signed_mul!(i32);

macro_rules! impl_unsigned_mul {
  ($u:ty) => {
    impl<const B: u32> Mul for Fixed<$u, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn mul(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u32).mul(rhs.0 as u32) >> B) as $u)
      }
    }
  };
}
impl_unsigned_mul!(u8);
impl_unsigned_mul!(u16);
impl_unsigned_mul!(u32);

impl<I, const B: u32> MulAssign for Fixed<I, B>
where
  Self: Mul<Output = Self> + Clone,
{
  #[inline]
  fn mul_assign(&mut self, rhs: Self) {
    *self = self.clone().mul(rhs);
  }
}

macro_rules! impl_signed_div {
  ($i:ty) => {
    impl<const B: u32> Div for Fixed<$i, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn div(self, rhs: Self) -> Self::Output {
        Self((self.0 as i32).mul(1 << B).div(rhs.0 as i32) as $i)
      }
    }
  };
}
impl_signed_div!(i8);
impl_signed_div!(i16);
impl_signed_div!(i32);

macro_rules! impl_unsigned_div {
  ($u:ty) => {
    impl<const B: u32> Div for Fixed<$u, B> {
      type Output = Self;
      #[inline]
      #[must_use]
      #[allow(clippy::suspicious_arithmetic_impl)]
      fn div(self, rhs: Self) -> Self::Output {
        Self((self.0 as u32).mul(1 << B).div(rhs.0 as u32) as $u)
      }
    }
  };
}
impl_unsigned_div!(u8);
impl_unsigned_div!(u16);
impl_unsigned_div!(u32);

impl<I, const B: u32> DivAssign for Fixed<I, B>
where
  Self: Div<Output = Self> + Clone,
{
  #[inline]
  fn div_assign(&mut self, rhs: Self) {
    *self = self.clone().div(rhs);
  }
}
