//! Provides the [`GbaCell`] type.

use core::{
  fmt::Debug,
  num::{NonZeroI16, NonZeroI32, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU8},
  ptr::NonNull,
};

use crate::{irq::IrqBits, keys::KeyInput, video::Color};

/// Marker trait bound for the methods of [`GbaCell`].
///
/// When a type implements this trait it indicates that the type can be
/// atomically loaded/stored using a single volatile access.
///
/// ## Safety
/// The type must fit in a single register, and have an alignment equal to its
/// size. Generally that means it should be one of:
///
/// * an 8, 16, or 32 bit integer
/// * a function pointer
/// * a data pointer to a sized type
/// * an optional non-null pointer (to function or sized data)
/// * a `repr(transparent)` newtype over one of the above
pub unsafe trait GbaCellSafe: Copy {}
// Note(Lokathor): The list here is not exhaustive, it's just all the stuff I
// thought of at the time. Add more as necessary.
unsafe impl GbaCellSafe for u8 {}
unsafe impl GbaCellSafe for u16 {}
unsafe impl GbaCellSafe for u32 {}
unsafe impl GbaCellSafe for i8 {}
unsafe impl GbaCellSafe for i16 {}
unsafe impl GbaCellSafe for i32 {}
unsafe impl GbaCellSafe for NonZeroI16 {}
unsafe impl GbaCellSafe for NonZeroI32 {}
unsafe impl GbaCellSafe for NonZeroI8 {}
unsafe impl GbaCellSafe for NonZeroU16 {}
unsafe impl GbaCellSafe for NonZeroU32 {}
unsafe impl GbaCellSafe for NonZeroU8 {}
unsafe impl GbaCellSafe for Option<NonZeroI16> {}
unsafe impl GbaCellSafe for Option<NonZeroI32> {}
unsafe impl GbaCellSafe for Option<NonZeroI8> {}
unsafe impl GbaCellSafe for Option<NonZeroU16> {}
unsafe impl GbaCellSafe for Option<NonZeroU32> {}
unsafe impl GbaCellSafe for Option<NonZeroU8> {}
unsafe impl GbaCellSafe for char {}
unsafe impl GbaCellSafe for bool {}
unsafe impl GbaCellSafe for Color {}
unsafe impl GbaCellSafe for KeyInput {}
unsafe impl<T> GbaCellSafe for *const T {}
unsafe impl<T> GbaCellSafe for *mut T {}
unsafe impl<T> GbaCellSafe for NonNull<T> {}
unsafe impl<T> GbaCellSafe for Option<NonNull<T>> {}
unsafe impl GbaCellSafe for Option<unsafe extern "C" fn(u16)> {}
unsafe impl GbaCellSafe for Option<unsafe extern "C" fn(IrqBits)> {}
unsafe impl<I: GbaCellSafe, const B: u32> GbaCellSafe
  for crate::gba_fixed::Fixed<I, B>
{
}
#[cfg(feature = "fixed")]
unsafe impl<T> GbaCellSafe for fixed::FixedI32<T> {}
#[cfg(feature = "fixed")]
unsafe impl<T> GbaCellSafe for fixed::FixedI16<T> {}
#[cfg(feature = "fixed")]
unsafe impl<T> GbaCellSafe for fixed::FixedI8<T> {}
#[cfg(feature = "fixed")]
unsafe impl<T> GbaCellSafe for fixed::FixedU32<T> {}
#[cfg(feature = "fixed")]
unsafe impl<T> GbaCellSafe for fixed::FixedU16<T> {}
#[cfg(feature = "fixed")]
unsafe impl<T> GbaCellSafe for fixed::FixedU8<T> {}

/// A "cell" type suitable to hold a global on the GBA.
#[repr(transparent)]
pub struct GbaCell<T>(core::cell::UnsafeCell<T>);
#[cfg(feature = "on_gba")]
impl<T> Debug for GbaCell<T>
where
  T: GbaCellSafe + Debug,
{
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <T as Debug>::fmt(&self.read(), f)
  }
}
impl<T> Default for GbaCell<T>
where
  T: GbaCellSafe + Default,
{
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::new(T::default())
  }
}
#[cfg(feature = "on_gba")]
impl<T> Clone for GbaCell<T>
where
  T: GbaCellSafe + Default,
{
  #[inline]
  #[must_use]
  fn clone(&self) -> Self {
    Self::new(self.read())
  }
}

#[cfg(feature = "on_gba")]
unsafe impl<T> Sync for GbaCell<T> {}

impl<T> GbaCell<T>
where
  T: GbaCellSafe,
{
  /// Constructs a new cell with the value given
  #[inline]
  #[must_use]
  pub const fn new(t: T) -> Self {
    Self(core::cell::UnsafeCell::new(t))
  }

  /// Read the value in the cell.
  ///
  /// ## Panics
  /// The size and alignment of the type must be equal, and they must be 1, 2,
  /// or 4. Anything else will panic.
  #[inline]
  #[must_use]
  #[cfg(feature = "on_gba")]
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub fn read(&self) -> T {
    match (core::mem::size_of::<T>(), core::mem::align_of::<T>()) {
      (4, 4) | (2, 2) | (1, 1) => unsafe { self.0.get().read_volatile() },
      _ => unimplemented!(),
    }
  }

  /// Writes a new value to the cell.
  ///
  /// ## Panics
  /// The size and alignment of the type must be equal, and they must be 1, 2,
  /// or 4. Anything else will panic.
  #[inline]
  #[cfg(feature = "on_gba")]
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub fn write(&self, t: T) {
    match (core::mem::size_of::<T>(), core::mem::align_of::<T>()) {
      (4, 4) | (2, 2) | (1, 1) => unsafe { self.0.get().write_volatile(t) },
      _ => unimplemented!(),
    }
  }
}
