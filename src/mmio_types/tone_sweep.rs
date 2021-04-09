use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ToneSweep(u8);
impl ToneSweep {
  const_new!();
  bitfield_int!(u8; 0..=2: u8, sweep_shift, with_sweep_shift, set_sweep_shift);
  bitfield_bool!(u8; 3, frequency_decreasing, with_frequency_decreasing, set_frequency_decreasing);
  bitfield_int!(u8; 4..=6: u8, sweep_time, with_sweep_time, set_sweep_time);
}
