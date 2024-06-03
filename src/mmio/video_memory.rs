use super::*;

/// The backdrop color is the color shown when no *other* element is displayed
/// in a given pixel.
pub const BACKDROP_COLOR: PlainAddr<Color> =
  unsafe { VolAddress::new(0x0500_0000) };

/// Palette data for the backgrounds
pub const BG_PALRAM: VolBlock<Color, SOGBA, SOGBA, 256> =
  unsafe { VolBlock::new(0x0500_0000) };

/// Palette data for the objects.
pub const OBJ_PALRAM: VolBlock<Color, SOGBA, SOGBA, 256> =
  unsafe { VolBlock::new(0x0500_0200) };

/// Gets the block for a specific palbank.
///
/// ## Panics
/// * If the `bank` requested is 16 or greater this will panic.
#[inline]
#[must_use]
#[cfg_attr(feature = "track_caller", track_caller)]
pub const fn obj_palbank(bank: usize) -> VolBlock<Color, SOGBA, SOGBA, 16> {
  let u = OBJ_PALRAM.index(bank * 16).as_usize();
  unsafe { VolBlock::new(u) }
}

/// The VRAM byte offset per screenblock index.
///
/// This is the same for all background types and sizes.
pub const SCREENBLOCK_INDEX_OFFSET: usize = 2 * 1_024;

/// The VRAM's background tile view, using 4bpp tiles.
pub const VRAM_BG_TILE4: VolBlock<Tile4, SOGBA, SOGBA, 2048> =
  unsafe { VolBlock::new(0x0600_0000) };

/// The VRAM's background tile view, using 8bpp tiles.
pub const VRAM_BG_TILE8: VolBlock<Tile4, SOGBA, SOGBA, 1024> =
  unsafe { VolBlock::new(0x0600_0000) };

/// The text mode screenblocks.
pub const TEXT_SCREENBLOCKS: VolGrid2dStrided<
  TextEntry,
  SOGBA,
  SOGBA,
  32,
  32,
  32,
  SCREENBLOCK_INDEX_OFFSET,
> = unsafe { VolGrid2dStrided::new(0x0600_0000) };

/// The VRAM's object tile view, using 4bpp tiles.
pub const VRAM_OBJ_TILE4: VolBlock<Tile4, SOGBA, SOGBA, 1024> =
  unsafe { VolBlock::new(0x0601_0000) };

/// The VRAM's object tile view, using 8bpp tiles.
pub const VRAM_OBJ_TILE8: VolBlock<Tile4, SOGBA, SOGBA, 512> =
  unsafe { VolBlock::new(0x0601_0000) };

/// The VRAM's view in Video Mode 3.
///
/// Each location is a direct color value.
pub const MODE3_VRAM: VolGrid2d<Color, SOGBA, SOGBA, 240, 160> =
  unsafe { VolGrid2d::new(0x0600_0000) };

/// The VRAM's view in Video Mode 4.
///
/// Each location is a pair of palette indexes into the background palette.
/// Because the VRAM can't be written with a single byte, we have to work with
/// this in units of [`u8x2`]. It's annoying, I know.
pub const MODE4_VRAM: VolGrid2dStrided<
  u8x2,
  SOGBA,
  SOGBA,
  { 240 / 2 },
  160,
  2,
  0xA000,
> = unsafe { VolGrid2dStrided::new(0x0600_0000) };

/// The VRAM's view in Video Mode 5.
///
/// Each location is a direct color value, but there's a lower image size to
/// allow for two frames.
pub const MODE5_VRAM: VolGrid2dStrided<
  Color,
  SOGBA,
  SOGBA,
  160,
  128,
  2,
  0xA000,
> = unsafe { VolGrid2dStrided::new(0x0600_0000) };

/// The combined object attributes.
pub const OBJ_ATTR_ALL: VolSeries<
  ObjAttr,
  SOGBA,
  SOGBA,
  128,
  { core::mem::size_of::<[i16; 4]>() },
> = unsafe { VolSeries::new(0x0700_0000) };

/// The object 0th attributes.
pub const OBJ_ATTR0: VolSeries<
  ObjAttr0,
  SOGBA,
  SOGBA,
  128,
  { core::mem::size_of::<[i16; 4]>() },
> = unsafe { VolSeries::new(0x0700_0000) };

/// The object 1st attributes.
pub const OBJ_ATTR1: VolSeries<
  ObjAttr1,
  SOGBA,
  SOGBA,
  128,
  { core::mem::size_of::<[i16; 4]>() },
> = unsafe { VolSeries::new(0x0700_0000 + 2) };

/// The object 2nd attributes.
pub const OBJ_ATTR2: VolSeries<
  ObjAttr2,
  SOGBA,
  SOGBA,
  128,
  { core::mem::size_of::<[i16; 4]>() },
> = unsafe { VolSeries::new(0x0700_0000 + 4) };
