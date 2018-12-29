//! Module for tiled mode types and operations.

use super::*;

newtype! {
  /// A screenblock entry for use in Text mode.
  #[derive(Debug, Clone, Copy, Default)]
  TextScreenblockEntry, u16
}
impl TextScreenblockEntry {
  /// Generates a default entry with the specified tile index.
  pub const fn from_tile_index(index: u16) -> Self {
    Self::new().with_tile_index(index)
  }

  bool_bits!(u16, [(10, hflip), (11, vflip)]);

  multi_bits!(u16, [(0, 10, tile_index), (12, 4, palbank)]);
}

newtype! {
  /// A screenblock for use in Text mode.
  #[derive(Clone, Copy)]
  TextScreenblock, [TextScreenblockEntry; 32 * 32], no frills
}

#[test]
pub fn test_text_screen_block_size() {
  assert_eq!(core::mem::size_of::<TextScreenblock>(), 0x800);
}
