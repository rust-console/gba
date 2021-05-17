use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SoundControl(u16);
impl SoundControl {
  const_new!();
  bitfield_int!(u16; 0..=2: u16, right_volume, with_right_volume, set_right_volume);
  bitfield_int!(u16; 4..=6: u16, left_volume, with_left_volume, set_left_volume);
  bitfield_bool!(u16; 8, tone1_right, with_tone1_right, set_tone1_right);
  bitfield_bool!(u16; 9, tone2_right, with_tone2_right, set_tone2_right);
  bitfield_bool!(u16; 10, wave_right, with_wave_right, set_wave_right);
  bitfield_bool!(u16; 11, noise_right, with_noise_right, set_noise_right);
  bitfield_bool!(u16; 12, tone1_left, with_tone1_left, set_tone1_left);
  bitfield_bool!(u16; 13, tone2_left, with_tone2_left, set_tone2_left);
  bitfield_bool!(u16; 14, wave_left, with_wave_left, set_wave_left);
  bitfield_bool!(u16; 15, noise_left, with_noise_left, set_noise_left);
}
