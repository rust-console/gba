use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SoundStatus(u8);
impl SoundStatus {
  const_new!();
  bitfield_bool!(u8; 0, tone1_playing, with_tone1_playing, set_tone1_playing);
  bitfield_bool!(u8; 1, tone2_playing, with_tone2_playing, set_tone2_playing);
  bitfield_bool!(u8; 2, wave_playing, with_wave_playing, set_wave_playing);
  bitfield_bool!(u8; 3, noise_playing, with_noise_playing, set_noise_playing);
  bitfield_bool!(u8; 7, enabled, with_enabled, set_enabled);
}
