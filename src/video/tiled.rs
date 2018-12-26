//! Module for tiled mode types and operations.

use super::*;

// Note(Lokathor): We've got several newtypes here that don't use the `newtype!`
// macro because it's insufficient at parsing array types being wrapped.

newtype! {
  /// An 8x8 tile with 4bpp, packed as `u32` values for proper alignment.
  #[derive(Debug, Clone, Copy, Default)]
  Tile4bpp, pub [u32; 8], no frills
}

newtype! {
  /// An 8x8 tile with 8bpp, packed as `u32` values for proper alignment.
  #[derive(Debug, Clone, Copy, Default)]
  Tile8bpp, pub [u32; 16], no frills
}

newtype! {
  /// A 4bpp charblock has 512 tiles in it
  #[derive(Clone, Copy)]
  Charblock4bpp, pub [Tile4bpp; 512], no frills
}

newtype! {
  /// An 8bpp charblock has 256 tiles in it
  #[derive(Clone, Copy)]
  Charblock8bpp, pub [Tile4bpp; 256], no frills
}

newtype! {
  /// A screenblock entry for use in Text mode.
  #[derive(Debug, Clone, Copy, Default)]
  TextScreenblockEntry, u16
}
impl TextScreenblockEntry {
  pub const fn from_tile_index(index: u16) -> Self {
    TextScreenblockEntry(index & Self::TILE_ID_MASK)
  }

  bool_bits!(u16, [(10, hflip), (11, vflip)]);

  multi_bits!(u16, [(0, 10, tile_id), (12, 4, palbank)]);
}

newtype! {
  /// A screenblock for use in Text mode.
  #[derive(Clone, Copy)]
  TextScreenblock, [TextScreenblockEntry; 32 * 32], no frills
}

newtype! {
  /// A screenblock entry for use in Affine mode.
  #[derive(Debug, Clone, Copy, Default)]
  AffineScreenblockEntry, u8
}

newtype! {
  /// A screenblock for use in Affine mode.
  #[derive(Clone, Copy)]
  AffineScreenblock16x16, [AffineScreenblockEntry; 16*16], no frills
}

newtype! {
  /// A screenblock for use in Affine mode.
  #[derive(Clone, Copy)]
  AffineScreenblock32x32, [AffineScreenblockEntry; 32*32], no frills
}

newtype! {
  /// A screenblock for use in Affine mode.
  #[derive(Clone, Copy)]
  AffineScreenblock64x64, [AffineScreenblockEntry; 64*64], no frills
}

newtype! {
  /// A screenblock for use in Affine mode.
  #[derive(Clone, Copy)]
  AffineScreenblock128x128, [AffineScreenblockEntry; 128*128], no frills
}

pub const VRAM_CHARBLOCKS: VolAddressBlock<Charblock4bpp> = unsafe { VolAddressBlock::new_unchecked(VolAddress::new_unchecked(VRAM_BASE_USIZE), 6) };

pub const VRAM_TEXT_SCREENBLOCKS: VolAddressBlock<TextScreenblock> =
  unsafe { VolAddressBlock::new_unchecked(VolAddress::new_unchecked(VRAM_BASE_USIZE), 32) };
