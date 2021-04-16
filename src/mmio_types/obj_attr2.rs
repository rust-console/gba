use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr2(u16);
impl ObjAttr2 {
  const_new!();
  bitfield_int!(u16; 0..=9: u16, tile_index, with_tile_index, set_tile_index);
  bitfield_int!(u16; 10..=11: u16, priority, with_priority, set_priority);
  bitfield_int!(u16; 12..=15: u16, palbank_index, with_palbank_index, set_palbank_index);
}
