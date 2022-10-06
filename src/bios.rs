#![allow(non_snake_case)]

//! The GBA's BIOS provides limited built-in utility functions.
//!
//! BIOS functions are accessed with an `swi` instruction to perform a software
//! interrupt. This means that there's a *significant* overhead for a BIOS call
//! (tens of cycles) compared to a normal function call (3 cycles, or even none
//! of the function ends up inlined). Despite this higher cost, some bios
//! functions are useful enough to justify the overhead.

use crate::interrupts::IrqBits;

// Note(Lokathor): All `swi` calls will preserve the flags. You should generally
// not use any other inline-asm options with `swi` calls.

/// `0x04`: Waits for a specific interrupt type(s) to happen.
///
/// Pauses the CPU until any of the interrupt types set in `target_irqs` to
/// occur. This can create a significant savings of the battery while you're
/// waiting, so use this function when possible.
///
/// **Important:** This function forces [`IME`](crate::mmio::IME) on.
///
/// Your interrupt handler (if any) will be run before this function returns.
///
/// If none of the interrupts specified in `target_irqs` are properly configured
/// to fire then this function will loop forever without returning.
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

/// `0x05`: Builtin shorthand for [`IntrWait(true, IrqBits::VBLANK)`](IntrWait)
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

/// Used to provide info to a call of the [`BitUnPack`] function.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BitUnpackInfo {
  /// Number of bytes in the source buffer
  pub src_byte_len: u16,
  /// Bits per source element: 1, 2, 4, or 8.
  pub src_elem_width: u8,
  /// Bits per destination element: 1, 2, 4, 8, 16, or 32.
  pub dest_elem_width: u8,
  /// Bits `0..=30` are the offset value added to all non-zero elements.
  ///
  /// If bit `31` is set then offset value is *also* added to zero elements.
  pub offset_and_touch_zero: u32,
}

/// `0x10`: Copy data from `src` to `dest` while increasing the bit depth of the
/// elements copied.
///
/// * This reads one byte at a time from `src`. Each source byte holds 1 or more
///   source elements, depending on the source bit depth you specify. Elements
///   within a byte are packed from low bit to high bit.
/// * Each non-zero source element has the offset added to it. If the source
///   element is zero and the "touch zero" flag is set, then that source element
///   will also have the offset added to it. This creates a destination element.
/// * Destination elements are collected into the output `u32` buffer one at a
///   time, from low bit to high bit. If a source element plus the offset
///   produces a value larger than the destination element bit size this will
///   corrupt any following destination elements within the buffer. When the
///   buffer has 32 bits held then it's written to the destination pointer.
/// * When the source byte read has no more source elements remaining the source
///   pointer will advance and `src_byte_len` will go down by 1. When
///   `src_byte_len` goes to 0 the function's main loop will break and return.
///   If there was partial output in the `u32` buffer when the function's
///   primary loop ends this data will be lost.
///
/// ## Safety
/// * The `info` provided must correctly describe the data.
/// * `src` must be readable for the number of **bytes** specified
/// * `dest` must be writable for the number of **words** that the source
///   buffer, source depth, and destination depth will total up to.
/// * `dest` must be 4 byte aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn BitUnPack(src: *const u8, dest: *mut u32, info: &BitUnpackInfo) {
  core::arch::asm! {
    "swi #0x10",
    inout("r0") src => _,
    inout("r1") dest => _,
    inout("r2") info => _,
    out("r3") _,
    options(preserves_flags),
  }
}
