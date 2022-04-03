use super::*;

/// Use with [BitUnPack]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BitUnPackInfo {
  pub src_len: u16,
  pub src_bit_width: u8,
  pub dest_bit_width: u8,
  pub offset_and_flags: u32,
}

/// `swi #0x10`
#[inline]
pub unsafe fn BitUnPack(src: *const u8, dest: *mut u32, info: &BitUnPackInfo) {
  asm!(
    "swi #0x10",
    inout("r0") src => _,
    inout("r1") dest => _,
    in("r2") info,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x11`
#[inline]
pub unsafe fn LZ77UnCompReadNormalWrite8bit(src: *const u32, dest: *mut u8) {
  asm!(
    "swi #0x11",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x12`
#[inline]
pub unsafe fn LZ77UnCompReadNormalWrite16bit(src: *const u32, dest: *mut u16) {
  asm!(
    "swi #0x12",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x13`
#[inline]
pub unsafe fn HuffUnCompReadNormal(src: *const u32, dest: *mut u8) {
  asm!(
    "swi #0x13",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x14`
#[inline]
pub unsafe fn RLUnCompReadNormalWrite8bit(src: *const u32, dest: *mut u8) {
  asm!(
    "swi #0x14",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x15`
#[inline]
pub unsafe fn RLUnCompReadNormalWrite16bit(src: *const u32, dest: *mut u16) {
  asm!(
    "swi #0x15",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x16`
#[inline]
pub unsafe fn Diff8bitUnFilterWrite8bit(src: *const u32, dest: *mut u8) {
  asm!(
    "swi #0x16",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x17`
#[inline]
pub unsafe fn Diff8bitUnFilterWrite16bit(src: *const u32, dest: *mut u16) {
  asm!(
    "swi #0x17",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x18`
#[inline]
pub unsafe fn Diff16bitUnFilter(src: *const u32, dest: *mut u16) {
  asm!(
    "swi #0x18",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}
