use core::{
  cell::UnsafeCell,
  mem::{align_of, size_of},
};

#[repr(transparent)]
pub struct GbaCell<T>(UnsafeCell<T>);
unsafe impl<T> Send for GbaCell<T> {}
unsafe impl<T> Sync for GbaCell<T> {}
impl<T> GbaCell<T> {
  #[inline]
  pub const fn new(val: T) -> Self {
    Self(UnsafeCell::new(val))
  }
  #[inline]
  pub fn read(&self) -> T {
    match (size_of::<T>(), align_of::<T>()) {
      (4, 4) => unsafe {
        let val: u32;
        let p: *mut T = self.0.get();
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
        let p: *mut T = self.0.get();
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
        let p: *mut T = self.0.get();
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
  pub fn write(&self, val: T) {
    match (size_of::<T>(), align_of::<T>()) {
      (4, 4) => unsafe {
        let u: u32 = core::mem::transmute_copy(&val);
        let p: *mut T = self.0.get();
        core::arch::asm!(
          "str {val}, [{addr}]",
          val = in(reg) u,
          addr = in(reg) p,
          options(nostack)
        )
      },
      (2, 2) => unsafe {
        let u: u16 = core::mem::transmute_copy(&val);
        let p: *mut T = self.0.get();
        core::arch::asm!(
          "strh {val}, [{addr}]",
          val = in(reg) u,
          addr = in(reg) p,
          options(nostack)
        )
      },
      (1, 1) => unsafe {
        let u: u8 = core::mem::transmute_copy(&val);
        let p: *mut T = self.0.get();
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
