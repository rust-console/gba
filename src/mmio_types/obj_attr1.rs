use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr1(u16);
impl ObjAttr1 {
  const_new!();
  bitfield_int!(u16; 0..=8: u16, x_pos, with_x_pos, set_x_pos);
  bitfield_int!(u16; 9..=13: u16, affine_index, with_affine_index, set_affine_index);
  bitfield_bool!(u16; 12, hflip, with_hflip, set_hflip);
  bitfield_bool!(u16; 13, vflip, with_vflip, set_vflip);
  bitfield_int!(u16; 14..=15: u16, obj_size, with_obj_size, set_obj_size);
}
