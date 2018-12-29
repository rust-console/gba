//! Types and declarations for the Object Attribute Memory (`OAM`).

use super::*;

newtype! {
  /// 0th part of an object's attributes.
  ///
  /// * Bits 0-7: row-coordinate
  /// * Bits 8-9: Rendering style: Normal, Affine, Disabled, Double Area Affine
  /// * Bits 10-11: Object mode: Normal, SemiTransparent, Object Window
  /// * Bit 12: Mosaic
  /// * Bit 13: is 8bpp
  /// * Bits 14-15: Object Shape: Square, Horizontal, Vertical
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  OBJAttr0, u16
}
impl OBJAttr0 {
  phantom_fields! {
    self.0: u16,
    row_coordinate: 0-7,
    obj_rendering: 8-9=ObjectRender<Normal, Affine, Disabled, DoubleAreaAffine>,
    obj_mode: 10-11=ObjectMode<Normal, SemiTransparent, OBJWindow>,
    mosaic: 12,
    is_8bpp: 13,
    obj_shape: 14-15=ObjectShape<Square, Horizontal, Vertical>,
  }
}

/// What style of rendering for this object
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectRender {
  /// Standard, non-affine rendering
  Normal = 0,
  /// Affine rendering
  Affine = 1,
  /// Object disabled (saves cycles for elsewhere!)
  Disabled = 2,
  /// Affine with double render space (helps prevent clipping)
  DoubleAreaAffine = 3,
}

/// What mode to ues for the object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectMode {
  /// Show the object normally
  Normal = 0,
  /// The object becomes the "Alpha Blending 1st target" (see Alpha Blending)
  SemiTransparent = 1,
  /// Use the object's non-transparent pixels as part of a mask for the object
  /// window (see Windows).
  OBJWindow = 2,
}

/// What shape the object's appearance should be.
///
/// The specifics also depend on the `ObjectSize` set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectShape {
  /// Equal parts wide and tall
  Square = 0,
  /// Wider than tall
  Horizontal = 1,
  /// Taller than wide
  Vertical = 2,
}

newtype! {
  /// 1st part of an object's attributes.
  ///
  /// * Bits 0-8: column coordinate
  /// * Bits 9-13:
  ///   * Normal render: Bit 12 holds hflip and 13 holds vflip.
  ///   * Affine render: The affine parameter selection.
  /// * Bits 14-15: Object Size
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  OBJAttr1, u16
}
impl OBJAttr1 {
  phantom_fields! {
    self.0: u16,
    col_coordinate: 0-8,
    affine_index: 9-13,
    hflip: 12,
    vflip: 13,
    obj_size: 14-15=ObjectSize<Zero, One, Two, Three>,
  }
}

/// The object's size.
///
/// Also depends on the `ObjectShape` set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectSize {
  /// * Square: 8x8px
  /// * Horizontal: 16x8px
  /// * Vertical: 8x16px
  Zero = 0,
  /// * Square: 16x16px
  /// * Horizontal: 32x8px
  /// * Vertical: 8x32px
  One = 1,
  /// * Square: 32x32px
  /// * Horizontal: 32x16px
  /// * Vertical: 16x32px
  Two = 2,
  /// * Square: 64x64px
  /// * Horizontal: 64x32px
  /// * Vertical: 32x64px
  Three = 3,
}

newtype! {
  /// 2nd part of an object's attributes.
  ///
  /// * Bits 0-9: Base Tile Index (tile offset from CBB4)
  /// * Bits 10-11: Priority
  /// * Bits 12-15: Palbank (if using 4bpp)
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  OBJAttr2, u16
}
impl OBJAttr2 {
  phantom_fields! {
    self.0: u16,
    tile_id: 0-9,
    priority: 10-11,
    palbank: 12-15,
  }
}
