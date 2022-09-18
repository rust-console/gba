use crate::macros::{
  pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field, u16_int_field,
};

/// An RGB555 color value (packed into `u16`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);
impl Color {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 4, red, with_red);
  u16_int_field!(5 - 9, green, with_green);
  u16_int_field!(10 - 14, blue, with_blue);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum VideoMode {
  #[default]
  _0 = 0,
  _1 = 1,
  _2 = 2,
  _3 = 3,
  _4 = 4,
  _5 = 5,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayControl(u16);
impl DisplayControl {
  pub_const_fn_new_zeroed!();
  u16_enum_field!(0 - 2: VideoMode, video_mode, with_video_mode);
  u16_bool_field!(4, show_frame1, with_show_frame1);
  u16_bool_field!(5, hblank_oam_free, with_hblank_oam_free);
  u16_bool_field!(6, obj_vram_1d, with_obj_vram_1d);
  u16_bool_field!(7, forced_blank, with_forced_blank);
  u16_bool_field!(8, show_bg0, with_show_bg0);
  u16_bool_field!(9, show_bg1, with_show_bg1);
  u16_bool_field!(10, show_bg2, with_show_bg2);
  u16_bool_field!(11, show_bg3, with_show_bg3);
  u16_bool_field!(12, show_obj, with_show_obj);
  u16_bool_field!(13, enable_win0, with_enable_win0);
  u16_bool_field!(14, enable_win1, with_enable_win1);
  u16_bool_field!(15, enable_obj_win, with_enable_obj_win);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayStatus(u16);
impl DisplayStatus {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, currently_vblank, with_currently_vblank);
  u16_bool_field!(1, currently_hblank, with_currently_hblank);
  u16_bool_field!(2, currently_vcount, with_currently_vcount);
  u16_bool_field!(3, irq_vblank, with_irq_vblank);
  u16_bool_field!(4, irq_hblank, with_irq_hblank);
  u16_bool_field!(5, irq_vcount, with_irq_vcount);
  u16_int_field!(8 - 15, vcount_setting, with_vcount_setting);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BackgroundControl(u16);
impl BackgroundControl {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 1, priority, with_priority);
  u16_int_field!(2 - 3, charblock, with_charblock);
  u16_bool_field!(6, mosaic, with_mosaic);
  u16_bool_field!(7, bpp8, with_bpp8);
  u16_int_field!(8 - 12, screenblock, with_screenblock);
  u16_bool_field!(13, is_affine_wrapping, with_is_affine_wrapping);
  u16_int_field!(14 - 15, size, with_size);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowInside(u16);
impl WindowInside {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, win0_bg0, with_win0_bg0);
  u16_bool_field!(1, win0_bg1, with_win0_bg1);
  u16_bool_field!(2, win0_bg2, with_win0_bg2);
  u16_bool_field!(3, win0_bg3, with_win0_bg3);
  u16_bool_field!(4, win0_obj, with_win0_obj);
  u16_bool_field!(5, win0_effect, with_win0_effect);
  u16_bool_field!(8, win1_bg0, with_win1_bg0);
  u16_bool_field!(9, win1_bg1, with_win1_bg1);
  u16_bool_field!(10, win1_bg2, with_win1_bg2);
  u16_bool_field!(11, win1_bg3, with_win1_bg3);
  u16_bool_field!(12, win1_obj, with_win1_obj);
  u16_bool_field!(13, win1_effect, with_win1_effect);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowOutside(u16);
impl WindowOutside {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, outside_bg0, with_outside_bg0);
  u16_bool_field!(1, outside_bg1, with_outside_bg1);
  u16_bool_field!(2, outside_bg2, with_outside_bg2);
  u16_bool_field!(3, outside_bg3, with_outside_bg3);
  u16_bool_field!(4, outside_obj, with_outside_obj);
  u16_bool_field!(5, outside_effect, with_outside_effect);
  u16_bool_field!(8, obj_win_bg0, with_obj_win_bg0);
  u16_bool_field!(9, obj_win_bg1, with_obj_win_bg1);
  u16_bool_field!(10, obj_win_bg2, with_obj_win_bg2);
  u16_bool_field!(11, obj_win_bg3, with_obj_win_bg3);
  u16_bool_field!(12, obj_win_obj, with_obj_win_obj);
  u16_bool_field!(13, obj_win_effect, with_obj_win_effect);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Mosaic(u16);
impl Mosaic {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 3, bg_h_extra, with_bg_h_extra);
  u16_int_field!(4 - 7, bg_v_extra, with_bg_v_extra);
  u16_int_field!(8 - 11, obj_h_extra, with_obj_h_extra);
  u16_int_field!(12 - 15, obj_v_extra, with_obj_v_extra);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ColorEffectMode {
  #[default]
  NoEffect = 0 << 6,
  AlphaBlend = 1 << 6,
  Brighten = 2 << 6,
  Darken = 3 << 6,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BlendControl(u16);
impl BlendControl {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, target1_bg0, with_target1_bg0);
  u16_bool_field!(1, target1_bg1, with_target1_bg1);
  u16_bool_field!(2, target1_bg2, with_target1_bg2);
  u16_bool_field!(3, target1_bg3, with_target1_bg3);
  u16_bool_field!(4, target1_obj, with_target1_obj);
  u16_bool_field!(5, target1_backdrop, with_target1_backdrop);
  u16_enum_field!(6 - 7: ColorEffectMode, mode, with_mode);
  u16_bool_field!(8, target2_bg0, with_target2_bg0);
  u16_bool_field!(9, target2_bg1, with_target2_bg1);
  u16_bool_field!(10, target2_bg2, with_target2_bg2);
  u16_bool_field!(11, target2_bg3, with_target2_bg3);
  u16_bool_field!(12, target2_obj, with_target2_obj);
  u16_bool_field!(13, target2_backdrop, with_target2_backdrop);
}

/// Data for a 4-bit-per-pixel tile.
pub type Tile4 = [u32; 8];

/// Data for an 8-bit-per-pixel tile.
pub type Tile8 = [u32; 16];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ObjDisplayStyle {
  #[default]
  Normal = 0 << 8,
  Affine = 1 << 8,
  NotDisplayed = 2 << 8,
  DoubleSizeAffine = 3 << 8,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ObjDisplayMode {
  #[default]
  Normal = 0 << 10,
  SemiTransparent = 1 << 10,
  Window = 2 << 10,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ObjShape {
  #[default]
  Square = 0 << 14,
  Horizontal = 1 << 14,
  Vertical = 2 << 14,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjAttr0(u16);
impl ObjAttr0 {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 7, y, with_y);
  u16_enum_field!(8 - 9: ObjDisplayStyle, style, with_style);
  u16_enum_field!(10 - 11: ObjDisplayMode, mode, with_mode);
  u16_bool_field!(12, mosaic, with_mosaic);
  u16_bool_field!(13, bpp8, with_bpp8);
  u16_enum_field!(14 - 15: ObjShape, shape, with_shape);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjAttr1(u16);
impl ObjAttr1 {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 8, x, with_x);
  u16_int_field!(9 - 13, affine_index, with_affine_index);
  u16_bool_field!(12, hflip, with_hflip);
  u16_bool_field!(13, vflip, with_vflip);
  u16_int_field!(14 - 15, size, with_size);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjAttr2(u16);
impl ObjAttr2 {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 9, tile_id, with_tile_id);
  u16_int_field!(10 - 11, priority, with_priority);
  u16_int_field!(12 - 15, palbank, with_palbank);
}
