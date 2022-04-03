use crate::macros::{const_new, u16_bool_field, u16_enum_field};
use voladdress::{Safe, VolAddress};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum VideoMode {
  VideoMode0 = 0,
  VideoMode1 = 1,
  VideoMode2 = 2,
  VideoMode3 = 3,
  VideoMode4 = 4,
  VideoMode5 = 5,
}
impl Default for VideoMode {
  #[inline]
  #[must_use]
  fn default() -> Self {
    VideoMode0
  }
}
pub use VideoMode::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayControl(u16);
impl DisplayControl {
  const_new!();
  u16_enum_field!(0 - 2: VideoMode, video_mode, with_video_mode);
  u16_bool_field!(4, display_frame1, with_display_frame1);
  u16_bool_field!(5, hblank_oam_free, with_hblank_oam_free);
  u16_bool_field!(6, obj_vram_1d, with_obj_vram_1d);
  u16_bool_field!(7, forced_blank, with_forced_blank);
  u16_bool_field!(8, display_bg0, with_display_bg0);
  u16_bool_field!(9, display_bg1, with_display_bg1);
  u16_bool_field!(10, display_bg2, with_display_bg2);
  u16_bool_field!(11, display_bg3, with_display_bg3);
  u16_bool_field!(12, display_obj, with_display_obj);
  u16_bool_field!(13, display_win0, with_display_win0);
  u16_bool_field!(14, display_win1, with_display_win1);
  u16_bool_field!(15, display_obj_win, with_display_obj_win);
}
pub const DISPCNT: VolAddress<DisplayControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };
