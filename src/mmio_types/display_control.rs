use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayControl(u16);
impl DisplayControl {
  const_new!();
  bitfield_int!(u16; 0..=2: u16, display_mode, with_display_mode, set_display_mode);
  bitfield_bool!(u16; 4, display_frame1, with_display_frame1, set_display_frame1);
  bitfield_bool!(u16; 5, hblank_interval_free, with_hblank_interval_free, set_hblank_interval_free);
  bitfield_bool!(u16; 6, obj_vram_1d, with_obj_vram_1d, set_obj_vram_1d);
  bitfield_bool!(u16; 7, forced_blank, with_forced_blank, set_forced_blank);
  bitfield_bool!(u16; 8, display_bg0, with_display_bg0, set_display_bg0);
  bitfield_bool!(u16; 9, display_bg1, with_display_bg1, set_display_bg1);
  bitfield_bool!(u16; 10, display_bg2, with_display_bg2, set_display_bg2);
  bitfield_bool!(u16; 11, display_bg3, with_display_bg3, set_display_bg3);
  bitfield_bool!(u16; 12, display_obj, with_display_obj, set_display_obj);
  bitfield_bool!(u16; 13, display_win0, with_display_win0, set_display_win0);
  bitfield_bool!(u16; 14, display_win1, with_display_win1, set_display_win1);
  bitfield_bool!(u16; 15, display_obj_win, with_display_obj_win, set_display_obj_win);
}

/*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DisplayMode {
  _0 = 0,
  _1 = 1,
  _2 = 2,
  _3 = 3,
  _4 = 4,
  _5 = 5,
  _6 = 6,
  _7 = 7,
}
*/
