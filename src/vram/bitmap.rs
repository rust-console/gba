//! Module for the Bitmap video modes.

use super::*;

/// A bitmap video mode with full color and full resolution.
///
/// * **Width:** 240
/// * **Height:** 160
///
/// Because it takes so much space to have full color and full resolution at the
/// same time, there's no alternate page available when using mode 3.
///
/// As with all the bitmap video modes, the bitmap is considered to be BG2, so
/// you have to enable BG2 as well if you want to see the bitmap.
pub struct Mode3;

impl Mode3 {
  /// The screen's width in this mode.
  pub const WIDTH: usize = 240;

  /// The screen's height in this mode.
  pub const HEIGHT: usize = 160;

  const VRAM: VolBlock<Color, { 256 * 160 }> = unsafe { VolBlock::new(VRAM_BASE_USIZE) };
  const WORDS_BLOCK: VolBlock<u32, { 256 * 160 / 2 }> = unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  /// Gets the address of the pixel specified.
  ///
  /// ## Failure
  ///
  /// Gives `None` if out of bounds
  fn get(col: usize, row: usize) -> Option<VolAddress<Color>> {
    Self::VRAM.get(col + row * Self::WIDTH)
  }

  /// Reads the color of the pixel specified.
  ///
  /// ## Failure
  ///
  /// Gives `None` if out of bounds
  pub fn read(col: usize, row: usize) -> Option<Color> {
    Self::get(col, row).map(VolAddress::read)
  }

  /// Writes a color to the pixel specified.
  ///
  /// ## Failure
  ///
  /// Gives `None` if out of bounds
  pub fn write(col: usize, row: usize, color: Color) -> Option<()> {
    Self::get(col, row).map(|va| va.write(color))
  }

  /// Clear the screen to the color specified.
  ///
  /// Takes ~430,000 cycles (~1.5 frames).
  pub fn clear_to(color: Color) {
    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    for va in Self::WORDS_BLOCK.iter() {
      va.write(bulk_color)
    }
  }

  /// Clears the screen to the color specified using DMA3.
  ///
  /// Takes ~61,500 frames (~73% of VBlank)
  pub fn dma_clear_to(color: Color) {
    use crate::io::dma::DMA3;
    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    unsafe {
      DMA3::fill32(&bulk_color, VRAM_BASE_USIZE as *mut u32, Self::WORDS_BLOCK.len() as u16)
    };
  }

  /// Draws a line between the two points given `(c1,r1,c2,r2,color)`.
  ///
  /// Works fine with out of bounds points. It only draws to in bounds
  /// locations.
  pub fn draw_line(c1: isize, r1: isize, c2: isize, r2: isize, color: Color) {
    let mut col = c1;
    let mut row = r1;
    let w = c2 - c1;
    let h = r2 - r1;
    let mut dx1 = 0;
    let mut dx2 = 0;
    let mut dy1 = 0;
    let mut dy2 = 0;
    let mut longest = w.abs();
    let mut shortest = h.abs();
    if w < 0 {
      dx1 = -1;
    } else if w > 0 {
      dx1 = 1;
    };
    if h < 0 {
      dy1 = -1;
    } else if h > 0 {
      dy1 = 1;
    };
    if w < 0 {
      dx2 = -1;
    } else if w > 0 {
      dx2 = 1;
    };
    if !(longest > shortest) {
      core::mem::swap(&mut longest, &mut shortest);
      if h < 0 {
        dy2 = -1;
      } else if h > 0 {
        dy2 = 1
      };
      dx2 = 0;
    }
    let mut numerator = longest >> 1;

    (0..(longest + 1)).for_each(|_| {
      Self::write(col as usize, row as usize, color);
      numerator += shortest;
      if !(numerator < longest) {
        numerator -= longest;
        col += dx1;
        row += dy1;
      } else {
        col += dx2;
        row += dy2;
      }
    });
  }
}

/// Used to select what page to read from or write to in Mode 4 and Mode 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
  /// Page 0
  Zero,
  /// Page 1
  One,
}

/// A bitmap video mode with full resolution and paletted color.
///
/// * **Width:** 240
/// * **Height:** 160
/// * **Pages:** 2
///
/// Because the pixels use palette indexes there's enough space to have two
/// pages.
///
/// As with all the bitmap video modes, the bitmap is considered to be BG2, so
/// you have to enable BG2 as well if you want to see the bitmap.
pub struct Mode4;

impl Mode4 {
  /// The screen's width in this mode.
  pub const WIDTH: usize = 240;

  /// The screen's height in this mode.
  pub const HEIGHT: usize = 160;

  const PAGE0_INDEXES: VolBlock<u8, { 256 * 160 }> = unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  const PAGE1_INDEXES: VolBlock<u8, { 256 * 160 }> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + PAGE1_OFFSET) };

  const PAGE0_WORDS: VolBlock<u32, { 256 * 160 / 4 }> = unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  const PAGE1_WORDS: VolBlock<u32, { 256 * 160 / 4 }> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + PAGE1_OFFSET) };

  /// Reads the color of the pixel specified.
  ///
  /// ## Failure
  ///
  /// Gives `None` if out of bounds
  pub fn read(page: Page, col: usize, row: usize) -> Option<u8> {
    match page {
      Page::Zero => Self::PAGE0_INDEXES,
      Page::One => Self::PAGE1_INDEXES,
    }
    .get(col + row * Self::WIDTH)
    .map(VolAddress::read)
  }

  /// Writes a color to the pixel specified.
  ///
  /// ## Failure
  ///
  /// Gives `None` if out of bounds
  pub fn write(page: Page, col: usize, row: usize, pal8bpp: u8) -> Option<()> {
    // Note(Lokathor): Byte writes to VRAM aren't permitted, we have to jump
    // through some hoops.
    if col < Self::WIDTH && row < Self::HEIGHT {
      let real_index = col + row * Self::WIDTH;
      let rounded_down_index = real_index & !1;
      let address: VolAddress<u16> = unsafe {
        match page {
          Page::Zero => Self::PAGE0_INDEXES,
          Page::One => Self::PAGE1_INDEXES,
        }
        .index_unchecked(rounded_down_index)
        .cast::<u16>()
      };
      if real_index == rounded_down_index {
        // even byte, change the high bits
        let old_val = address.read();
        address.write((old_val & 0xFF) | ((pal8bpp as u16) << 8));
      } else {
        // odd byte, change the low bits
        let old_val = address.read();
        address.write((old_val & 0xFF00) | pal8bpp as u16);
      }
      Some(())
    } else {
      None
    }
  }

  /// Clear the screen to the palette index specified.
  ///
  /// Takes ~215,000 cycles (~76% of a frame)
  pub fn clear_to(page: Page, pal8bpp: u8) {
    let pal8bpp_32 = pal8bpp as u32;
    let bulk_color = (pal8bpp_32 << 24) | (pal8bpp_32 << 16) | (pal8bpp_32 << 8) | pal8bpp_32;
    let words = match page {
      Page::Zero => Self::PAGE0_WORDS,
      Page::One => Self::PAGE1_WORDS,
    };
    for va in words.iter() {
      va.write(bulk_color)
    }
  }

  /// Clears the screen to the palette index specified using DMA3.
  ///
  /// Takes ~30,800 frames (~37% of VBlank)
  pub fn dma_clear_to(page: Page, pal8bpp: u8) {
    use crate::io::dma::DMA3;

    let pal8bpp_32 = pal8bpp as u32;
    let bulk_color = (pal8bpp_32 << 24) | (pal8bpp_32 << 16) | (pal8bpp_32 << 8) | pal8bpp_32;
    let words_address = unsafe {
      match page {
        Page::Zero => Self::PAGE0_WORDS.index_unchecked(0).to_usize(),
        Page::One => Self::PAGE1_WORDS.index_unchecked(0).to_usize(),
      }
    };
    unsafe { DMA3::fill32(&bulk_color, words_address as *mut u32, Self::PAGE0_WORDS.len() as u16) };
  }

  /// Draws a line between the two points given `(c1,r1,c2,r2,color)`.
  ///
  /// Works fine with out of bounds points. It only draws to in bounds
  /// locations.
  pub fn draw_line(page: Page, c1: isize, r1: isize, c2: isize, r2: isize, pal8bpp: u8) {
    let mut col = c1;
    let mut row = r1;
    let w = c2 - c1;
    let h = r2 - r1;
    let mut dx1 = 0;
    let mut dx2 = 0;
    let mut dy1 = 0;
    let mut dy2 = 0;
    let mut longest = w.abs();
    let mut shortest = h.abs();
    if w < 0 {
      dx1 = -1;
    } else if w > 0 {
      dx1 = 1;
    };
    if h < 0 {
      dy1 = -1;
    } else if h > 0 {
      dy1 = 1;
    };
    if w < 0 {
      dx2 = -1;
    } else if w > 0 {
      dx2 = 1;
    };
    if !(longest > shortest) {
      core::mem::swap(&mut longest, &mut shortest);
      if h < 0 {
        dy2 = -1;
      } else if h > 0 {
        dy2 = 1
      };
      dx2 = 0;
    }
    let mut numerator = longest >> 1;

    (0..(longest + 1)).for_each(|_| {
      Self::write(page, col as usize, row as usize, pal8bpp);
      numerator += shortest;
      if !(numerator < longest) {
        numerator -= longest;
        col += dx1;
        row += dy1;
      } else {
        col += dx2;
        row += dy2;
      }
    });
  }
}

/// Mode 5 is a bitmap mode with full color and reduced resolution.
///
/// * **Width:** 160
/// * **Height:** 128
/// * **Pages:** 2
///
/// Because of the reduced resolutions there's enough space to have two pages.
///
/// As with all the bitmap video modes, the bitmap is considered to be BG2, so
/// you have to enable BG2 as well if you want to see the bitmap.
pub struct Mode5;

impl Mode5 {
  /// The screen's width in this mode.
  pub const WIDTH: usize = 160;

  /// The screen's height in this mode.
  pub const HEIGHT: usize = 128;

  const PAGE0_PIXELS: VolBlock<Color, { 160 * 128 }> = unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  const PAGE1_PIXELS: VolBlock<Color, { 160 * 128 }> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + PAGE1_OFFSET) };

  const PAGE0_WORDS: VolBlock<u32, { 160 * 128 / 2 }> = unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  const PAGE1_WORDS: VolBlock<u32, { 160 * 128 / 2 }> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + PAGE1_OFFSET) };

  /// Reads the color of the pixel specified.
  ///
  /// ## Failure
  ///
  /// Gives `None` if out of bounds
  pub fn read(page: Page, col: usize, row: usize) -> Option<Color> {
    match page {
      Page::Zero => Self::PAGE0_PIXELS,
      Page::One => Self::PAGE1_PIXELS,
    }
    .get(col + row * Self::WIDTH)
    .map(VolAddress::read)
  }

  /// Writes a color to the pixel specified.
  ///
  /// ## Failure
  ///
  /// Gives `None` if out of bounds
  pub fn write(page: Page, col: usize, row: usize, color: Color) -> Option<()> {
    match page {
      Page::Zero => Self::PAGE0_PIXELS,
      Page::One => Self::PAGE1_PIXELS,
    }
    .get(col + row * Self::WIDTH)
    .map(|va| va.write(color))
  }

  /// Clear the screen to the color specified.
  ///
  /// Takes ~215,000 cycles (~76% of a frame)
  pub fn clear_to(page: Page, color: Color) {
    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    let words = match page {
      Page::Zero => Self::PAGE0_WORDS,
      Page::One => Self::PAGE1_WORDS,
    };
    for va in words.iter() {
      va.write(bulk_color)
    }
  }

  /// Clears the screen to the color specified using DMA3.
  ///
  /// Takes ~30,800 frames (~37% of VBlank)
  pub fn dma_clear_to(page: Page, color: Color) {
    use crate::io::dma::DMA3;

    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    let words_address = unsafe {
      match page {
        Page::Zero => Self::PAGE0_WORDS.index_unchecked(0).to_usize(),
        Page::One => Self::PAGE1_WORDS.index_unchecked(0).to_usize(),
      }
    };
    unsafe { DMA3::fill32(&bulk_color, words_address as *mut u32, Self::PAGE0_WORDS.len() as u16) };
  }

  /// Draws a line between the two points given `(c1,r1,c2,r2,color)`.
  ///
  /// Works fine with out of bounds points. It only draws to in bounds
  /// locations.
  pub fn draw_line(page: Page, c1: isize, r1: isize, c2: isize, r2: isize, color: Color) {
    let mut col = c1;
    let mut row = r1;
    let w = c2 - c1;
    let h = r2 - r1;
    let mut dx1 = 0;
    let mut dx2 = 0;
    let mut dy1 = 0;
    let mut dy2 = 0;
    let mut longest = w.abs();
    let mut shortest = h.abs();
    if w < 0 {
      dx1 = -1;
    } else if w > 0 {
      dx1 = 1;
    };
    if h < 0 {
      dy1 = -1;
    } else if h > 0 {
      dy1 = 1;
    };
    if w < 0 {
      dx2 = -1;
    } else if w > 0 {
      dx2 = 1;
    };
    if !(longest > shortest) {
      core::mem::swap(&mut longest, &mut shortest);
      if h < 0 {
        dy2 = -1;
      } else if h > 0 {
        dy2 = 1
      };
      dx2 = 0;
    }
    let mut numerator = longest >> 1;

    (0..(longest + 1)).for_each(|_| {
      Self::write(page, col as usize, row as usize, color);
      numerator += shortest;
      if !(numerator < longest) {
        numerator -= longest;
        col += dx1;
        row += dy1;
      } else {
        col += dx2;
        row += dy2;
      }
    });
  }
}
