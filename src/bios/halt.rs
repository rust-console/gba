use super::*;

/// `swi 0x02`: Halts the CPU until an interrupt request occurs.
///
/// The CPU is placed into low-power mode, while other parts (video, sound,
/// timers, serial, keypad) continue to operate. This mode only terminates when
/// one of the interrupts set in [`IE`] occurs.
///
/// If [`IME`] is set then the interrupt handler will be called as normal when
/// the CPU wakes (before this function returns). Otherwise the CPU will simply
/// wake up without calling the interrupt handler.
#[inline]
pub unsafe fn Halt() {
  asm! {
    "swi #0x02",
    out("r0") _,
    out("r1") _,
    out("r3") _,
    options(preserves_flags)
  }
}

/// `swi 0x03`
#[inline]
pub unsafe fn Stop() {
  asm! {
    "swi #0x03",
    out("r0") _,
    out("r1") _,
    out("r3") _,
    options(preserves_flags)
  }
}

/// `swi 0x04`: Performs an "interrupt wait".
///
/// This function:
/// * Forces [`IME`] to be enabled.
/// * Halts the CPU (until an interrupt).
/// * Checks if `target_irqs & IntrWaitFlags` has any bits set. If so, all bits
///   set in `target_irqs` are cleared from the `IntrWaitFlags` value and the
///   function returns. Otherwise the CPU will loop and halt again.
///
/// If you want the main program to wait until after a specific type of
/// interrupt has occurred, using this function is significantly more efficient
/// then repeatedly calling [Halt] yourself.
///
/// If the `clear_old_flags` value is `true` then all `target_irqs` bits in
/// `IntrWaitFlags` will be cleared before the halt loop begins, ensuring that
/// the function only returns once a *new* interrupt of the desired type(s) has
/// occurred.
///
/// The `IME` register is left enabled even after the function returns.
///
/// Note: The `IntrWaitFlags` are automatically updated by the assembly runtime
/// whenever an interrupt occurs. Your own interrupt handler does not (and
/// should not) need to update the value itself.
#[inline]
pub unsafe fn IntrWait(clear_old_flags: bool, target_irqs: IrqBits) {
  asm!("swi #0x04",
    inout("r0") clear_old_flags as u32 => _,
    inout("r1") target_irqs.0 => _,
    out("r3") _,
    options(preserves_flags)
  )
}

/// `swi 0x05`: Performs an "interrupt wait" for a new Vertical-blank
/// Interrupt.
///
/// This is effectively just an alternate way to write
/// ```no_run
/// IntrWait(true, IrqBits::V_BLANK);
/// ```
#[inline]
pub fn VBlankIntrWait() {
  unsafe {
    asm!(
      "swi #0x05",
      out("r0") _,
      out("r1") _,
      out("r3") _,
      options(preserves_flags),
    )
  };
}
