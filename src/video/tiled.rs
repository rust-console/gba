//! Module for tiled mode types and operations.

use super::*;

/// An 8x8 tile with 4bpp
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Tile4bpp {
  pub data: [u32; 8],
}

/// An 8x8 tile with 8bpp
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Tile8bpp {
  pub data: [u32; 16],
}

/// A charblock of 4bpp tiles
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock4bpp {
  pub data: [Tile4bpp; 512],
}

/// A charblock of 8bpp tiles
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock8bpp {
  pub data: [Tile8bpp; 256],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct TextScreenblockEntry(u16);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct TextScreenblock {
  pub data: [TextScreenblockEntry; 32 * 32],
}
