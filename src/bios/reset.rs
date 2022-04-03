use super::*;

/// `swi 0x00`: Performs a "soft reset" of the device.
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
  unsafe { asm!("swi #0x00", options(noreturn)) }
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
/// * Using this to reset EWRAM or IWRAM while the program is still running is
///   **very** ill advised and you probably just shouldn't do it.
#[inline]
pub unsafe fn RegisterRamReset(flags: RegisterRamResetFlags) {
  asm!("swi #0x01",
    inout("r0") flags.0 => _,
    out("r1") _,
    out("r3") _,
    options(preserves_flags)
  )
}
