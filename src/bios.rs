#![allow(non_snake_case)]

//! This module allows access to the "bios functions".
//!
//! All bios function is accessed through a Software Interrupt (SWI). This means
//! that they're significantly more costly to call than a normal function
//! because the SWI handler has to perform some common setup and then call to
//! the correct bios function. Still, some of the BIOS functions are useful
//! enough to justify this additional overhead.
//!
//! | SWI | GbaTek Name |
//! |:-:|:-|
//! | `0x00` | [SoftReset] |
//! | `0x01` | [RegisterRamReset] |
//! | `0x02` | [Halt] |
//! | `0x03` | `Stop` is unimplemented at this time |
//! | `0x04` | [IntrWait] |
//! | `0x05` | [VBlankIntrWait] |

use core::arch::asm;

use crate::{
  macros::{const_new, u8_bool_field},
  IrqBits,
};

/// (`swi 0x00`) Performs a "soft reset" of the device.
///
/// This resets the following memory and registers:
/// * `0x300_7E00` ..= `0x300_7FFF`: zeroed
/// * `r0` ..= `r12`: zeroed
/// * `sp_usr`: `0x300_7F00`
/// * `sp_irq`: `0x300_7FA0`
/// * `sp_svc`: `0x300_7FE0`
/// * `lr_svc`, `lr_irq` : zeroed
/// * `spsr_svc`, `spsr_irq`: zeroed
///
/// Then control jumps to a starting address based on the `u8` value that was at
/// `0x0300_7FFA` when the function is first called:
/// * zero: `0x0800_0000` (ROM)
/// * non-zero: `0x0200_0000` (EWRAM)
pub fn SoftReset() -> ! {
  unsafe { asm!("swi 0x00", options(noreturn)) }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct RegisterRamResetFlags(pub(crate) u8);
impl RegisterRamResetFlags {
  const_new!();
  u8_bool_field!(0, ewram, with_ewram);
  u8_bool_field!(1, iwram, with_iwram);
  u8_bool_field!(2, palram, with_palram);
  u8_bool_field!(3, vram, with_vram);
  u8_bool_field!(4, oam, with_oam);
  u8_bool_field!(5, sio, with_sio);
  u8_bool_field!(6, sound, with_sound);
  u8_bool_field!(7, all_other_io, with_all_other_io);
}

/// (`swi 0x01`) Resets IO registers and/or RAM
///
/// * Note that if the IWRAM flag is used it doesn't reset the final `0x200`
///   bytes of IWRAM. Instead, those bytes are reset during a call to the
///   [`SoftReset`] function.
/// * BIOS Bug: Data in `SIODATA32` is always destroyed, even if the `sio` flag
///   is not set.
///
/// ## Safety
/// * Using this to reset EWRAM or IWRAM is **very** ill advised and you
///   probably just shouldn't do it.
#[inline]
pub unsafe fn RegisterRamReset(flags: RegisterRamResetFlags) {
  asm!("swi 0x01",
    inout("r0") flags.0 => _,
    out("r1") _,
    out("r3") _,
    options(preserves_flags)
  )
}

/// (`swi 0x02`) Halts the CPU until an interrupt request occurs.
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
  asm!("swi 0x02",
    out("r0") _,
    out("r1") _,
    out("r3") _,
    options(preserves_flags)
  )
}

// TODO: Add `Stop` (0x03), which is easy to implement, but it's unclear what to
// write as the documentation.

/// (`swi 0x04`) Performs an "interrupt wait".
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
  asm!("swi 0x02",
    inout("r0") clear_old_flags as u32 => _,
    inout("r1") target_irqs.0 => _,
    out("r3") _,
    options(preserves_flags)
  )
}

/// (`swi 0x05`) Performs an "interrupt wait" for a new Vertical-blank
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

#[repr(C)]
pub struct UnPackInfo {
  pub src_len: u16,
  pub src_bit_width: u8,
  pub dest_bit_width: u8,
  pub offset_and_flags: u32,
}

/// `swi #0x10`
#[inline]
pub unsafe fn BitUnPack(src: *const u8, dest: *mut u32, info: &UnPackInfo) {
  asm!(
    "swi #0x10",
    inout("r0") src => _,
    inout("r1") dest => _,
    in("r2") info,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x12`
#[inline]
pub unsafe fn LZ77UnCompReadNormalWrite16bit(src: *const u32, dest: *mut u32) {
  asm!(
    "swi #0x12",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x13`
#[inline]
pub unsafe fn HuffUnCompReadNormal(src: *const u32, dest: *mut u32) {
  asm!(
    "swi #0x13",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}
