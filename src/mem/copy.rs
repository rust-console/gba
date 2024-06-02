/// Copies `u8` at a time between exclusive regions.
///
/// * This will *always* copy one byte at a time, and the code is always stored
///   in IWRAM, making it suitable for use with SRAM memory.
///
/// ## Safety
/// * As with all copying routines, the source must be readable for the size you
///   specify, and the destination must be writable for the size you specify.
/// * The regions must not overlap.
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(feature = "on_gba", link_section = ".iwram.copy_u8_unchecked")]
pub unsafe extern "C" fn copy_u8_unchecked(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  on_gba_or_unimplemented!(unsafe {
    // Note(Lokathor): This loop setup assumes that the `byte_count` is usually
    // greater than 0, and so subtracts first and then does a conditional
    // load/store pair if the value (after subtracting) is greater than or equal
    // to 0 (meaning that the value before the subtract *was* 1 or more).
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

/// Copies `[u32; 8]` sized blocks, to `dest` from `src`
///
/// Particularly, this is the size of one [`Tile4`][crate::video::Tile4], half a
/// [`Tile8`][crate::video::Tile8], or one complete palbank of
/// [`Color`][crate::video::Color] values.
///
/// ## Safety
/// * As with all copying routines, the source must be readable for the size you
///   specify, and the destination must be writable for the size you specify.
/// * Both pointers must be aligned to 4.
/// * The regions must not overlap.
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(feature = "on_gba", link_section = ".iwram.copy_u32x8_unchecked")]
pub unsafe fn copy_u32x8_unchecked(
  dest: *mut [u32; 8], src: *const [u32; 8], count: usize,
) {
  on_gba_or_unimplemented!(unsafe {
    // Note(Lokathor): Same loop logic as `copy_u8_unchecked`, we're just
    // processing bigger chunks of data at a time.
    core::arch::asm!(
      "1:",
      "subs  {count}, {count}, #1",
      "ldmge {src}!, {{r3,r4,r5,r7, r8,r9,r12,lr}}",
      "stmge {dest}!, {{r3,r4,r5,r7, r8,r9,r12,lr}}",
      "bgt   1b",

      count = inout(reg) count => _,
      dest = inout(reg) dest => _,
      src = inout(reg) src => _,
      out("r3") _,
      out("r4") _,
      out("r5") _,
      out("r7") _,
      out("r8") _,
      out("r9") _,
      out("r12") _,
      // Note(Lokathor): LLVM will always put `lr` on the stack as part of the
      // push/pop for the function, even if we don't use `lr`, so we might as
      // well use `lr`, because if we use a different register (such as `r10`)
      // that would only add to the amount of push/pop LLVM does.
      out("lr") _,
      options(nostack)
    )
  });
}
