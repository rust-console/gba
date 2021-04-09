use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct MosaicSize(u8);
impl MosaicSize {
  const_new!();
  bitfield_int!(u8; 0..=3: u8, horizontal, with_horizontal, set_horizontal);
  bitfield_int!(u8; 4..=7: u8, vertical, with_vertical, set_vertical);
}
