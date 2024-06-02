/// `u8` copy between exclusive regions.
///
/// * This will *always* copy one byte at a time, and the code is always stored
///   in IWRAM, making it suitable for use with SRAM memory.
///
/// ## Safety
/// * If `byte_count` is zero then the pointers are not used at all, and they
///   can be any value.
/// * If `byte_count` is non-zero then:
///   * Both pointers must be valid for the number of bytes given.
///   * The two regions must either be *entirely* disjoint or *entirely*
///     overlapping. Partial overlap is not allowed.
#[inline]
#[link_section = ".iwram.__aeabi_memcpy1"]
#[cfg_attr(feature = "no_mangle_memcpy", no_mangle)]
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
pub unsafe extern "C" fn __aeabi_memcpy1(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm! {
      "1:",
      "subs    {count}, {count}, #1",
      "ldrbge  {temp}, [{src}], #1",
      "strbge  {temp}, [{dest}], #1",
      "bgt     1b",
      temp = out(reg) _,
      count = inout(reg) byte_count => _,
      src = inout(reg) src => _,
      dest = inout(reg) dest => _,
      options(nostack)
    }
  });
}

/// Copies eight `u32` at a time to `dest` from `src`
///
/// Particularly, this is the size of one [`Tile4`][crate::video::Tile4], half a
/// [`Tile8`][crate::video::Tile8], or one complete palbank of
/// [`Color`][crate::video::Color] values.
///
/// ## Safety
/// * As with all copying routines, the source must be readable for the size you
///   specify, and the destination must be writable for the size you specify.
/// * Both pointers must be aligned to 4.
#[link_section = ".iwram.copy_u32x8_unchecked"]
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
pub unsafe fn copy_u32x8_unchecked(
  mut dest: *mut u32, mut src: *const u32, mut count: usize,
) {
  while count > 0 {
    on_gba_or_unimplemented!(unsafe {
      core::arch::asm!(
        "ldm {src}!, {{r3,r4,r5,r7, r8,r9,r10,r12}}",
        "stm {dest}!, {{r3,r4,r5,r7, r8,r9,r10,r12}}",
        dest = inout(reg) dest,
        src = inout(reg) src,
        out("r3") _,
        out("r4") _,
        out("r5") _,
        out("r7") _,
        out("r8") _,
        out("r9") _,
        out("r10") _,
        out("r12") _,
        options(nostack)
      )
    });
    count -= 1;
  }
}
