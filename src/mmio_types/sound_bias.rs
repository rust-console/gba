use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SoundBias(u32);
impl SoundBias {
  const_new!();
  bitfield_int!(u32; 1..=9: u16, bias, with_bias, set_bias);
  bitfield_enum!(u32; 14..=15: SampleBits, sample_bits, with_sample_bits, set_sample_bits);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SampleBits {
  _9 = 0 << 14,
  _8 = 1 << 14,
  _7 = 2 << 14,
  _6 = 3 << 14,
}
