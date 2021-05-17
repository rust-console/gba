use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SoundBias(u16);
impl SoundBias {
  const_new!();
  bitfield_int!(u16; 1..=9: u16, bias, with_bias, set_bias);
  bitfield_enum!(u16; 14..=15: SampleBits, sample_bits, with_sample_bits, set_sample_bits);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SampleBits {
  _9 = 0 << 14,
  _8 = 1 << 14,
  _7 = 2 << 14,
  _6 = 3 << 14,
}
