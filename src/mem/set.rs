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
