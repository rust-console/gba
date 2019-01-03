//! Module for tiled mode types and operations.

use super::*;

newtype! {
  /// A screenblock entry for use in Text mode.
  TextScreenblockEntry, u16
}
impl TextScreenblockEntry {
  /// Generates a default entry with the specified tile index.
  pub const fn from_tile_id(id: u16) -> Self {
    Self::new().with_tile_id(id)
  }

  phantom_fields! {
    self.0: u16,
    tile_id: 0-9,
    hflip: 10,
    vflip: 11,
    palbank: 12-15,
  }
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
