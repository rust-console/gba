use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr0(u16);
impl ObjAttr0 {
  const_new!();
  bitfield_int!(u16; 0..=7: u16, y_pos, with_y_pos, set_y_pos);
  bitfield_bool!(u16; 8, affine, with_affine, set_affine);
  bitfield_bool!(u16; 9, double_disabled, with_double_disabled, set_double_disabled);
  bitfield_int!(u16; 10..=11: u16, obj_mode, with_obj_mode, set_obj_mode);
  bitfield_bool!(u16; 12, mosaic, with_mosaic, set_mosaic);
  bitfield_bool!(u16; 13, use_palbank, with_use_palbank, set_use_palbank);
  bitfield_int!(u16; 14..=15: u16, obj_shape, with_obj_shape, set_obj_shape);
}
