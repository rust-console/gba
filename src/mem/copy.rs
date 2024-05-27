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
#[cfg_addr(feature = "on_gba", instruction_set(arm::a32))]
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
