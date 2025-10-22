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
//! When you've got an object's data configured how you want, use the `write`
//! function on the [ObjAttr] struct, the [ObjAttrWriteExt] extension trait for
//! the [`OBJ_ATTR_ALL`] control (to write all fields at once), or the
//! [`OBJ_ATTR0`], [`OBJ_ATTR1`], and/or [`OBJ_ATTR2`] controls (to write just
//! some of the fields).
//!
//! **Note:** When the GBA first boots, the object layer will be off but the
//! object entries in OAM will *not* be set to prevent individual objects from
//! being displayed. Before enabling the object layer you should generally set
//! the [ObjDisplayStyle] of all [ObjAttr0] fields so that any objects you're
//! not using don't appear on the screen. Otherwise, you'll end up with
//! un-configured objects appearing in the upper left corner of the display.
//!
//! [`OBJ_ATTR_ALL`] is defined as read-only because writing to OAM requires
//! halfword alignment, and at higher compiler optimization levels the compiler
//! will generate a `memcpy` call which may do byte-wise writes. To avoid this,
//! use the `write` function on the [ObjAttr] struct, the `write` function on
//! the [ObjAttrWriteExt] trait implemented for the [`OBJ_ATTR_ALL`] control,
//! or write to the individual [`OBJ_ATTR0`], [`OBJ_ATTR1`], and [`OBJ_ATTR2`]
//! controls.

use super::*;
use voladdress::{Safe, VolAddress};

/// How the object should be displayed.
///
/// Bit 9 of Attr0 changes meaning depending on Bit 8, so this merges the two
/// bits into a single property.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(missing_docs)]
pub enum ObjShape {
  #[default]
  Square = 0 << 14,
  Horizontal = 1 << 14,
  Vertical = 2 << 14,
}

/// Object Attributes, field 0 of the entry.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjAttr0(u16);
impl ObjAttr0 {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 7, y, with_y);
  u16_enum_field!(8 - 9: ObjDisplayStyle, style, with_style);
  u16_enum_field!(10 - 11: ObjEffectMode, mode, with_mode);
  u16_bool_field!(12, mosaic, with_mosaic);
  u16_bool_field!(13, bpp8, with_bpp8);
  u16_enum_field!(14 - 15: ObjShape, shape, with_shape);
}

/// Object Attributes, field 1 of the entry.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjAttr1(u16);
impl ObjAttr1 {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 8, x, with_x);
  u16_int_field!(9 - 13, affine_index, with_affine_index);
  u16_bool_field!(12, hflip, with_hflip);
  u16_bool_field!(13, vflip, with_vflip);
  u16_int_field!(14 - 15, size, with_size);
}

/// Object Attributes, field 2 of the entry.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjAttr2(u16);
impl ObjAttr2 {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 9, tile_id, with_tile_id);
  u16_int_field!(10 - 11, priority, with_priority);
  u16_int_field!(12 - 15, palbank, with_palbank);
}

/// Object Attributes.
///
/// The fields of this struct are all `pub` so that you can simply alter them as
/// you wish. Some "setter" methods are also provided as a shorthand.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
  #[inline]
  pub fn write(&self, addr: VolAddress<ObjAttr, Safe, ()>) {
    unsafe {
      let addr: *mut u16 = addr.as_usize() as *mut u16;
      core::ptr::write_volatile(addr.add(0), self.0 .0);
      core::ptr::write_volatile(addr.add(1), self.1 .0);
      core::ptr::write_volatile(addr.add(2), self.2 .0);
    }
  }
}

pub trait ObjAttrWriteExt {
  fn write(&self, attr: ObjAttr);
}

impl ObjAttrWriteExt for VolAddress<ObjAttr, Safe, ()> {
  #[inline]
  fn write(&self, attr: ObjAttr) {
    attr.write(*self);
  }
}
