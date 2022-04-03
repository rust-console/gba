use super::*;

/// Used with [BgAffineSet]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BgAffineSetSrc {
  pub original_center_x: i32,
  pub original_center_y: i32,
  pub display_center_x: i16,
  pub display_center_y: i16,
  pub scale_x: i16,
  pub scale_y: i16,
  pub angle: u16,
}

/// Used with [BgAffineSet]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BgAffineSetDest {
  pub dx_same: i16,
  pub dx_next: i16,
  pub dy_same: i16,
  pub dy_next: i16,
  pub start_x: i32,
  pub start_y: i32,
}

/// `swi #0x0E`
#[inline]
pub unsafe fn BgAffineSet(
  src: *const BgAffineSetSrc, dst: *mut BgAffineSetDest, count: usize,
) {
  asm! {
    "swi #0x0E",
    inout("r0") src => _,
    inout("r1") dst => _,
    in("r2") count,
    out("r3") _,
    options(preserves_flags)
  }
}

/// Used with [ObjAffineSet]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ObjAffineSetSrc {
  pub scale_x: i16,
  pub scale_y: i16,
  pub angle: u16,
}

/// Used with [ObjAffineSet]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ObjAffineSetDest {
  pub dx_same: i16,
  pub dx_next: i16,
  pub dy_same: i16,
  pub dy_next: i16,
}

/// `swi #0x0F`
#[inline]
pub unsafe fn ObjAffineSet(
  src: *const ObjAffineSetSrc, dst: *mut ObjAffineSetDest, count: usize,
  affine_param_offset: usize,
) {
  asm! {
    "swi #0x0F",
    inout("r0") src => _,
    inout("r1") dst => _,
    in("r2") count,
    inout("r3") affine_param_offset => _,
    options(preserves_flags)
  }
}
