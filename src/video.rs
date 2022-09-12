use crate::macros::{
  pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field, u16_int_field,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum VideoMode {
  _0 = 0,
  _1 = 1,
  _2 = 2,
  _3 = 3,
  _4 = 4,
  _5 = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayControl(u16);
impl DisplayControl {
  pub_const_fn_new_zeroed!();
  u16_enum_field!(0 - 2: VideoMode, video_mode);
  u16_bool_field!(4, show_frame1);
  u16_bool_field!(5, hblank_oam_free);
  u16_bool_field!(6, obj_vram_1d);
  u16_bool_field!(7, forced_blank);
  u16_bool_field!(8, show_bg0);
  u16_bool_field!(9, show_bg1);
  u16_bool_field!(10, show_bg2);
  u16_bool_field!(11, show_bg3);
  u16_bool_field!(12, show_obj);
  u16_bool_field!(13, enable_win0);
  u16_bool_field!(14, enable_win1);
  u16_bool_field!(15, enable_obj_win);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayStatus(u16);
impl DisplayStatus {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, is_vblank);
  u16_bool_field!(1, is_hblank);
  u16_bool_field!(2, is_vcount);
  u16_bool_field!(3, irq_vblank);
  u16_bool_field!(4, irq_hblank);
  u16_bool_field!(5, irq_vcount);
  u16_int_field!(8 - 15, vcount_setting);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BackgroundControl(u16);
impl BackgroundControl {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 1, priority);
  u16_int_field!(2 - 3, charblock);
  u16_bool_field!(6, is_mosaic);
  u16_bool_field!(7, is_8bpp);
  u16_int_field!(8 - 12, screenblock);
  u16_bool_field!(13, is_affine_wrapping);
  u16_int_field!(14 - 15, size);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowInside(u16);
impl WindowInside {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, win0_bg0);
  u16_bool_field!(1, win0_bg1);
  u16_bool_field!(2, win0_bg2);
  u16_bool_field!(3, win0_bg3);
  u16_bool_field!(4, win0_obj);
  u16_bool_field!(5, win0_effect);
  u16_bool_field!(8, win1_bg0);
  u16_bool_field!(9, win1_bg1);
  u16_bool_field!(10, win1_bg2);
  u16_bool_field!(11, win1_bg3);
  u16_bool_field!(12, win1_obj);
  u16_bool_field!(13, win1_effect);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowOutside(u16);
impl WindowOutside {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, outside_bg0);
  u16_bool_field!(1, outside_bg1);
  u16_bool_field!(2, outside_bg2);
  u16_bool_field!(3, outside_bg3);
  u16_bool_field!(4, outside_obj);
  u16_bool_field!(5, outside_effect);
  u16_bool_field!(8, obj_win_bg0);
  u16_bool_field!(9, obj_win_bg1);
  u16_bool_field!(10, obj_win_bg2);
  u16_bool_field!(11, obj_win_bg3);
  u16_bool_field!(12, obj_win_obj);
  u16_bool_field!(13, obj_win_effect);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Mosaic(u16);
impl Mosaic {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 3, bg_h_extra);
  u16_int_field!(4 - 7, bg_v_extra);
  u16_int_field!(8 - 11, obj_h_extra);
  u16_int_field!(12 - 15, obj_v_extra);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ColorEffectMode {
  NoEffect = 0 << 6,
  AlphaBlend = 1 << 6,
  Brighten = 2 << 6,
  Darken = 3 << 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BlendControl(u16);
impl BlendControl {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, target1_bg0);
  u16_bool_field!(1, target1_bg1);
  u16_bool_field!(2, target1_bg2);
  u16_bool_field!(3, target1_bg3);
  u16_bool_field!(4, target1_obj);
  u16_bool_field!(5, target1_backdrop);
  u16_enum_field!(6 - 7: ColorEffectMode, mode);
  u16_bool_field!(8, target2_bg0);
  u16_bool_field!(9, target2_bg1);
  u16_bool_field!(10, target2_bg2);
  u16_bool_field!(11, target2_bg3);
  u16_bool_field!(12, target2_obj);
  u16_bool_field!(13, target2_backdrop);
}
