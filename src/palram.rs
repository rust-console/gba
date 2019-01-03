//! Module that allows interacting with palette memory, (`PALRAM`).
//!
//! The `PALRAM` contains 256 `Color` values for Background use, and 256 `Color`
//! values for Object use.
//!
//! Each block of `PALRAM` can be viewed as "8 bits per pixel" (8bpp), where
//! there's a single palette of 256 entries. It can also be viewed as "4 bits
//! per pixel" (4bpp), where there's 16 "palbank" entries that each have 16
//! slots. **Both** interpretations are correct, simultaneously. If you're a
//! real palette wizard you can carefully arrange for some things to use 4bpp
//! mode while other things use 8bpp mode and have it all look good.
//!
//! ## Transparency
//!
//! In 8bpp mode the 0th palette index is "transparent" when used in an image
//! (giving you 255 usable slots). In 4bpp mode the 0th palbank index _of each
//! palbank_ is considered a transparency pixel (giving you 15 usable slots per
//! palbank).
//!
//! ## Clear Color
//!
//! The 0th palette index of the background palette holds the color that the
//! display will show if no background or object draws over top of a given pixel
//! during rendering.

use super::{
  base::volatile::{VolAddress, VolAddressBlock},
  Color,
};

// TODO: PalIndex newtypes?

/// The `PALRAM` for background colors, 256 slot view.
pub const PALRAM_BG: VolAddressBlock<Color> = unsafe { VolAddressBlock::new_unchecked(VolAddress::new_unchecked(0x500_0000), 256) };

/// The `PALRAM` for object colors, 256 slot view.
pub const PALRAM_OBJ: VolAddressBlock<Color> = unsafe { VolAddressBlock::new_unchecked(VolAddress::new_unchecked(0x500_0200), 256) };

/// Obtains the address of the specified 8bpp background palette slot.
pub const fn index_palram_bg_8bpp(slot: u8) -> VolAddress<Color> {
  // Note(Lokathor): because of the `u8` limit we can't go out of bounds here.
  unsafe { PALRAM_BG.index_unchecked(slot as usize) }
}

/// Obtains the address of the specified 8bpp object palette slot.
pub const fn index_palram_obj_8bpp(slot: u8) -> VolAddress<Color> {
  // Note(Lokathor): because of the `u8` limit we can't go out of bounds here.
  unsafe { PALRAM_OBJ.index_unchecked(slot as usize) }
}

/// Obtains the address of the specified 4bpp background palbank and palslot.
///
/// Accesses `palbank * 16 + palslot`, if this is out of bounds the computation
/// will wrap.
pub const fn index_palram_bg_4bpp(palbank: u8, palslot: u8) -> VolAddress<Color> {
  // Note(Lokathor): because of the `u8` limit we can't go out of bounds here.
  unsafe { PALRAM_BG.index_unchecked(palbank.wrapping_mul(16).wrapping_add(palslot) as usize) }
}

/// Obtains the address of the specified 4bpp object palbank and palslot.
///
/// Accesses `palbank * 16 + palslot`, if this is out of bounds the computation
/// will wrap.
pub const fn index_palram_obj_4bpp(palbank: u8, palslot: u8) -> VolAddress<Color> {
  // Note(Lokathor): because of the `u8` limit we can't go out of bounds here.
  unsafe { PALRAM_OBJ.index_unchecked(palbank.wrapping_mul(16).wrapping_add(palslot) as usize) }
}
