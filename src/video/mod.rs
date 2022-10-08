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
//! different background layers will be available for use with either "text",
//! "affine", or "bitmap" mode.
//!
//! In addition to the background layers, there's also an "OBJ" layer. This
//! allows the display of a number of "objects", which can move independently of
//! any background. Generally, one or more objects will be used to display the
//! "sprites" within a game. Because there isn't an exact 1:1 mapping between
//! sprites and objects, these docs will attempt to only talk about objects.
//!
//! ## Color And Bit Depth
//!
//! [Color] values on the GBA are 5-bits-per-channel RGB values. They're always
//! stored packed and aligned to 2, so think of them as being like a `u16`.
//!
//! Because of the GBA's limited memory, image data will rarely be stored with
//! one full color value per pixel. Instead they'll be stored as
//! 4-bits-per-pixel (4bpp) or 8-bits-per-pixel (8bpp). In both cases, each
//! pixel is an index into the PALRAM (either the [`BG_PALETTE`] or
//! [`OBJ_PALETTE`]), which stores the color to draw. This is known as "indexed"
//! or "paletted" color.
//!
//! Each palette has 256 slots. The palettes are always indexed with 8 bits
//! total, but how those bits are determined depends on the bit depth of the
//! image:
//! * 8bpp images index into the full range of the palette directly.
//! * 4bpp images are always associated with a "palbank". The palbank acts as
//!   the upper 4 bits of the index, selecting which block of 16 palette entries
//!   the image will be able to use. Then each 4-bit pixel within the image
//!   indexes into that palbank.
//! * In both 8bpp and 4bpp modes, if a pixel's value is 0 then that pixel is
//!   transparent.
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

use crate::macros::{
  pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field, u16_int_field,
};
#[allow(unused_imports)]
use crate::prelude::*;

pub mod affine_backgrounds;
pub mod bitmap_backgrounds;
pub mod tiled_backgrounds;

/// An RGB555 color value (packed into `u16`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);
impl Color {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 4, red, with_red);
  u16_int_field!(5 - 9, green, with_green);
  u16_int_field!(10 - 14, blue, with_blue);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum VideoMode {
  #[default]
  _0 = 0,
  _1 = 1,
  _2 = 2,
  _3 = 3,
  _4 = 4,
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
/// * `tile_id` is the index of the tile, offset from the `charblock` that the
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
pub struct TileEntry(u16);
impl TileEntry {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 9, tile_id, with_tile_id);
  u16_bool_field!(10, hflip, with_hflip);
  u16_bool_field!(11, vflip, with_vflip);
  u16_int_field!(12 - 15, palbank, with_palbank);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ObjDisplayStyle {
  #[default]
  Normal = 0 << 8,
  Affine = 1 << 8,
  NotDisplayed = 2 << 8,
  DoubleSizeAffine = 3 << 8,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ObjDisplayMode {
  #[default]
  Normal = 0 << 10,
  SemiTransparent = 1 << 10,
  Window = 2 << 10,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ObjShape {
  #[default]
  Square = 0 << 14,
  Horizontal = 1 << 14,
  Vertical = 2 << 14,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjAttr0(u16);
impl ObjAttr0 {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 7, y, with_y);
  u16_enum_field!(8 - 9: ObjDisplayStyle, style, with_style);
  u16_enum_field!(10 - 11: ObjDisplayMode, mode, with_mode);
  u16_bool_field!(12, mosaic, with_mosaic);
  u16_bool_field!(13, bpp8, with_bpp8);
  u16_enum_field!(14 - 15: ObjShape, shape, with_shape);
}

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjAttr2(u16);
impl ObjAttr2 {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 9, tile_id, with_tile_id);
  u16_int_field!(10 - 11, priority, with_priority);
  u16_int_field!(12 - 15, palbank, with_palbank);
}
