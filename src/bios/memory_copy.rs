use super::*;

/// `swi #0x0B`
#[inline]
pub unsafe fn CpuSet(src: *const u32, dest: *mut u32, len_mode: u32) {
  debug_assert_eq!(len_mode % 8, 0);
  asm!(
    "swi #0x0B",
    inout("r0") src => _,
    inout("r1") dest => _,
    in("r2") len_mode,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x0C`
#[inline]
pub unsafe fn CpuFastSet(src: *const u32, dest: *mut u32, len_mode: u32) {
  asm!(
    "swi #0x0C",
    inout("r0") src => _,
    inout("r1") dest => _,
    in("r2") len_mode,
    out("r3") _,
    options(preserves_flags),
  );
}
