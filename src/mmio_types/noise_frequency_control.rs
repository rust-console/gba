use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct NoiseFrequencyControl(u16);
impl NoiseFrequencyControl {
  const_new!();
  bitfield_int!(u16; 0..=2: u16, div_ratio, with_div_ratio, set_div_ratio);
  bitfield_bool!(u16; 3, counter_width, with_counter_width, set_counter_width);
  bitfield_int!(u16; 4..=7: u16, shift_frequency, with_shift_frequency, set_shift_frequency);
  bitfield_bool!(u16; 14, auto_stop, with_auto_stop, set_auto_stop);
  bitfield_bool!(u16; 15, restart, with_restart, set_restart);
}
