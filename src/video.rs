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
  u16_bool_field!(15, enable_win_obj);
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
