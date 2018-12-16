//! This module contains wrappers for all GBA BIOS function calls.
//!
//! A GBA BIOS call has significantly more overhead than a normal function call,
//! so think carefully before using them too much.
//!
//! The actual content of each function here is generally a single inline asm
//! instruction to invoke the correct BIOS function (`swi x`, with `x` being
//! whatever value is necessary for that function). Some functions also perform
//! necessary checks to save you from yourself, such as not dividing by zero.

/// (`swi 0x00`) SoftReset the device.
///
/// This function does not ever return.
///
/// Instead, it clears the top `0x200` bytes of IWRAM (containing stacks, and
/// BIOS IRQ vector/flags), re-initializes the system, supervisor, and irq stack
/// pointers (new values listed below), sets `r0` through `r12`, `LR_svc`,
/// `SPSR_svc`, `LR_irq`, and `SPSR_irq` to zero, and enters system mode. The
/// return address is loaded into `r14` and then the function jumps there with
/// `bx r14`.
///
/// * sp_svc: `0x300_7FE0`
/// * sp_irq: `0x300_7FA0`
/// * sp_sys: `0x300_7F00`
/// * Zero-filled Area: `0x300_7E00` to `0x300_7FFF`
/// * Return Address: Depends on the 8-bit flag value at `0x300_7FFA`. In either
///   case execution proceeds in ARM mode.
///   * zero flag: `0x800_0000` (ROM), which for our builds means that the
///     `crt0` program to execute (just like with a fresh boot), and then
///     control passes into `main` and so on.
///   * non-zero flag: `0x200_0000` (RAM), This is where a multiboot image would
///     go if you were doing a multiboot thing. However, this project doesn't
///     support multiboot at the moment. You'd need an entirely different build
///     pipeline because there's differences in header format and things like
///     that. Perhaps someday, but probably not even then. Submit the PR for it
///     if you like!
///
/// ## Safety
///
/// This functions isn't ever unsafe to the current iteration of the program.
/// However, because not all memory is fully cleared you theoretically could
/// threaten the _next_ iteration of the program that runs. I'm _fairly_
/// convinced that you can't actually use this to force purely safe code to
/// perform UB, but such a scenario might exist.
#[inline(always)]
pub unsafe fn soft_reset() -> ! {
  asm!(/* ASM */ "swi 0x00"
      :/* OUT */ // none
      :/* INP */ // none
      :/* CLO */ // none
      :/* OPT */ "volatile"
  );
  core::hint::unreachable_unchecked()
}

/// (`swi 0x01`) RegisterRamReset.
///
/// Clears the portions of memory given by the `flags` value, sets the Display
/// Control Register to `0x80` (forced blank and nothing else), then returns.
///
/// * Flag bits:
///   0) Clears the 256k of EWRAM (don't use if this is where your function call
///      will return to!)
///   1) Clears the 32k of IWRAM _excluding_ the last `0x200` bytes (see also:
///      the `soft_reset` function)
///   2) Clears all Palette data
///   3) Clears all VRAM
///   4) Clears all OAM (reminder: a zeroed object isn't disabled!)
///   5) Reset SIO registers (resets them to general purpose mode)
///   6) Reset Sound registers
///   7) Reset all IO registers _other than_ SIO and Sound
///
/// **Bug:** The LSB of `SIODATA32` is always zeroed, even if bit 5 was not
/// enabled. This is sadly a bug in the design of the GBA itself.
///
/// ## Safety
///
/// It is generally a safe operation to suddenly clear any part of the GBA's
/// memory, except in the case that you were executing out of EWRAM and clear
/// that. If you do then you return to nothing and have a bad time.
#[inline(always)]
pub unsafe fn register_ram_reset(flags: u8) {
  asm!(/* ASM */ "swi 0x01"
      :/* OUT */ // none
      :/* INP */ "{r0}"(flags)
      :/* CLO */ // none
      :/* OPT */ "volatile"
  );
}
//TODO(lokathor): newtype this flag business.

/// (`swi 0x06`) Software Division and Remainder.
///
/// ## Panics
///
/// If the denominator is 0.
#[inline(always)]
pub fn div_rem(numerator: i32, denominator: i32) -> (i32, i32) {
  assert!(denominator != 0);
  let div_out: i32;
  let rem_out: i32;
  unsafe {
    asm!(/* ASM */ "swi 0x06"
        :/* OUT */ "={r0}"(div_out), "={r1}"(rem_out)
        :/* INP */ "{r0}"(numerator), "{r1}"(denominator)
        :/* CLO */ "r3"
        :/* OPT */
    );
  }
  (div_out, rem_out)
}

/// As `div_rem`, but keeping only the `div` part.
#[inline(always)]
pub fn div(numerator: i32, denominator: i32) -> i32 {
  div_rem(numerator, denominator).0
}

/// As `div_rem`, but keeping only the `rem` part.
#[inline(always)]
pub fn rem(numerator: i32, denominator: i32) -> i32 {
  div_rem(numerator, denominator).1
}

/// (`swi 0x08`) Integer square root.
///
/// If you want more fractional precision, you can shift your input to the left
/// by `2n` bits to get `n` more bits of fractional precision in your output.
#[inline(always)]
pub fn sqrt(val: u32) -> u16 {
  let out: u16;
  unsafe {
    asm!(/* ASM */ "swi 0x08"
        :/* OUT */ "={r0}"(out)
        :/* INP */ "{r0}"(val)
        :/* CLO */ "r1", "r3"
        :/* OPT */
    );
  }
  out
}

/// (`swi 0x09`) Gives the arctangent of `theta`.
///
/// The input format is 1 bit for sign, 1 bit for integral part, 14 bits for
/// fractional part.
///
/// Accuracy suffers if `theta` is less than `-pi/4` or greater than `pi/4`.
#[inline(always)]
pub fn atan(theta: i16) -> i16 {
  let out: i16;
  unsafe {
    asm!(/* ASM */ "swi 0x09"
        :/* OUT */ "={r0}"(out)
        :/* INP */ "{r0}"(theta)
        :/* CLO */ "r1", "r3"
        :/* OPT */
    );
  }
  out
}

/// (`swi 0x0A`) Gives the atan2 of `y` over `x`.
///
/// The output `theta` value maps into the range `[0, 2pi)`, or `0 .. 2pi` if
/// you prefer Rust's range notation.
///
/// `y` and `x` use the same format as with `atan`: 1 bit for sign, 1 bit for
/// integral, 14 bits for fractional.
#[inline(always)]
pub fn atan2(y: i16, x: i16) -> u16 {
  let out: u16;
  unsafe {
    asm!(/* ASM */ "swi 0x0A"
        :/* OUT */ "={r0}"(out)
        :/* INP */ "{r0}"(x), "{r1}"(y)
        :/* CLO */ "r3"
        :/* OPT */
    );
  }
  out
}
