use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct WaveFrequencyControl(u16);
impl WaveFrequencyControl {
  const_new!();
  bitfield_int!(u16; 0..=10: u16, frequency, with_frequency, set_frequency);
  bitfield_bool!(u16; 14, auto_stop, with_auto_stop, set_auto_stop);
  bitfield_bool!(u16; 15, restart, with_restart, set_restart);
}
