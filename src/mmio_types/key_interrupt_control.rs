use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyInterruptControl(u16);
impl KeyInterruptControl {
  const_new!();
  bitfield_bool!(u16; 0, a, with_a, set_a);
  bitfield_bool!(u16; 1, b, with_b, set_b);
  bitfield_bool!(u16; 2, select, with_select, set_select);
  bitfield_bool!(u16; 3, start, with_start, set_start);
  bitfield_bool!(u16; 4, right, with_right, set_right);
  bitfield_bool!(u16; 5, left, with_left, set_left);
  bitfield_bool!(u16; 6, up, with_up, set_up);
  bitfield_bool!(u16; 7, down, with_down, set_down);
  bitfield_bool!(u16; 8, r, with_r, set_r);
  bitfield_bool!(u16; 9, l, with_l, set_l);
  //
  bitfield_bool!(u16; 14, enabled, with_enabled, set_enabled);
  bitfield_bool!(u16; 15, require_all, with_require_all, set_require_all);
}
