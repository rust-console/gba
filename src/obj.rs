//! Module for object (OBJ) entry data.
//!
//! The GBA's object drawing allows for hardware drawing that is independent of
//! the background layers. Another common term for objects is "sprites", but
//! within the GBA community they're called objects, so this crate calls them
//! objects too.
//!
//! The GBA has 128 object entries within the Object Attribute Memory (OAM)
//! region. The object entries are also interspersed with the memory for the
//! affine entries, so OAM should not be thought of as being an array of just
//! one or the other types of data.
//!
//! A few of the GBA's controls will affect all objects at once, particularly
//! the Display Control (which can control if the objects are visible at all),
//! but in general each object can be controlled independently.
//!
//! Each object entry consists of a number of bit-packed "attributes". The
//! object's attributes are stored in three 16-bit fields. The [ObjAttr] struct
//! has one field for each 16-bit group of attributes: [ObjAttr0], [ObjAttr1],
//! [ObjAttr2].
//!
//! When you've got an object's data configured how you want, use either the
//! [`OBJ_ATTR_ALL`][crate::mmio::OBJ_ATTR_ALL] control (to write all fields at
//! once) or the [`OBJ_ATTR0`][crate::mmio::OBJ_ATTR0],
//! [`OBJ_ATTR1`][crate::mmio::OBJ_ATTR1], and/or
//! [`OBJ_ATTR2`][crate::mmio::OBJ_ATTR2] controls (to write just some of the
//! fields).
//!
//! **Note:** When the GBA first boots, the object layer will be off but the
//! object entries in OAM will *not* be set to prevent individual objects from
//! being displayed. Before enabling the object layer you should generally set
//! the [ObjDisplayStyle] of all [ObjAttr0] fields so that any objects you're
//! not using don't appear on the screen. Otherwise, you'll end up with
//! un-configured objects appearing in the upper left corner of the display.

use bitfrob::{u16_with_bit, u16_with_region, u16_with_value};

/// How the object should be displayed.
///
/// Bit 9 of Attr0 changes meaning depending on Bit 8, so this merges the two
/// bits into a single property.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u16)]
pub enum ObjDisplayStyle {
  /// The default, non-affine display
  #[default]
  Normal = 0 << 8,
  /// Affine display
  Affine = 1 << 8,
  /// The object is *not* displayed at all.
  NotDisplayed = 2 << 8,
  /// Shows the object using Affine style but double sized.
  DoubleSizeAffine = 3 << 8,
}

/// What special effect the object interacts with
#[derive(Debug, Clone, Copy, Default)]
#[repr(u16)]
pub enum ObjEffectMode {
  /// The default, no special effect interaction
  #[default]
  Normal = 0 << 10,
  /// The object counts as a potential 1st target for alpha blending,
  /// regardless of the actual blend control settings register configuration.
  SemiTransparent = 1 << 10,
  /// The object is not displayed. Instead, all non-transparent pixels in this
  /// object become part of the "OBJ Window" mask.
  Window = 2 << 10,
}

/// The shape of an object.
///
/// The object's actual display area also depends on its `size` setting:
///
/// | Size | Square | Horizontal | Vertical |
/// |:-:|:-:|:-:|:-:|
/// | 0 | 8x8 | 16x8 | 8x16 |
/// | 1 | 16x16 | 32x8 | 8x32 |
/// | 2 | 32x32 | 32x16 | 16x32 |
/// | 3 | 64x64 | 64x32 | 32x64 |
#[derive(Debug, Clone, Copy, Default)]
#[repr(u16)]
#[allow(missing_docs)]
pub enum ObjShape {
  #[default]
  Square = 0 << 14,
  Horizontal = 1 << 14,
  Vertical = 2 << 14,
}

/// Object Attributes, field 0 of the entry.
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct ObjAttr0(u16);
impl ObjAttr0 {
  /// A new blank attr 0.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// Sets the `y` position of this object
  #[inline]
  pub const fn with_y(self, y: u16) -> Self {
    Self(u16_with_value(0, 7, self.0, y as u16))
  }
  /// The object's display styling.
  #[inline]
  pub const fn with_style(self, style: ObjDisplayStyle) -> Self {
    Self(u16_with_region(8, 9, self.0, style as u16))
  }
  /// The special effect mode of the object, if any.
  #[inline]
  pub const fn with_effect(self, effect: ObjEffectMode) -> Self {
    Self(u16_with_region(10, 11, self.0, effect as u16))
  }
  /// If the object should use the mosaic effect.
  #[inline]
  pub const fn with_mosaic(self, mosaic: bool) -> Self {
    Self(u16_with_bit(12, self.0, mosaic))
  }
  /// If the object draws using 8-bits-per-pixel.
  #[inline]
  pub const fn with_bpp8(self, bpp8: bool) -> Self {
    Self(u16_with_bit(13, self.0, bpp8))
  }
  /// The object's shape
  #[inline]
  pub const fn with_shape(self, shape: ObjShape) -> Self {
    Self(u16_with_region(14, 15, self.0, shape as u16))
  }
}

/// Object Attributes, field 1 of the entry.
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct ObjAttr1(u16);
impl ObjAttr1 {
  /// A new blank attr 1.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// Sets the `x` position of this object
  #[inline]
  pub const fn with_x(self, x: u16) -> Self {
    Self(u16_with_value(0, 8, self.0, x as u16))
  }
  /// The affine index of the object.
  #[inline]
  pub const fn with_affine_index(self, index: u16) -> Self {
    Self(u16_with_value(9, 13, self.0, index as u16))
  }
  /// If the object is horizontally flipped
  #[inline]
  pub const fn with_hflip(self, hflip: bool) -> Self {
    Self(u16_with_bit(12, self.0, hflip))
  }
  /// If the object is vertically flipped
  #[inline]
  pub const fn with_vflip(self, vflip: bool) -> Self {
    Self(u16_with_bit(13, self.0, vflip))
  }
  /// The object's size
  ///
  /// The size you set here, combined with the shape of the object, determines
  /// the object's actual area.
  ///
  /// | Size | Square|   Horizontal|  Vertical|
  /// |:-:|:-:|:-:|:-:|
  /// | 0 |  8x8    |  16x8   |     8x16 |
  /// | 1 |  16x16  |  32x8   |     8x32 |
  /// | 2 |  32x32  |  32x16  |     16x32 |
  /// | 3 |  64x64  |  64x32  |     32x64 |
  #[inline]
  pub const fn with_size(self, size: u16) -> Self {
    Self(u16_with_value(14, 15, self.0, size as u16))
  }
}

/// Object Attributes, field 2 of the entry.
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct ObjAttr2(u16);
impl ObjAttr2 {
  /// A new blank attr 2.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// The base tile id of the object.
  ///
  /// All other tiles in the object are automatically selected using the
  /// following tiles, according to if
  /// [`with_obj_vram_1d`][crate::video::DisplayControl::with_obj_vram_1d] it
  /// set or not.
  #[inline]
  pub const fn with_tile_id(self, id: u16) -> Self {
    Self(u16_with_value(0, 9, self.0, id as u16))
  }
  /// Sets the object's priority sorting.
  ///
  /// Lower priority objects are closer to the viewer, and will appear in front
  /// other objects that have *higher* priority, and in front of backgrounds of
  /// *equal or higher* priority. If two objects have the same priority, the
  /// lower index object is shown.
  #[inline]
  pub const fn with_priority(self, priority: u16) -> Self {
    Self(u16_with_value(10, 11, self.0, priority as u16))
  }
  /// Sets the palbank value of this object.
  #[inline]
  pub const fn with_palbank(self, palbank: u16) -> Self {
    Self(u16_with_value(12, 15, self.0, palbank as u16))
  }
}

/// Object Attributes.
///
/// The fields of this struct are all `pub` so that you can simply alter them as
/// you wish. Some "setter" methods are also provided as a shorthand.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ObjAttr(pub ObjAttr0, pub ObjAttr1, pub ObjAttr2);
#[allow(missing_docs)]
impl ObjAttr {
  #[inline]
  pub const fn new() -> Self {
    Self(ObjAttr0::new(), ObjAttr1::new(), ObjAttr2::new())
  }
  #[inline]
  pub fn set_y(&mut self, y: u16) {
    self.0 = self.0.with_y(y);
  }
  #[inline]
  pub fn set_style(&mut self, style: ObjDisplayStyle) {
    self.0 = self.0.with_style(style);
  }
  #[inline]
  pub fn set_x(&mut self, x: u16) {
    self.1 = self.1.with_x(x);
  }
  #[inline]
  pub fn set_tile_id(&mut self, id: u16) {
    self.2 = self.2.with_tile_id(id);
  }
  #[inline]
  pub fn set_palbank(&mut self, palbank: u16) {
    self.2 = self.2.with_palbank(palbank);
  }
}
