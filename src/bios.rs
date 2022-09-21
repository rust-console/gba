#![allow(non_snake_case)]

use crate::interrupts::IrqBits;

/// `0x04` Waits for a specific interrupt type(s) to happen.
///
/// **Important:** This function forces [`IME`](crate::mmio::IME) on.
///
/// Waits for *any* of the interrupt types set in `target_irqs` to occur. Your
/// interrupt handler (if any) will be run before this function returns.
///
/// This function uses a special BIOS variable to track what interrupts have
/// occured recently.
/// * If `ignore_existing` is set, then any previous interrupts (since
///   `IntrWait` was last called) that match `target_irqs` are *ignored* and
///   this function will wait for a new target interrupt to occur.
/// * Otherwise, any previous interrupts that match `target_irqs` will cause the
///   function to return immediately without waiting for a new interrupt.
#[inline]
#[instruction_set(arm::t32)]
pub fn IntrWait(ignore_existing: bool, target_irqs: IrqBits) {
  unsafe {
    core::arch::asm! {
      "swi #0x04",
      inout("r0") ignore_existing as u32 => _,
      inout("r1") target_irqs.to_u16() => _,
      out("r3") _,
      options(preserves_flags),
    }
  };
}

/// `0x05` Builtin shorthand for [`IntrWait(true, IrqBits::VBLANK)`](IntrWait)
#[inline]
#[instruction_set(arm::t32)]
pub fn VBlankIntrWait() {
  unsafe {
    core::arch::asm! {
      "swi #0x05",
      out("r0") _,
      out("r1") _,
      out("r3") _,
      options(preserves_flags),
    }
  };
}
