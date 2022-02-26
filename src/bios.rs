#![allow(non_snake_case)]

use core::arch::asm;

/// `swi #0x05`
///
/// Works as per `IntrWait`, but always discards old flags, and then waits for
/// a VBlank interrupt.
#[inline]
pub fn VBlankIntrWait() {
  unsafe {
    asm!(
      "swi #0x05",
      out("r0") _,
      out("r1") _,
      out("r3") _,
      options(preserves_flags),
    )
  };
}

#[repr(C)]
pub struct UnPackInfo {
  pub src_len: u16,
  pub src_bit_width: u8,
  pub dest_bit_width: u8,
  pub offset_and_flags: u32,
}

/// `swi #0x10`
#[inline]
pub unsafe fn BitUnPack(src: *const u8, dest: *mut u32, info: &UnPackInfo) {
  asm!(
    "swi #0x10",
    inout("r0") src => _,
    inout("r1") dest => _,
    in("r2") info,
    out("r3") _,
    options(preserves_flags),
  );
}

/// `swi #0x12`
#[inline]
pub unsafe fn LZ77UnCompReadNormalWrite16bit(src: *const u32, dest: *mut u32) {
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
pub unsafe fn HuffUnCompReadNormal(src: *const u32, dest: *mut u32) {
  asm!(
    "swi #0x13",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  );
}
