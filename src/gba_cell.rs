use core::{
  cell::UnsafeCell,
  fmt::Debug,
  mem::{align_of, size_of},
};

use crate::IrqFn;

#[derive(Default)]
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
impl<T> GbaCell<T> {
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
  pub fn read(&self) -> T
  where
    T: GbaCellSafe,
  {
    let p: *const T = self.0.get();
    unsafe { <T as GbaCellSafe>::read(p) }
  }
  #[inline]
  pub fn write(&self, val: T)
  where
    T: GbaCellSafe,
  {
    let p: *mut T = self.0.get();
    unsafe { <T as GbaCellSafe>::write(p, val) }
  }
}

pub unsafe trait GbaCellSafe: Copy {
  #[inline]
  #[must_use]
  unsafe fn read(p: *const Self) -> Self {
    match (size_of::<Self>(), align_of::<Self>()) {
      (4, 4) => unsafe {
        let val: u32;
        core::arch::asm!(
          "ldr {r}, [{addr}]",
          r = out(reg) val,
          addr = in(reg) p,
          options(nostack)
        );
        core::mem::transmute_copy(&val)
      },
      (2, 2) => unsafe {
        let val: u16;
        core::arch::asm!(
          "ldrh {r}, [{addr}]",
          r = out(reg) val,
          addr = in(reg) p,
          options(nostack)
        );
        core::mem::transmute_copy(&val)
      },
      (1, 1) => unsafe {
        let val: u8;
        core::arch::asm!(
          "ldrb {r}, [{addr}]",
          r = out(reg) val,
          addr = in(reg) p,
          options(nostack)
        );
        core::mem::transmute_copy(&val)
      },
      _ => {
        unimplemented!()
      }
    }
  }

  #[inline]
  unsafe fn write(p: *mut Self, val: Self) {
    match (size_of::<Self>(), align_of::<Self>()) {
      (4, 4) => unsafe {
        let u: u32 = core::mem::transmute_copy(&val);
        core::arch::asm!(
          "str {val}, [{addr}]",
          val = in(reg) u,
          addr = in(reg) p,
          options(nostack)
        )
      },
      (2, 2) => unsafe {
        let u: u16 = core::mem::transmute_copy(&val);
        core::arch::asm!(
          "strh {val}, [{addr}]",
          val = in(reg) u,
          addr = in(reg) p,
          options(nostack)
        )
      },
      (1, 1) => unsafe {
        let u: u8 = core::mem::transmute_copy(&val);
        core::arch::asm!(
          "strb {val}, [{addr}]",
          val = in(reg) u,
          addr = in(reg) p,
          options(nostack)
        )
      },
      _ => {
        unimplemented!()
      }
    }
  }
}

unsafe impl GbaCellSafe for u8 {}
unsafe impl GbaCellSafe for i8 {}
unsafe impl GbaCellSafe for bool {}
unsafe impl GbaCellSafe for u16 {}
unsafe impl GbaCellSafe for i16 {}
unsafe impl GbaCellSafe for u32 {}
unsafe impl GbaCellSafe for i32 {}
unsafe impl GbaCellSafe for char {}
unsafe impl GbaCellSafe for IrqFn {}
// TODO: many more impls can be added over time
