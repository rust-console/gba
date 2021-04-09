use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct NoiseLenEnv(u16);
impl NoiseLenEnv {
  const_new!();
  bitfield_int!(u16; 0..=5: u16, sound_length, with_sound_length, set_sound_length);
  bitfield_int!(u16; 8..=10: u16, envelope_step, with_envelope_step, set_envelope_step);
  bitfield_bool!(u16; 11, envelope_increasing, with_envelope_increasing, set_envelope_increasing);
  bitfield_int!(u16; 12..=15: u16, volume, with_volume, set_volume);
}
