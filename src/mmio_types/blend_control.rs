use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct BlendControl(u16);
impl BlendControl {
  const_new!();
  bitfield_bool!(u16; 0, bg0_1st_target, with_bg0_1st_target, set_bg0_1st_target);
  bitfield_bool!(u16; 1, bg1_1st_target, with_bg1_1st_target, set_bg1_1st_target);
  bitfield_bool!(u16; 2, bg2_1st_target, with_bg2_1st_target, set_bg2_1st_target);
  bitfield_bool!(u16; 3, bg3_1st_target, with_bg3_1st_target, set_bg3_1st_target);
  bitfield_bool!(u16; 4, obj_1st_target, with_obj_1st_target, set_obj_1st_target);
  bitfield_bool!(u16; 5, backdrop_1st_target, with_backdrop_1st_target, set_backdrop_1st_target);
  bitfield_enum!(u16; 6..=7: ColorSpecialEffect, effect, with_effect, set_effect);
  bitfield_bool!(u16; 8, bg0_2nd_target, with_bg0_2nd_target, set_bg0_2nd_target);
  bitfield_bool!(u16; 9, bg1_2nd_target, with_bg1_2nd_target, set_bg1_2nd_target);
  bitfield_bool!(u16; 10, bg2_2nd_target, with_bg2_2nd_target, set_bg2_2nd_target);
  bitfield_bool!(u16; 11, bg3_2nd_target, with_bg3_2nd_target, set_bg3_2nd_target);
  bitfield_bool!(u16; 12, obj_2nd_target, with_obj_2nd_target, set_obj_2nd_target);
  bitfield_bool!(u16; 13, backdrop_2nd_target, with_backdrop_2nd_target, set_backdrop_2nd_target);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ColorSpecialEffect {
  NoEffect = 0 << 6,
  AlphaBlend = 1 << 6,
  BrightnessIncrease = 2 << 6,
  BrightnessDecrease = 3 << 6,
}
