use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct WaveLenVolume(u16);
impl WaveLenVolume {
  const_new!();
  bitfield_int!(u16; 0..=7: u16, length, with_length, set_length);
  bitfield_int!(u16; 13..=14: u16, volume, with_volume, set_volume);
  bitfield_bool!(u16; 15, force75, with_force75, set_force75);
}
