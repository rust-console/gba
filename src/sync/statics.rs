#![cfg_attr(not(target_arch = "arm"), allow(unused_variables))]

use crate::sync::with_irqs_disabled;
use core::{
  cell::UnsafeCell,
  mem::{align_of, size_of, MaybeUninit},
  ptr,
};

/// The internal function for replacing a `Copy` (really `!Drop`) value in a
/// [`Static`]. This uses assembly to use an `stmia` instruction to ensure
/// an IRQ cannot occur during the write operation.
#[cfg(target_arch = "arm")]
unsafe fn transfer<T: Copy>(dst: *mut T, src: *const T) {
  let align = align_of::<T>();
  let size = size_of::<T>();
  if size == 0 {
    // Do nothing with ZSTs. Obviously.
  } else if size <= 16 && align % 4 == 0 {
    // We can do an 4-byte aligned transfer up to 16 bytes.
    transfer_align4_thumb(dst, src);
  } else if size <= 36 && align % 4 == 0 {
    // We can do the same up to 36 bytes, but we need to switch to ARM.
    transfer_align4_arm(dst, src);
  } else if size <= 2 && align % 2 == 0 {
    // We can do a 2-byte aligned transfer up to 2 bytes.
    asm!(
      "ldrh {2},[{0}]",
      "strh {2},[{1}]",
      in(reg) src, in(reg) dst, out(reg) _,
    )
  } else if size == 1 {
    // We can do a simple byte copy.
    asm!(
      "ldrb {2},[{0}]",
      "strb {2},[{1}]",
      in(reg) src, in(reg) dst, out(reg) _,
    )
  } else {
    // When we don't have an optimized path, we just disable IRQs.
    with_irqs_disabled(|| ptr::write_volatile(dst, ptr::read_volatile(src)));
  }
}

#[cfg(target_arch = "arm")]
#[allow(unused_assignments)]
unsafe fn transfer_align4_thumb<T: Copy>(mut dst: *mut T, mut src: *const T) {
  let size = size_of::<T>();
  if size <= 4 {
    // We use assembly here regardless to just do the word aligned copy. This
    // ensures it's done with a single ldr/str instruction.
    asm!(
      "ldr {2},[{0}]",
      "str {2},[{1}]",
      inout(reg) src, in(reg) dst, out(reg) _,
    )
  } else if size <= 8 {
    // Starting at size == 5, we begin using ldmia/stmia to load/save multiple
    // words in one instruction, avoiding IRQs from interrupting our operation.
    asm!(
      "ldmia {0}!, {{r2-r3}}",
      "stmia {1}!, {{r2-r3}}",
      inout(reg) src, inout(reg) dst,
      out("r2") _, out("r3") _,
    )
  } else if size <= 12 {
    asm!(
      "ldmia {0}!, {{r2-r4}}",
      "stmia {1}!, {{r2-r4}}",
      inout(reg) src, inout(reg) dst,
      out("r2") _, out("r3") _, out("r4") _,
    )
  } else if size <= 16 {
    asm!(
      "ldmia {0}!, {{r2-r5}}",
      "stmia {1}!, {{r2-r5}}",
      inout(reg) src, inout(reg) dst,
      out("r2") _, out("r3") _, out("r4") _, out("r5") _,
    )
  } else {
    unimplemented!("This should be done via transfer_arm.");
  }
}

#[cfg(target_arch = "arm")]
#[instruction_set(arm::a32)]
#[allow(unused_assignments)]
unsafe fn transfer_align4_arm<T: Copy>(mut dst: *mut T, mut src: *const T) {
  let size = size_of::<T>();
  if size <= 16 {
    unimplemented!("This should be done via transfer_thumb.");
  } else if size <= 20 {
    // Starting at size == 20, we have to switch to ARM due to lack of
    // accessible registers in THUMB mode.
    asm!(
      "ldmia {0}!, {{r2-r5,r8}}",
      "stmia {1}!, {{r2-r5,r8}}",
      inout(reg) src, inout(reg) dst,
      out("r2") _, out("r3") _, out("r4") _, out("r5") _, out("r8") _,
    )
  } else if size <= 24 {
    asm!(
      "push {{r9}}",
      "ldmia {0}!, {{r2-r5,r8-r9}}",
      "stmia {1}!, {{r2-r5,r8-r9}}",
      "pop {{r9}}",
      inout(reg) src, inout(reg) dst,
      out("r2") _, out("r3") _, out("r4") _, out("r5") _, out("r8") _,
    )
  } else if size <= 28 {
    asm!(
      "push {{r9}}",
      "ldmia {0}!, {{r2-r5,r8-r10}}",
      "stmia {1}!, {{r2-r5,r8-r10}}",
      "pop {{r9}}",
      inout(reg) src, inout(reg) dst,
      out("r2") _, out("r3") _, out("r4") _, out("r5") _, out("r8") _,
      out("r10") _,
    )
  } else if size <= 32 {
    asm!(
      "push {{r9}}",
      "ldmia {0}!, {{r2-r5,r8-r10,r12}}",
      "stmia {1}!, {{r2-r5,r8-r10,r12}}",
      "pop {{r9}}",
      inout(reg) src, inout(reg) dst,
      out("r2") _, out("r3") _, out("r4") _, out("r5") _, out("r8") _,
      out("r10") _, out("r12") _,
    )
  } else if size <= 36 {
    asm!(
      "push {{r9}}",
      "ldmia {0}!, {{r2-r5,r8-r10,r12,r14}}",
      "stmia {1}!, {{r2-r5,r8-r10,r12,r14}}",
      "pop {{r9}}",
      inout(reg) src, inout(reg) dst,
      out("r2") _, out("r3") _, out("r4") _, out("r5") _, out("r8") _,
      out("r10") _, out("r12") _, out("r14") _,
    )
  } else {
    unimplemented!("Copy too large for use of ldmia/stmia.");
  }
}

/// The internal function for swapping the current value of a [`Static`] with
/// another value.
#[cfg(target_arch = "arm")]
unsafe fn exchange<T>(dst: *mut T, src: *const T) -> T {
  let align = align_of::<T>();
  let size = size_of::<T>();
  if size == 0 {
    // Do nothing with ZSTs.
    ptr::read(dst)
  } else if size <= 4 && align % 4 == 0 {
    // Swap a single word with the SWP instruction.
    let val = ptr::read(src as *const u32);
    let new_val = exchange_align4_arm(dst, val);
    ptr::read(&new_val as *const _ as *const T)
  } else if size == 1 {
    // Swap a byte with the SWPB instruction.
    let val = ptr::read(src as *const u8);
    let new_val = exchange_align1_arm(dst, val);
    ptr::read(&new_val as *const _ as *const T)
  } else {
    // fallback
    with_irqs_disabled(|| {
      let cur = ptr::read_volatile(dst);
      ptr::write_volatile(dst, ptr::read_volatile(src));
      cur
    })
  }
}

#[cfg(target_arch = "arm")]
#[instruction_set(arm::a32)]
unsafe fn exchange_align4_arm<T>(dst: *mut T, i: u32) -> u32 {
  let out;
  asm!("swp {2}, {1}, [{0}]", in(reg) dst, in(reg) i, lateout(reg) out);
  out
}

#[cfg(target_arch = "arm")]
#[instruction_set(arm::a32)]
unsafe fn exchange_align1_arm<T>(dst: *mut T, i: u8) -> u8 {
  let out;
  asm!("swpb {2}, {1}, [{0}]", in(reg) dst, in(reg) i, lateout(reg) out);
  out
}

#[cfg(not(target_arch = "arm"))]
unsafe fn exchange<T>(dst: *mut T, src: *const T) -> T {
  unimplemented!("This function is not supported on this target.")
}

#[cfg(not(target_arch = "arm"))]
unsafe fn transfer<T: Copy>(dst: *mut T, src: *const T) {
  unimplemented!("This function is not supported on this target.")
}

/// A helper that implements static variables.
///
/// It ensures that even if you use the same static variable in both an IRQ
/// and normal code, the IRQ will never observe an invalid value of the
/// variable.
///
/// This type only works with owned values. If you need to work with borrows,
/// consider using [`super::Mutex`] instead.
///
/// ## Performance
///
/// Writing or reading from a static variable is efficient under the following
/// conditions:
///
/// * The type is aligned to 4 bytes and can be stored in 36 bytes or less.
/// * The type is aligned to 2 bytes and can be stored in 2 bytes.
/// * The type is can be stored in a single byte.
///
/// Replacing the current value of the static variable is efficient under the
/// following conditions:
///
/// * The type is aligned to 4 bytes and can be stored in 4 bytes or less.
/// * The type is can be stored in a single byte.
///
/// When these conditions are not met, static variables are handled using a
/// fallback routine that disables IRQs and does a normal copy. This can be
/// dangerous as disabling IRQs can cause your program to miss out on important
/// interrupts such as V-Blank.
///
/// Consider using [`super::Mutex`] instead if you need to use a large amount of
/// operations that would cause IRQs to be disabled. Also consider using
/// `#[repr(align(4))]` to force proper alignment for your type.
pub struct Static<T> {
  data: UnsafeCell<T>,
}
impl<T> Static<T> {
  /// Creates a new static variable.
  pub const fn new(val: T) -> Self {
    Static { data: UnsafeCell::new(val) }
  }

  /// Replaces the current value of the static variable with another, and
  /// returns the old value.
  pub fn replace(&self, val: T) -> T {
    unsafe { exchange(self.data.get(), &val) }
  }

  /// Extracts the interior value of the static variable.
  pub fn into_inner(self) -> T {
    self.data.into_inner()
  }
}
impl<T: Copy> Static<T> {
  /// Writes a new value into this static variable.
  pub fn write(&self, val: T) {
    unsafe { transfer(self.data.get(), &val) }
  }

  /// Reads a value from this static variable.
  pub fn read(&self) -> T {
    unsafe {
      let mut out: MaybeUninit<T> = MaybeUninit::uninit();
      transfer(out.as_mut_ptr(), self.data.get());
      out.assume_init()
    }
  }
}
impl<T: Default> Default for Static<T> {
  fn default() -> Self {
    Static::new(T::default())
  }
}
unsafe impl<T> Send for Static<T> {}
unsafe impl<T> Sync for Static<T> {}
