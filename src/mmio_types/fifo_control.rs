use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct FifoControl(u16);
impl FifoControl {
  const_new!();
  bitfield_enum!(u16; 0..=1: MixVolume, mix_volume, with_mix_volume, set_mix_volume);
  bitfield_bool!(u16; 2, full_volume_a, with_full_volume_a, set_full_volume_a);
  bitfield_bool!(u16; 3, full_volume_b, with_full_volume_b, set_full_volume_b);
  bitfield_bool!(u16; 8, enable_right_a, with_enable_right_a, set_enable_right_a);
  bitfield_bool!(u16; 9, enable_left_a, with_enable_left_a, set_enable_left_a);
  bitfield_bool!(u16; 10, use_timer1_a, with_use_timer1_a, set_use_timer1_a);
  bitfield_bool!(u16; 12, enable_right_b, with_enable_right_b, set_enable_right_b);
  bitfield_bool!(u16; 13, enable_left_b, with_enable_left_b, set_enable_left_b);
  bitfield_bool!(u16; 14, use_timer1_b, with_use_timer1_b, set_use_timer1_b);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum MixVolume {
  _25 = 0,
  _50 = 1,
  _100 = 2,
}
