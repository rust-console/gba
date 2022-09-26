use super::*;

/// Tile information for backgrounds in one of the tile video modes (0, 1, 2).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct TextEntry(u16);
impl TextEntry {
  const_new!();
  bitfield_int!(u16; 0..=9: u16, tile_index, with_tile_index, set_tile_index);
  bitfield_bool!(u16; 10, hflip, with_hflip, set_hflip);
  bitfield_bool!(u16; 11, vflip, with_vflip, set_vflip);
  bitfield_int!(u16; 12..=15: u16, palbank_index, with_palbank_index, set_palbank_index);
}
