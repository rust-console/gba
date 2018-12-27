use super::*;

newtype! {
  /// A screenblock entry for use in Affine mode.
  #[derive(Debug, Clone, Copy, Default)]
  AffineScreenblockEntry, u8
}

newtype! {
  /// A 16x16 screenblock for use in Affine mode.
  #[derive(Clone, Copy)]
  AffineScreenblock16x16, [AffineScreenblockEntry; 16*16], no frills
}

newtype! {
  /// A 32x32 screenblock for use in Affine mode.
  #[derive(Clone, Copy)]
  AffineScreenblock32x32, [AffineScreenblockEntry; 32*32], no frills
}

newtype! {
  /// A 64x64 screenblock for use in Affine mode.
  #[derive(Clone, Copy)]
  AffineScreenblock64x64, [AffineScreenblockEntry; 64*64], no frills
}

newtype! {
  /// A 128x128 screenblock for use in Affine mode.
  #[derive(Clone, Copy)]
  AffineScreenblock128x128, [AffineScreenblockEntry; 128*128], no frills
}
