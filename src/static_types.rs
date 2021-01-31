//! Module containing types that allow for mutable statics.

use core::{
  cell::UnsafeCell,
  cmp::Ordering,
  fmt, ptr,
  sync::atomic::{fence, Ordering::AcqRel},
};

macro_rules! impl_static {
  ($i: ident, $ty: ty) => {
    impl_static!($i, $ty, stringify!($i));
  };

  ($i: ident, $ty: ty, $n: expr) => {
    #[repr(transparent)]
    /// A static type with interior mutability
    pub struct $i {
      inner: UnsafeCell<$ty>,
    }

    impl $i {
      #[inline(always)]
      /// Creates a new static value
      pub const fn new(val: $ty) -> Self {
        Self { inner: UnsafeCell::new(val) }
      }

      #[inline(always)]
      /// Sets the inner value and discards the old value
      pub fn set(&self, val: $ty) {
        self.replace(val);
      }

      #[inline(always)]
      /// Swaps the two inner values without requiring exclusive access
      pub fn swap(&self, other: &Self) {
        if !ptr::eq(self, other) {
          unsafe { ptr::swap_nonoverlapping(self.inner.get(), other.inner.get(), 1) }
          fence(AcqRel)
        }
      }

      #[inline(always)]
      /// Sets the inner value and returns the old value
      pub fn replace(&self, val: $ty) -> $ty {
        unsafe {
          let old_val = self.inner.get().read_volatile();
          self.inner.get().write_volatile(val);
          fence(AcqRel);
          old_val
        }
      }

      #[inline(always)]
      /// Returns a copy of the inner value
      pub fn get(&self) -> $ty {
        let inner = unsafe { self.inner.get().read_volatile() };
        fence(AcqRel);
        inner
      }

      #[inline(always)]
      /// Updates the inner value using the given function and returns a copy of the updated value
      pub fn update<F: FnOnce($ty) -> $ty>(&self, f: F) -> $ty {
        let old_val = self.get();
        let new_val = f(old_val);
        self.set(new_val);
        new_val
      }

      #[inline(always)]
      /// Returns the inner value and replaces it with the default value
      pub fn take(&self) -> $ty {
        self.replace(<$ty>::default())
      }
    }

    impl Clone for $i {
      fn clone(&self) -> Self {
        Self::new(self.get())
      }
    }

    impl fmt::Debug for $i {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple(stringify!(i)).field(&self.get()).finish()
      }
    }

    impl Default for $i {
      fn default() -> Self {
        Self::new(<$ty>::default())
      }
    }

    impl Eq for $i {}

    impl PartialEq for $i {
      fn eq(&self, rhs: &Self) -> bool {
        self.get() == rhs.get()
      }
    }

    impl Ord for $i {
      fn cmp(&self, rhs: &Self) -> Ordering {
        self.get().cmp(&rhs.get())
      }
    }

    impl PartialOrd for $i {
      fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
      }
    }

    // SAFETY: Because the GBA doesn't have hardware threads, data races are impossible
    unsafe impl Sync for $i {}
  };
}

impl_static!(StaticBool, bool);
impl_static!(StaticI8, i8);
impl_static!(StaticU8, u8);
impl_static!(StaticI16, i16);
impl_static!(StaticU16, u16);
impl_static!(StaticI32, i32);
impl_static!(StaticU32, u32);
impl_static!(StaticIsize, isize);
impl_static!(StaticUsize, usize);

/// A type representing a static mutable memory location.
///
/// The way it achieves memory safety is very similar to [`Cell<T>`](core::cell::Cell):
/// It never allows you to get a reference to whats inside the `Static<T>`.
/// The only way to get the inner value is to either copy it out of there
/// or to replace it with another value. That way, you can never get reference
/// invalidation from overwriting the inner value. Also, for this type to be
/// safe to use both inside the interrupt handler and the main program, it has
/// to carry an extra flag with it. The reason for this is that if `size_of::<T>() > size_of::<usize>()`,
/// the CPU can't write the entire value in a single instruction, so the write can be interrupted
/// by in IRQ. That way you could get data tearing when reading the value inside the IRQ handler,
/// so we have to make sure that you can only access the inner value if it's not currently being written to.
/// 
/// ## NOTE
/// 
/// If you want a static value that fits inside a `usize`, ***don't*** use this type. If you want
/// to have a static primitive (bool, i8, u8, ..., usize), use the StaticT versions of those (for example [`StaticBool`]).
/// If you want a static variable of a custom type that fits in one word, use [`StaticWord<T>`] instead.
/// The reason for that is that this type carries a runtime overhead because it needs to check for
/// availability on every read/write. This availability check also makes the API more complicated to use.
/// 
/// # Examples
/// 
/// ```ignore
/// 
/// struct State([u8; 5]);
/// 
/// static STATE: Static<State> = Static::new(State([0; 5]));
/// 
/// extern "C" fn irq_handler(flags: IrqFlags) {
///   STATE.try_set([1, 2, 3, 4, 5]);
/// } 
/// 
/// fn main() {
///   set_irq_handler(irq_handler);
///   while STATE.try_get().unwrap_or([0; 5])[1] == 0 {}
///   debug!("STATE changed!");
/// }
/// ```
/// 
/// This admittedly very unrealistic example shows that you can use the `Static<T>` both in the IRQ handler and
/// in the main program and the types aren't constrained to primitives.
pub struct Static<T> {
  available: StaticBool,
  inner: UnsafeCell<T>,
}

impl<T> Static<T> {
  #[inline(always)]
  /// Constructs a new `Static<T>` with the given value.
  pub const fn new(val: T) -> Self {
    Self { available: StaticBool::new(true), inner: UnsafeCell::new(val) }
  }

  #[inline(always)]
  /// Tries to set the inner value to the given one.
  /// 
  /// If this succeeds, the method returns `Ok(())`.
  /// If it fails, the method returns `Err(val)`, where `val` is the function argument
  pub fn try_set(&self, val: T) -> Result<(), T> {
    self.try_replace(val).map(|_| ())
  }

  /// Tries to replace the inner value with the given one.
  /// 
  /// If this succeeds, the method returns `Ok(old_val)`, where `old_val` is the previous inner value.
  /// If it fails, it returns `Err(val)`, where `val` is the function argument
  pub fn try_replace(&self, val: T) -> Result<T, T> {
    if self.available.get() {
      self.available.set(false);
      let old_val = unsafe {
        let old_val = self.inner.get().read_volatile();
        self.inner.get().write_volatile(val);
        old_val
      };
      self.available.set(true);
      fence(AcqRel);
      Ok(old_val)
    } else {
      Err(val)
    }
  }

  #[inline(always)]
  /// Tries to extract the inner value by consuming the `Static<T>`.
  /// 
  /// If this succeeds, it returns `Ok(inner)`
  /// If it fails, it returns `Err(self)`
  pub fn try_into_inner(self) -> Result<T, Self> {
    if self.available.get() {
      Ok(self.inner.into_inner())
    } else {
      Err(self)
    }
  }
}

impl<T: Copy> Static<T> {
  /// Tries to copy the inner value.
  /// 
  /// If this fails, the method will return `None`
  pub fn try_get(&self) -> Option<T> {
    if self.available.get() {
      self.available.set(false);
      let v = unsafe { self.inner.get().read_volatile() };
      self.available.set(true);
      fence(AcqRel);
      Some(v)
    } else {
      None
    }
  }

  /// Tries to update the inner value using the given function.
  /// 
  /// If this fails, the method will return `None`
  pub fn try_update<F: FnOnce(T) -> T>(&self, f: F) -> Option<T> {
    let old_val = self.try_get()?;
    let new_val = f(old_val);
    self.try_set(new_val).ok()?;
    Some(new_val)
  }
}

impl<T: Default> Default for Static<T> {
  fn default() -> Self {
    Self::new(T::default())
  }
}

// SAFETY: Because `Static<T>` checks for availability before every operation, data races cannot occur
unsafe impl<T> Sync for Static<T> {}

#[repr(transparent)]
/// A mutable static memory location that fits inside one machine word.
/// 
/// This works pretty similarly to [`Static<T>`](self::Static), except that it doesn't have the
/// associated runtime overhead and complicated API. This is because `StaticWord<T>` only
/// allows for inner types that fit in one machine word (`usize`), so an IRQ can't happen
/// during a write to the inner value. Because of that data tearing cannot happen so we don't
/// have to check for it.
/// 
/// ## NOTE:
/// 
/// If you use this type with a `T` which is larger than one word, the constructor will panic at
/// runtime. This is because rust currently doesn't allow for const assertions, so we can't check
/// that `size_of::<T>() <= size_of::<usize>()` at compile time.
/// 
/// # Example:
/// 
/// ```ignore
/// 
/// #[derive(PartialEq, Copy, Clone)]
/// enum State {
///   Alive,
///   Dead
/// }
/// 
/// static STATE: Static<State> = Static::new(State::Alive);
/// 
/// extern "C" fn irq_handler(flags: IrqFlags) {
///   STATE.set(State::Dead)
/// }
/// 
/// fn main() {
///   set_irq_handler(irq_handler);
///   while STATE.get() == State::Alive {}
///   debug!("You died!");
/// }
/// ```
/// 
/// This example shows that it's possible to use the static variable from the IRQ handler
/// and the main program
pub struct StaticWord<T> {
  inner: UnsafeCell<T>,
}

impl<T> StaticWord<T> {
  #[inline(always)]
  /// Constructs a new `StaticWord<T>` with the given type.
  /// 
  /// ## NOTE:
  /// If the size of the inner type is larger than a word, this
  /// function will panic with a very ugly panic message.
  /// This is because currently there's no proper way to assert
  /// things in a `const fn`
  pub const fn new(val: T) -> Self {
    // An ugly way of asserting that `T` is not larger that a word.
    // This panics at runtime if `T` is too large.
    // Obviously a compile time check would be way preferable,
    // but that's not possible in current stable rust
    [()][(core::mem::size_of::<T>() > core::mem::size_of::<usize>()) as usize];
    Self { inner: UnsafeCell::new(val) }
  }

  #[inline(always)]
  /// Sets the inner value and discards the old value.
  pub fn set(&self, val: T) {
    self.replace(val);
  }

  #[inline(always)]
  /// Swaps the contents of the two `StaticWord<T>`.
  /// Unlike [`core::mem::swap`] this doesn't require exclusive access
  pub fn swap(&self, other: &Self) {
    if !ptr::eq(self, other) {
      unsafe {
        ptr::swap_nonoverlapping(self.inner.get(), other.inner.get(), 1);
        fence(AcqRel)
      }
    }
  }

  /// Sets the new inner value and returns the old value.
  pub fn replace(&self, val: T) -> T {
    unsafe {
      let old_val = self.inner.get().read_volatile();
      self.inner.get().write_volatile(val);
      fence(AcqRel);
      old_val
    }
  }

  #[inline(always)]
  /// Consumes the `StaticWord<T>` and returns the inner value.
  pub fn into_inner(self) -> T {
    self.inner.into_inner()
  }
}

impl<T: Copy> StaticWord<T> {
  #[inline(always)]
  /// Copies the inner value and returns it.
  pub fn get(&self) -> T {
    let v = unsafe { self.inner.get().read_volatile() };
    fence(AcqRel);
    v
  }

  /// Updates the inner value with the given function and returns the new inner value.
  pub fn update<F: FnOnce(T) -> T>(&self, f: F) -> T {
    let old_val = self.get();
    let new_val = f(old_val);
    self.set(new_val);
    old_val
  }
}

impl<T: Default> StaticWord<T> {
  /// Replaces the inner value with the default value.
  pub fn take(&self) -> T {
    self.replace(T::default())
  }
}

impl<T: Copy> Clone for StaticWord<T> {
  fn clone(&self) -> Self {
    Self::new(self.get())
  }
}

impl<T: Copy + Eq> Eq for StaticWord<T> {}

impl<T: Copy + PartialEq> PartialEq for StaticWord<T> {
  fn eq(&self, rhs: &Self) -> bool {
    self.get() == rhs.get()
  }
}

impl<T: Copy + Ord> Ord for StaticWord<T> {
  fn cmp(&self, rhs: &Self) -> Ordering {
    self.get().cmp(&rhs.get())
  }
}

impl<T: Copy + PartialOrd> PartialOrd for StaticWord<T> {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    self.get().partial_cmp(&rhs.get())
  }
}

// SAFETY: Because the GBA doesn't have hardware threads, data races are impossible
unsafe impl<T: Copy> Sync for StaticWord<T> {}
