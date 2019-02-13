//! This module contains wrappers for all GBA BIOS function calls.
//!
//! A GBA BIOS call has significantly more overhead than a normal function call,
//! so think carefully before using them too much.
//!
//! The actual content of each function here is generally a single inline asm
//! instruction to invoke the correct BIOS function (`swi x`, with `x` being
//! whatever value is necessary for that function). Some functions also perform
//! necessary checks to save you from yourself, such as not dividing by zero.

#![cfg_attr(not(all(target_vendor = "nintendo", target_env = "agb")), allow(unused_variables))]

use super::*;

//TODO: ALL functions in this module should have `if cfg!(test)` blocks. The
//functions that never return must panic, the functions that return nothing
//should just do so, and the math functions should just return the correct math
//I guess.

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
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    asm!(/* ASM */ "swi 0x00"
        :/* OUT */ // none
        :/* INP */ // none
        :/* CLO */ // none
        :/* OPT */ "volatile"
    );
    core::hint::unreachable_unchecked()
  }
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
pub unsafe fn register_ram_reset(flags: RegisterRAMResetFlags) {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    asm!(/* ASM */ "swi 0x01"
        :/* OUT */ // none
        :/* INP */ "{r0}"(flags.0)
        :/* CLO */ // none
        :/* OPT */ "volatile"
    );
  }
}
newtype! {
  /// Flags for use with `register_ram_reset`.
  RegisterRAMResetFlags, u8
}
#[allow(missing_docs)]
impl RegisterRAMResetFlags {
  phantom_fields! {
    self.0: u8,
    ewram: 0,
    iwram: 1,
    palram: 2,
    vram: 3,
    oam: 4,
    sio: 5,
    sound: 6,
    other_io: 7,
  }
}

/// (`swi 0x02`) Halts the CPU until an interrupt occurs.
///
/// Components _other than_ the CPU continue to function. Halt mode ends when
/// any enabled interrupt triggers.
#[inline(always)]
pub fn halt() {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x02"
          :/* OUT */ // none
          :/* INP */ // none
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}

/// (`swi 0x03`) Stops the CPU as well as most other components.
///
/// Stop mode must be stopped by an interrupt, but can _only_ be stopped by a
/// Keypad, Game Pak, or General-Purpose-SIO interrupt.
///
/// Before going into stop mode you should manually disable video and sound (or
/// they will continue to consume power), and you should also disable any other
/// optional externals such as rumble and infra-red.
#[inline(always)]
pub fn stop() {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x03"
          :/* OUT */ // none
          :/* INP */ // none
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}

/// (`swi 0x04`) "IntrWait", similar to halt but with more options.
///
/// * The first argument controls if you want to ignore all current flags and
///   wait until a new flag is set.
/// * The second argument is what flags you're waiting on (same format as the
///   IE/IF registers).
///
/// If you're trying to handle more than one interrupt at once this has less
/// overhead than calling `halt` over and over.
///
/// When using this routing your interrupt handler MUST update the BIOS
/// Interrupt Flags `0x300_7FF8` in addition to the usual interrupt
/// acknowledgement.
#[inline(always)]
pub fn interrupt_wait(ignore_current_flags: bool, target_flags: u16) {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x04"
          :/* OUT */ // none
          :/* INP */ "{r0}"(ignore_current_flags), "{r1}"(target_flags)
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}
//TODO(lokathor): newtype this flag business.

/// (`swi 0x05`) "VBlankIntrWait", VBlank Interrupt Wait.
///
/// This is as per `interrupt_wait(true, 1)` (aka "wait for a new vblank"). You
/// must follow the same guidelines that `interrupt_wait` outlines.
#[inline(always)]
pub fn vblank_interrupt_wait() {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x05"
          :/* OUT */ // none
          :/* INP */ // none
          :/* CLO */ "r0", "r1" // both set to 1 by the routine
          :/* OPT */ "volatile"
      );
    }
  }
}

/// (`swi 0x06`) Software Division and Remainder.
///
/// ## Panics
///
/// If the denominator is 0.
#[inline(always)]
pub fn div_rem(numerator: i32, denominator: i32) -> (i32, i32) {
  assert!(denominator != 0);
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    (numerator / denominator, numerator % denominator)
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
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
}

/// As `div_rem`, keeping only the `div` output.
#[inline(always)]
pub fn div(numerator: i32, denominator: i32) -> i32 {
  div_rem(numerator, denominator).0
}

/// As `div_rem`, keeping only the `rem` output.
#[inline(always)]
pub fn rem(numerator: i32, denominator: i32) -> i32 {
  div_rem(numerator, denominator).1
}

// (`swi 0x07`): We deliberately don't implement this one. It's the same as DIV
// but with reversed arguments, so it just runs 3 cycles slower as it does the
// swap.

/// (`swi 0x08`) Integer square root.
///
/// If you want more fractional precision, you can shift your input to the left
/// by `2n` bits to get `n` more bits of fractional precision in your output.
#[inline(always)]
pub fn sqrt(val: u32) -> u16 {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    0 // TODO: simulate this properly when not on GBA
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
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
}

/// (`swi 0x09`) Gives the arctangent of `theta`.
///
/// The input format is 1 bit for sign, 1 bit for integral part, 14 bits for
/// fractional part.
///
/// Accuracy suffers if `theta` is less than `-pi/4` or greater than `pi/4`.
#[inline(always)]
pub fn atan(theta: i16) -> i16 {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    0 // TODO: simulate this properly when not on GBA
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
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
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    0 // TODO: simulate this properly when not on GBA
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
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
}

/// (`swi 0x0B`) "CpuSet", `u16` memory copy.
///
/// * `count` is the number of `u16` values to copy (20 bits or less)
/// * `fixed_source` argument, if true, turns this copying routine into a
///   filling routine.
///
/// ## Safety
///
/// * Both pointers must be aligned
#[inline(always)]
pub unsafe fn cpu_set16(src: *const u16, dest: *mut u16, count: u32, fixed_source: bool) {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    let control = count + ((fixed_source as u32) << 24);
    asm!(/* ASM */ "swi 0x0B"
        :/* OUT */ // none
        :/* INP */ "{r0}"(src), "{r1}"(dest), "{r2}"(control)
        :/* CLO */ // none
        :/* OPT */ "volatile"
    );
  }
}

/// (`swi 0x0B`) "CpuSet", `u32`  memory copy/fill.
///
/// * `count` is the number of `u32` values to copy (20 bits or less)
/// * `fixed_source` argument, if true, turns this copying routine into a
///   filling routine.
///
/// ## Safety
///
/// * Both pointers must be aligned
#[inline(always)]
pub unsafe fn cpu_set32(src: *const u32, dest: *mut u32, count: u32, fixed_source: bool) {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    let control = count + ((fixed_source as u32) << 24) + (1 << 26);
    asm!(/* ASM */ "swi 0x0B"
        :/* OUT */ // none
        :/* INP */ "{r0}"(src), "{r1}"(dest), "{r2}"(control)
        :/* CLO */ // none
        :/* OPT */ "volatile"
    );
  }
}

/// (`swi 0x0C`) "CpuFastSet", copies memory in 32 byte chunks.
///
/// * The `count` value is the number of `u32` values to transfer (20 bits or
///   less), and it's rounded up to the nearest multiple of 8 words.
/// * The `fixed_source` argument, if true, turns this copying routine into a
///   filling routine.
///
/// ## Safety
///
/// * Both pointers must be aligned
#[inline(always)]
pub unsafe fn cpu_fast_set(src: *const u32, dest: *mut u32, count: u32, fixed_source: bool) {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    let control = count + ((fixed_source as u32) << 24);
    asm!(/* ASM */ "swi 0x0C"
        :/* OUT */ // none
        :/* INP */ "{r0}"(src), "{r1}"(dest), "{r2}"(control)
        :/* CLO */ // none
        :/* OPT */ "volatile"
    );
  }
}

/// (`swi 0x0C`) "GetBiosChecksum" (Undocumented)
///
/// Though we usually don't cover undocumented functionality, this one can make
/// it into the crate.
///
/// The function computes the checksum of the BIOS data. You should get either
/// `0xBAAE_187F` (GBA / GBA SP) or `0xBAAE_1880` (DS in GBA mode). If you get
/// some other value I guess you're probably running on an emulator that just
/// broke the fourth wall.
pub fn get_bios_checksum() -> u32 {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    let out: u32;
    unsafe {
      asm!(/* ASM */ "swi 0x0D"
          :/* OUT */ "={r0}"(out)
          :/* INP */ // none
          :/* CLO */ // none
          :/* OPT */ // none
      );
    }
    out
  }
}

// TODO: these things will require that we build special structs

//BgAffineSet
//ObjAffineSet
//BitUnPack
//LZ77UnCompReadNormalWrite8bit
//LZ77UnCompReadNormalWrite16bit
//HuffUnCompReadNormal
//RLUnCompReadNormalWrite8bit
//Diff8bitUnFilterWrite8bit
//Diff8bitUnFilterWrite16bit
//Diff16bitUnFilter

/// (`swi 0x19`) "SoundBias", adjusts the volume level to a new level.
///
/// This increases or decreases the current level of the `SOUNDBIAS` register
/// (with short delays) until at the new target level. The upper bits of the
/// register are unaffected.
///
/// The final sound level setting will be `level` * `0x200`.
pub fn sound_bias(level: u32) {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x19"
          :/* OUT */ // none
          :/* INP */ "{r0}"(level)
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}

//SoundDriverInit

/// (`swi 0x1B`) "SoundDriverMode", sets the sound driver operation mode.
///
/// The `mode` input uses the following flags and bits:
///
/// * Bits 0-6: Reverb value
/// * Bit 7: Reverb Enable
/// * Bits 8-11: Simultaneously-produced channel count (default=8)
/// * Bits 12-15: Master Volume (1-15, default=15)
/// * Bits 16-19: Playback Frequency Index (see below, default=4)
/// * Bits 20-23: "Final number of D/A converter bits (8-11 = 9-6bits, def. 9=8bits)" TODO: what the hek?
/// * Bits 24 and up: Not used
///
/// The frequency index selects a frequency from the following array:
/// * 0: 5734
/// * 1: 7884
/// * 2: 10512
/// * 3: 13379
/// * 4: 15768
/// * 5: 18157
/// * 6: 21024
/// * 7: 26758
/// * 8: 31536
/// * 9: 36314
/// * 10: 40137
/// * 11: 42048
pub fn sound_driver_mode(mode: u32) {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x1B"
          :/* OUT */ // none
          :/* INP */ "{r0}"(mode)
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}
//TODO(lokathor): newtype this mode business.

/// (`swi 0x1C`) "SoundDriverMain", main of the sound driver
///
/// You should call `SoundDriverVSync` immediately after the vblank interrupt
/// fires.
///
/// "After that, this routine is called after BG and OBJ processing is
/// executed." --what?
#[inline(always)]
pub fn sound_driver_main() {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x1C"
          :/* OUT */ // none
          :/* INP */ // none
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}

/// (`swi 0x1D`) "SoundDriverVSync", resets the sound DMA.
///
/// The timing is critical, so you should call this _immediately_ after the
/// vblank interrupt (every 1/60th of a second).
#[inline(always)]
pub fn sound_driver_vsync() {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x1D"
          :/* OUT */ // none
          :/* INP */ // none
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}

/// (`swi 0x1E`) "SoundChannelClear", clears the direct sound channels and stops
/// the sound.
///
/// "This function may not operate properly when the library which expands the
/// sound driver feature is combined afterwards. In this case, do not use it."
/// --what?
#[inline(always)]
pub fn sound_channel_clear() {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x1E"
          :/* OUT */ // none
          :/* INP */ // none
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}

//MidiKey2Freq
//MultiBoot

/// (`swi 0x28`) "SoundDriverVSyncOff", disables sound
///
/// If you can't use vblank interrupts to ensure that `sound_driver_vsync` is
/// called every 1/60th of a second for any reason you must use this function to
/// stop sound DMA. Otherwise the DMA will overrun its buffer and cause random
/// noise.
#[inline(always)]
pub fn sound_driver_vsync_off() {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x28"
          :/* OUT */ // none
          :/* INP */ // none
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}

/// (`swi 0x29`) "SoundDriverVSyncOn", enables sound that was stopped by
/// `sound_driver_vsync_off`.
///
/// Restarts sound DMA system. After restarting the sound you must have a vblank
/// interrupt followed by a `sound_driver_vsync` within 2/60th of a second.
#[inline(always)]
pub fn sound_driver_vsync_on() {
  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  {
    unimplemented!()
  }
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    unsafe {
      asm!(/* ASM */ "swi 0x29"
          :/* OUT */ // none
          :/* INP */ // none
          :/* CLO */ // none
          :/* OPT */ "volatile"
      );
    }
  }
}
