//! A GBA-specific "cell" type that allows safe global mutable data.
//!
//! Most importantly, data stored in a [`GbaCell`] can be safely shared between
//! the main program and the interrupt handler.
//!
//! All you have to do is declare a static `GbaCell`:
//!
//! ```
//! static THE_COLOR: GbaCell<Color> = GbaCell::new(Color::new());
//! ```
//!
//! And then you can use the [`read`](GbaCell::read) and
//! [`write`](GbaCell::write) methods to interact with the data:
//!
//! ```
//! # static THE_COLOR: GbaCell<Color> = GbaCell::new(Color::new());
//! # let some_new_color = Color::default();
//! let old_color = THE_COLOR.read();
//!
//! THE_COLOR.write(some_new_color);
//! ```

use core::{
  cell::UnsafeCell,
  fmt::Debug,
  mem::{align_of, size_of},
  panic::RefUnwindSafe,
};

use crate::IrqFn;

/// A GBA-specific wrapper around Rust's [`UnsafeCell`](core::cell::UnsafeCell)
/// type.
///
/// Supports any data type that implements the [`GbaCellSafe`] marker trait.
///
/// ## Safety Logic
///
/// * LLVM thinks that ARMv4T only supports atomic operations via special atomic
///   support library functions (which is basically true).
/// * If you directly write an Acquire/load, Release/store, or a Relaxed op with
///   a `compiler_fence`, then LLVM does generate correct code. However, it will
///   have sub-optimal performance. LLVM will generate calls to the mythical
///   atomic support library when it should just directly use an `ldr` or `str`
///   instruction.
/// * In response to this LLVM nonsense, the `GbaCell` type just uses inline
///   assembly to perform all accesses to the contained data.
/// * When LLVM sees inline assembly, it is forced to defensively act as if the
///   inline assembly might have done *anything* legally possible using the
///   pointer and value provided to the inline assembly. This includes that the
///   inline assembly *might* call the atomic support library to access the
///   pointer's data.
/// * So LLVM has to treat the inline assembly as an atomic sync point.
#[repr(transparent)]
pub struct GbaCell<T>(UnsafeCell<T>);
impl<T> Debug for GbaCell<T>
where
  T: GbaCellSafe + Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    <T as Debug>::fmt(&self.read(), f)
  }
}
unsafe impl<T> Send for GbaCell<T> {}
unsafe impl<T> Sync for GbaCell<T> {}
impl<T> RefUnwindSafe for GbaCell<T> {}
impl<T> GbaCell<T>
where
  T: GbaCellSafe,
{
  #[inline]
  #[must_use]
  pub const fn new(val: T) -> Self {
    Self(UnsafeCell::new(val))
  }
  #[inline]
  #[must_use]
  pub const fn get_ptr(&self) -> *mut T {
    self.0.get()
  }
  #[inline]
  #[must_use]
  pub fn read(&self) -> T {
    let p: *const T = self.get_ptr();
    match (size_of::<Self>(), align_of::<Self>()) {
      (4, 4) => unsafe {
        let val: u32;
        core::arch::asm!(
          "ldr {r}, [{addr}]",
          r = out(reg) val,
          addr = in(reg) p,
          options(readonly, preserves_flags, nostack)
        );
        core::mem::transmute_copy(&val)
      },
      (2, 2) => unsafe {
        let val: u16;
        core::arch::asm!(
          "ldrh {r}, [{addr}]",
          r = out(reg) val,
          addr = in(reg) p,
          options(readonly, preserves_flags, nostack)
        );
        core::mem::transmute_copy(&val)
      },
      (1, 1) => unsafe {
        let val: u8;
        core::arch::asm!(
          "ldrb {r}, [{addr}]",
          r = out(reg) val,
          addr = in(reg) p,
          options(readonly, preserves_flags, nostack)
        );
        core::mem::transmute_copy(&val)
      },
      _ => {
        unimplemented!()
      }
    }
  }
  #[inline]
  pub fn write(&self, val: T) {
    let p: *mut T = self.get_ptr();
    match (size_of::<Self>(), align_of::<Self>()) {
      (4, 4) => unsafe {
        let u: u32 = core::mem::transmute_copy(&val);
        core::arch::asm!(
          "str {val}, [{addr}]",
          val = in(reg) u,
          addr = in(reg) p,
          options(preserves_flags, nostack)
        )
      },
      (2, 2) => unsafe {
        let u: u16 = core::mem::transmute_copy(&val);
        core::arch::asm!(
          "strh {val}, [{addr}]",
          val = in(reg) u,
          addr = in(reg) p,
          options(preserves_flags, nostack)
        )
      },
      (1, 1) => unsafe {
        let u: u8 = core::mem::transmute_copy(&val);
        core::arch::asm!(
          "strb {val}, [{addr}]",
          val = in(reg) u,
          addr = in(reg) p,
          options(preserves_flags, nostack)
        )
      },
      _ => {
        unimplemented!()
      }
    }
  }
}

/// Marker trait bound for the methods of [`GbaCell`].
///
/// When a type implements this trait it indicates that the type can be loaded
/// from a pointer in a single instruction. Also it can be stored to a pointer
/// in a single instruction.
///
/// The exact pair of load/store instructions used will depend on the type's
/// size (`ldr`/`str`, `ldrh`/`strh`, or `ldrb`/`strb`).
///
/// ## Safety
/// The type must fit in a single register and have an alignment equal to its
/// size. Generally that means it should be one of:
///
/// * an 8, 16, or 32 bit integer
/// * a function pointer
/// * a data pointer to a sized type
/// * an optional non-null pointer (to function or sized data)
/// * a `repr(transparent)` newtype over one of the above
pub unsafe trait GbaCellSafe: Copy {}

unsafe impl GbaCellSafe for u8 {}
unsafe impl GbaCellSafe for i8 {}
unsafe impl GbaCellSafe for bool {}
unsafe impl GbaCellSafe for u16 {}
unsafe impl GbaCellSafe for i16 {}
unsafe impl GbaCellSafe for u32 {}
unsafe impl GbaCellSafe for i32 {}
unsafe impl GbaCellSafe for char {}
unsafe impl GbaCellSafe for Option<IrqFn> {}
// TODO: many more impls can be added over time
