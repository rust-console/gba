use core::cell::UnsafeCell;

/// This is a "cell" type that allows you to safely have global data on the GBA.
///
/// Particularly, this allows your interrupt handler to safely communicate with
/// your main program.
#[derive(Debug)]
#[repr(transparent)]
pub struct GbaCell<T>(UnsafeCell<T>);

// Safety: This is safe for any `T` that we can read/write from the cell in a
// single instruction. We control what types `T` can be using by only
// implementing the `new` method for known-good types.
unsafe impl<T> Sync for GbaCell<T> {}

impl<T> GbaCell<T> {
  /// Makes a new value.
  ///
  /// Prefer the [new](Self::new) method when possible. Only use this if you're
  /// very sure that your type is supported despite `new` not being implemented
  /// for that type.
  ///
  /// ## Safety
  /// * You must **only** use this with types that are accessed with a single
  ///   instruction.
  /// * This means just 1, 2, and 4 byte integer values, or newtype wrappers
  ///   over such values.
  /// * Also allowed is pointers (both function and data).
  /// * Do **not** put any multi-field structs in a `GbaCell`
  #[inline]
  #[must_use]
  pub const unsafe fn new_unchecked(t: T) -> Self {
    Self(UnsafeCell::new(t))
  }
  #[inline]
  #[must_use]
  pub fn read(&self) -> T {
    unsafe { self.0.get().read_volatile() }
  }
  #[inline]
  pub fn write(&self, t: T) {
    unsafe { self.0.get().write_volatile(t) }
  }
  #[inline]
  #[must_use]
  pub const fn get_ptr(&self) -> *mut T {
    self.0.get()
  }
}

macro_rules! unsafe_impl_gba_cell_new_for {
  ( $( $t:ty ),+ ) => {
    $(
      impl GbaCell<$t> {
        #[inline]
        #[must_use]
        pub const fn new(val: $t) -> Self {
          unsafe { Self::new_unchecked(val) }
        }
      }
    )+
  }
}

unsafe_impl_gba_cell_new_for! {
  u8, i8, u16, i16, u32, i32, usize, isize
}
