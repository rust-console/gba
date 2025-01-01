//! Module to control the GBA's screen.
//!
//! # Video Basics
//!
//! To configure the screen's display, you should first decide on the
//! [`DisplayControl`] value that you want to set to the
//! [`DISPCNT`](crate::mmio::DISPCNT) register. This configures several things,
//! but most importantly it determines the [`VideoMode`] for the display to use.
//!
//! The GBA has four Background layers. Depending on the current video mode,
//! different background layers will be available for use in either "text",
//! "affine", or "bitmap" mode.
//!
//! In addition to the background layers, there's also an "OBJ" layer. This
//! allows the display of a number of "objects", which can move independently of
//! any background. Generally, one or more objects will be used to display the
//! "sprites" within a game. Because there isn't an exact 1:1 mapping between
//! sprites and objects, these docs will attempt to only talk about objects.
//!
//! ## Color, Bit Depth, and Palettes
//!
//! [Color] values on the GBA are 5-bits-per-channel RGB values. They're always
//! bit-packed and aligned to 2, so think of them as being like a `u16`.
//!
//! Because of the GBA's limited memory, most images don't use direct color (one
//! color per pixel). Instead they use indexed color (one *palette index* per
//! pixel). Indexed image data can be 4-bits-per-pixel (4bpp) or
//! 8-bits-per-pixel (8bpp). In either case, the color values themselves are
//! stored in the PALRAM region. The PALRAM contains the [`BG_PALETTE`] and
//! [`OBJ_PALETTE`], which hold the color values for backgrounds and objects
//! respectively. Both palettes have 256 slots. The palettes are always indexed
//! with 8 bits total, but *how* those bits are determined depends on the bit
//! depth of the image:
//! * Things drawing with 8bpp image data index into the full range of the
//!   palette directly.
//! * Things drawing with 4bpp image data will also have a "palbank" setting.
//!   The palbank acts as the upper 4 bits of the index, selecting which block
//!   of 16 palette entries the that thing will be able to use. Then each 4-bit
//!   pixel within the image indexes within the palbank.
//!
//! In both 8bpp and 4bpp modes, if a particular pixel's index value is 0 then
//! that pixel is instead considered transparent. So 8bpp images can use 255
//! colors (+ transparent), and 4bpp images can use 15 colors (+ transparent).
//! Each background layer and each object can individually be set to display
//! with either 4bpp or 8bpp mode.
//!
//! ## Tiles, Screenblocks, and Charblocks
//!
//! The basic unit of the GBA's hardware graphics support is a "tile".
//! Regardless of their bit depth, a tile is always an 8x8 area. This means that
//! they're either 32 bytes (4bpp) or 64 bytes (8bpp). Since VRAM starts aligned
//! to 4, and since both size tiles are a multiple of 4 bytes in size, we model
//! tile data as being arrays of `u32` rather than arrays of `u8`. Having the
//! data stay aligned to 4 within the ROM gives a significant speed gain when
//! copying tiles from ROM into VRAM.
//!
//! The layout of tiles within a background is defined by a "screenblock".
//! * Text backgrounds use a fixed 32x32 size screenblock, with larger
//!   backgrounds using more than one screenblock. Each [TextEntry] value in the
//!   screenblock has a tile index (10-bit), bits for horizontal flip and
//!   vertical flip, and a palbank value. If the background is not in 4bpp mode
//!   the palbank value is simply ignored.
//! * Affine backgrounds always have a single screenblock each, and the size of
//!   the screenblock itself changes with the background's size (from 16x16 to
//!   128x128, in powers of 2). Each entry in an affine screenblock is just a
//!   `u8` tile index, with no special options. Affine backgrounds can't use
//!   4bpp color, and they also can't flip tiles on a per-tile basis.
//!
//! A background's screenblock is selected by an index (5-bit). The indexes go
//! in 2,048 byte (2k) jumps. This is exactly the size of a text screenblock,
//! but doesn't precisely match the size of any of the affine screenblocks.
//!
//! Because tile indexes can only be so large, there are also "charblocks". This
//! offsets all of the tile index values that the background uses, allowing you
//! to make better use of all of the VRAM. The charblock value provides a 16,384
//! byte (16k) offset, and can be in the range `0..=3`.
//!
//! ## Priority
//!
//! When more than one thing would be drawn to the same pixel, there's a
//! priority system that determines which pixel is actually drawn.
//! * Priority values are always 2-bit, the range `0..=3`. The priority acts
//!   like the sorting index, or you could also think of it as the distance from
//!   the viewer. Things with a *lower* priority number are *closer* to the
//!   viewer, and so they'll be what's drawn.
//! * Objects always draw over top a same-priority background.
//! * Lower indexed objects get drawn when two objects have the same priority.
//! * Lower numbered backgrounds get drawn when two backgrounds have the same
//!   priority.
//!
//! There's also one hardware bug that can occur: when there's two objects and
//! their the priority and index wouldn't sort them the same (eg: a lower index
//! number object has a higher priority number), if a background is *also*
//! between the two objects, then the object that's supposed to be behind the
//! background will instead appear through the background where the two objects
//! overlap. This might never happen to you, but if it does, the "fix" is to
//! sort your object entries so that any lower priority objects are also the
//! lower index objects.

use bytemuck::{Pod, TransparentWrapper, Zeroable};

#[allow(unused_imports)]
use crate::prelude::*;
use crate::{
  macros::{
    pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field, u16_int_field,
  },
  mem::{copy_u32x8_unchecked, set_u32x80_unchecked},
};

pub mod obj;

/// An RGB555 color value (packed into `u16`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);
#[allow(clippy::unusual_byte_groupings)]
#[allow(missing_docs)]
impl Color {
  pub const BLACK: Color = Color(0b0_00000_00000_00000);
  pub const RED: Color = Color(0b0_00000_00000_11111);
  pub const GREEN: Color = Color(0b0_00000_11111_00000);
  pub const YELLOW: Color = Color(0b0_00000_11111_11111);
  pub const BLUE: Color = Color(0b0_11111_00000_00000);
  pub const MAGENTA: Color = Color(0b0_11111_00000_11111);
  pub const CYAN: Color = Color(0b0_11111_11111_00000);
  pub const WHITE: Color = Color(0b0_11111_11111_11111);

  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 4, red, with_red);
  u16_int_field!(5 - 9, green, with_green);
  u16_int_field!(10 - 14, blue, with_blue);

  /// Constructs a new color value from the given channel values.
  #[inline]
  #[must_use]
  pub const fn from_rgb(r: u16, g: u16, b: u16) -> Self {
    Self(r & 0b11111 | (g & 0b11111) << 5 | (b & 0b11111) << 10)
  }
}

unsafe impl Zeroable for Color {}
unsafe impl Pod for Color {}
unsafe impl TransparentWrapper<u16> for Color {}

/// The video mode controls how each background layer will operate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum VideoMode {
  /// All four background layers use text mode.
  #[default]
  _0 = 0,
  /// BG0 and BG1 are text mode, while BG2 is affine. BG3 is unavailable.
  _1 = 1,
  /// BG2 and BG3 are affine. BG0 and BG1 are unavailable.
  _2 = 2,
  /// BG2 is a single full color bitmap.
  _3 = 3,
  /// BG2 holds two 8bpp indexmaps, and you can flip between.
  _4 = 4,
  /// BG2 holds two full color bitmaps of reduced size (only 160x128), and you
  /// can flip between.
  _5 = 5,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayControl(u16);
impl DisplayControl {
  pub_const_fn_new_zeroed!();
  u16_enum_field!(0 - 2: VideoMode, video_mode, with_video_mode);
  u16_bool_field!(4, show_frame1, with_show_frame1);
  u16_bool_field!(5, hblank_oam_free, with_hblank_oam_free);
  u16_bool_field!(6, obj_vram_1d, with_obj_vram_1d);
  u16_bool_field!(7, forced_blank, with_forced_blank);
  u16_bool_field!(8, show_bg0, with_show_bg0);
  u16_bool_field!(9, show_bg1, with_show_bg1);
  u16_bool_field!(10, show_bg2, with_show_bg2);
  u16_bool_field!(11, show_bg3, with_show_bg3);
  u16_bool_field!(12, show_obj, with_show_obj);
  u16_bool_field!(13, enable_win0, with_enable_win0);
  u16_bool_field!(14, enable_win1, with_enable_win1);
  u16_bool_field!(15, enable_obj_win, with_enable_obj_win);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayStatus(u16);
impl DisplayStatus {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, currently_vblank, with_currently_vblank);
  u16_bool_field!(1, currently_hblank, with_currently_hblank);
  u16_bool_field!(2, currently_vcount, with_currently_vcount);
  u16_bool_field!(3, irq_vblank, with_irq_vblank);
  u16_bool_field!(4, irq_hblank, with_irq_hblank);
  u16_bool_field!(5, irq_vcount, with_irq_vcount);
  u16_int_field!(8 - 15, vcount_setting, with_vcount_setting);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BackgroundControl(u16);
impl BackgroundControl {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 1, priority, with_priority);
  u16_int_field!(2 - 3, charblock, with_charblock);
  u16_bool_field!(6, mosaic, with_mosaic);
  u16_bool_field!(7, bpp8, with_bpp8);
  u16_int_field!(8 - 12, screenblock, with_screenblock);
  u16_bool_field!(13, is_affine_wrapping, with_is_affine_wrapping);
  u16_int_field!(14 - 15, size, with_size);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowInside(u16);
impl WindowInside {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, win0_bg0, with_win0_bg0);
  u16_bool_field!(1, win0_bg1, with_win0_bg1);
  u16_bool_field!(2, win0_bg2, with_win0_bg2);
  u16_bool_field!(3, win0_bg3, with_win0_bg3);
  u16_bool_field!(4, win0_obj, with_win0_obj);
  u16_bool_field!(5, win0_effect, with_win0_effect);
  u16_bool_field!(8, win1_bg0, with_win1_bg0);
  u16_bool_field!(9, win1_bg1, with_win1_bg1);
  u16_bool_field!(10, win1_bg2, with_win1_bg2);
  u16_bool_field!(11, win1_bg3, with_win1_bg3);
  u16_bool_field!(12, win1_obj, with_win1_obj);
  u16_bool_field!(13, win1_effect, with_win1_effect);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowOutside(u16);
impl WindowOutside {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, outside_bg0, with_outside_bg0);
  u16_bool_field!(1, outside_bg1, with_outside_bg1);
  u16_bool_field!(2, outside_bg2, with_outside_bg2);
  u16_bool_field!(3, outside_bg3, with_outside_bg3);
  u16_bool_field!(4, outside_obj, with_outside_obj);
  u16_bool_field!(5, outside_effect, with_outside_effect);
  u16_bool_field!(8, obj_win_bg0, with_obj_win_bg0);
  u16_bool_field!(9, obj_win_bg1, with_obj_win_bg1);
  u16_bool_field!(10, obj_win_bg2, with_obj_win_bg2);
  u16_bool_field!(11, obj_win_bg3, with_obj_win_bg3);
  u16_bool_field!(12, obj_win_obj, with_obj_win_obj);
  u16_bool_field!(13, obj_win_effect, with_obj_win_effect);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Mosaic(u16);
impl Mosaic {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 3, bg_h_extra, with_bg_h_extra);
  u16_int_field!(4 - 7, bg_v_extra, with_bg_v_extra);
  u16_int_field!(8 - 11, obj_h_extra, with_obj_h_extra);
  u16_int_field!(12 - 15, obj_v_extra, with_obj_v_extra);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ColorEffectMode {
  #[default]
  NoEffect = 0 << 6,
  AlphaBlend = 1 << 6,
  Brighten = 2 << 6,
  Darken = 3 << 6,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BlendControl(u16);
impl BlendControl {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, target1_bg0, with_target1_bg0);
  u16_bool_field!(1, target1_bg1, with_target1_bg1);
  u16_bool_field!(2, target1_bg2, with_target1_bg2);
  u16_bool_field!(3, target1_bg3, with_target1_bg3);
  u16_bool_field!(4, target1_obj, with_target1_obj);
  u16_bool_field!(5, target1_backdrop, with_target1_backdrop);
  u16_enum_field!(6 - 7: ColorEffectMode, mode, with_mode);
  u16_bool_field!(8, target2_bg0, with_target2_bg0);
  u16_bool_field!(9, target2_bg1, with_target2_bg1);
  u16_bool_field!(10, target2_bg2, with_target2_bg2);
  u16_bool_field!(11, target2_bg3, with_target2_bg3);
  u16_bool_field!(12, target2_obj, with_target2_obj);
  u16_bool_field!(13, target2_backdrop, with_target2_backdrop);
}

/// Data for a 4-bit-per-pixel tile.
pub type Tile4 = [u32; 8];

/// Data for an 8-bit-per-pixel tile.
pub type Tile8 = [u32; 16];

/// An entry within a tile mode tilemap.
///
/// * `tile` is the index of the tile, offset from the `charblock` that the
///   background is using. This is a 10-bit value, so indexes are in the range
///   `0..=1023`. You *cannot* index past the end of background VRAM into object
///   VRAM (it just won't draw properly), but you *can* index past the end of
///   one charblock into the next charblock.
/// * `hflip` If you want the tile horizontally flipped.
/// * `vflip` If you want the tile vertically flipped.
/// * `palbank` sets the palbank for this tile. If the background is in 8bpp
///   mode this has no effect.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TextEntry(u16);
impl TextEntry {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 9, tile, with_tile);
  u16_bool_field!(10, hflip, with_hflip);
  u16_bool_field!(11, vflip, with_vflip);
  u16_int_field!(12 - 15, palbank, with_palbank);

  /// Shorthand for `TextEntry::new().with_tile(id)`
  #[inline]
  #[must_use]
  pub const fn from_tile(id: u16) -> Self {
    Self(id & 0b11_1111_1111)
  }

  /// Unwrap this value into its raw `u16` form.
  #[inline]
  #[must_use]
  pub const fn to_u16(self) -> u16 {
    self.0
  }
}

#[inline]
#[cfg(feature = "on_gba")]
pub fn video3_clear_to(c: Color) {
  let u = u32::from(c.0) << 16 | u32::from(c.0);
  unsafe {
    let p = VIDEO3_VRAM.as_usize() as *mut _;
    set_u32x80_unchecked(p, u, 240_usize);
  }
}

#[repr(C, align(4))]
pub struct Video3Bitmap(pub [Color; 240 * 160]);
impl Video3Bitmap {
  /// Wraps an array of raw color bit data as a Video Mode 3 bitmap.
  ///
  /// This is intended for generating static values at compile time. You should
  /// not attempt to call this function at runtime, because the argument to the
  /// function is larger than the GBA's stack space.
  #[inline]
  #[must_use]
  pub const fn new_from_u16(bits: [u16; 240 * 160]) -> Self {
    Self(unsafe { core::mem::transmute(bits) })
  }
}

#[inline]
#[cfg(feature = "on_gba")]
pub fn video3_set_bitmap(bitmap: &Video3Bitmap) {
  let p = VIDEO3_VRAM.as_usize() as *mut _;
  unsafe {
    copy_u32x8_unchecked(p, bitmap as *const _ as *const _, 2400_usize)
  };
}

#[repr(C, align(4))]
pub struct Video4Indexmap(pub [u8; 240 * 160]);

/// Sets the indexmap of the frame requested.
///
/// ## Panics
/// Only frames 0 and 1 exist, if `frame` is 2 or more this will panic.
#[inline]
#[cfg(feature = "on_gba")]
pub fn video4_set_indexmap(indexes: &Video4Indexmap, frame: usize) {
  let p = VIDEO4_VRAM.get_frame(usize::from(frame)).unwrap().as_usize()
    as *mut [u32; 8];
  unsafe {
    copy_u32x8_unchecked(p, indexes as *const _ as *const _, 1200_usize)
  };
}
