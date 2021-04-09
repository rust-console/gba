use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct WaveControl(u8);
impl WaveControl {
  const_new!();
  bitfield_bool!(u8; 5, two_banks, with_two_banks, set_two_banks);
  bitfield_bool!(u8; 6, use_bank1, with_use_bank1, set_use_bank1);
  bitfield_bool!(u8; 7, playing, with_playing, set_playing);
}
