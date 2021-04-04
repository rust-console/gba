use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct WindowEnable(u8);
impl WindowEnable {
  const_new!();
  bitfield_bool!(u8; 0, bg0, with_bg0, set_bg0);
  bitfield_bool!(u8; 1, bg1, with_bg1, set_bg1);
  bitfield_bool!(u8; 2, bg2, with_bg2, set_bg2);
  bitfield_bool!(u8; 3, bg3, with_bg3, set_bg3);
  bitfield_bool!(u8; 4, obj, with_obj, set_obj);
  bitfield_bool!(u8; 5, effect, with_effect, set_effect);
}
