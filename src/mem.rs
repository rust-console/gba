use crate::macros::on_gba_or_unimplemented;

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

/// Copies `[u32; 8]` sized chunks, to `dest` from `src`
///
/// Particularly, this helps with:
/// * [`Tile4`][crate::video::Tile4] (one loop per tile).
/// * [`Tile8`][crate::video::Tile8] (two loops per tile).
/// * A palbank of [`Color`][crate::video::Color] values (one loop per palbank).
/// * A text mode screenblock (64 loops per screenblock).
///
/// This will, in general, be slightly faster than a generic `memcpy`, but
/// slightly slower than using DMA.
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

/// Sets `word` in blocks of 80 per loop.
///
/// This is intended for clearing VRAM to a particular color when using
/// background modes 3, 4, and 5.
/// * To clear the Mode 3 bitmap, pass `240` as the count.
/// * To clear a Mode 4 frame pass `120`.
/// * To clear a Mode 5 frame pass `128`.
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(feature = "on_gba", link_section = ".iwram.set_u32x80_unchecked")]
pub unsafe extern "C" fn set_u32x80_unchecked(
  dest: *mut u32, word: u32, count: usize,
) {
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm!(
      // Note(Lokathor): Same loop logic as `copy_u8_unchecked`, we're just
      // processing bigger chunks of data at a time, and also setting rather
      // than copying.
      "1:",
      "subs {count}, {count}, #1",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "bgt   1b",

      // The assembler will give us a warning (that we can't easily disable)
      // if the reg_list for `stm` doesn't give the registers in order from
      // low to high, so we just manually pick registers. The count register
      // and the pointer register can be anything else.
      in("r1") word,
      in("r3") word,
      in("r4") word,
      in("r5") word,
      in("r7") word,
      in("r8") word,
      in("r12") word,
      in("lr") word,
      dest = inout(reg) dest => _,
      count = inout(reg) count => _,
      options(nostack),
    )
  });
}
