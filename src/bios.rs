#![allow(non_snake_case)]

//! Module for calls to BIOS functions.
//!
//! The BIOS functions aren't called using a normal foreign function call (eg:
//! using the `extern "C"` ABI). Instead, a special instruction `swi`
//! (software-interrupt) is executed, and an immediate data byte in the
//! instruction tells the BIOS what function to execute. Because of this, the
//! BIOS functions have a rather high calls overhead compared to a normal
//! foreign function.

use crate::irq::IrqBits;

/// `0x04`: Waits for a specific interrupt type(s) to happen.
///
/// Pauses the CPU until any of the interrupt types set in `target_irqs` to
/// occur. This can create a significant savings of the battery while you're
/// waiting, so use this function when possible.
///
/// **Important:** This function forces [`IME`](crate::irq::IME) on.
///
/// Your interrupt handler (if any) will be run before this function returns.
///
/// If none of the interrupts specified in `target_irqs` are properly configured
/// to fire then this function will loop forever without returning.
///
/// This function uses a special BIOS variable to track what interrupts have
/// occurred recently.
/// * If `ignore_existing` is set, then any previous interrupts (since
///   `IntrWait` was last called) that match `target_irqs` are *ignored* and
///   this function will wait for a new target interrupt to occur.
/// * Otherwise, any previous interrupts that match `target_irqs` will cause the
///   function to return immediately without waiting for a new interrupt.
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::t32))]
pub fn IntrWait(ignore_existing: bool, target_irqs: IrqBits) {
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm! {
      "swi #0x04",
      inout("r0") ignore_existing as u32 => _,
      inout("r1") target_irqs.0 => _,
      out("r3") _,
      options(preserves_flags),
    }
  });
}

/// As [`IntrWait`], but using the `a32` instruction set.
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
pub fn a32_IntrWait(ignore_existing: bool, target_irqs: IrqBits) {
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm! {
      "swi #(0x04 << 24)",
      inout("r0") ignore_existing as u32 => _,
      inout("r1") target_irqs.0 => _,
      out("r3") _,
      options(preserves_flags),
    }
  });
}

/// `0x05`: Builtin shorthand for [`IntrWait(true, IrqBits::VBLANK)`](IntrWait)
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::t32))]
pub fn VBlankIntrWait() {
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm! {
      "swi #0x05",
      out("r0") _,
      out("r1") _,
      out("r3") _,
      options(preserves_flags),
    }
  });
}

/// As [`VBlankIntrWait`], but using the `a32` instruction set.
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::t32))]
pub fn a32_VBlankIntrWait() {
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm! {
      "swi #(0x05 << 24)",
      out("r0") _,
      out("r1") _,
      out("r3") _,
      options(preserves_flags),
    }
  });
}

/// `0x09`: Arc tangent.
///
/// * **Returns:** The output is in the range +/- `pi/2`, but accuracy is worse
///   outside of +/- `pi/4`.
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::t32))]
pub fn ArcTan(theta: crate::i16fx14) -> crate::i16fx14 {
  let mut i = theta.to_bits();
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm! {
      "swi #0x09",
      inout("r0") i,
      out("r1") _,
      out("r3") _,
      options(pure, nomem, preserves_flags),
    }
  });
  crate::i16fx14::from_bits(i)
}

/// `0x0A`: The "2-argument arctangent" ([atan2][wp-atan2]).
///
/// [wp-atan2]: https://en.wikipedia.org/wiki/Atan2
///
/// * **Returns:** The angle of the input vector, with `u16::MAX` being
///   equivalent to `2pi`.
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::t32))]
pub fn ArcTan2(x: crate::i16fx14, y: crate::i16fx14) -> u16 {
  let x = x.to_bits();
  let y = y.to_bits();
  let output: u16;
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm! {
      "swi #0x0A",
      inout("r0") x => output,
      inout("r1") y => _,
      out("r3") _,
      options(pure, nomem, preserves_flags),
    }
  });
  output
}

/// `0x12` TODO: document this more
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::t32))]
pub unsafe fn LZ77UnCompReadNormalWrite16bit(src: *const u32, dst: *mut u16) {
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm! {
      "swi #0x12",
      inout("r0") src => _,
      inout("r1") dst => _,
      out("r3") _,
      options(preserves_flags),
    }
  });
}
