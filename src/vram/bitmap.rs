//! Module for the Bitmap video modes.

use super::*;

use core::ops::{Div, Mul};
use typenum::consts::{U128, U160, U2, U256, U4};

/// Mode 3 is a bitmap mode with full color and full resolution.
///
/// * **Width:** 240
/// * **Height:** 160
///
/// Because the memory requirements are so large, there's only a single page
/// available instead of two pages like the other video modes have.
///
/// As with all bitmap modes, the image itself utilizes BG2 for display, so you
/// must have BG2 enabled in addition to being within Mode 3.
pub struct Mode3;
impl Mode3 {
  /// The physical width in pixels of the GBA screen.
  pub const SCREEN_WIDTH: usize = 240;

  /// The physical height in pixels of the GBA screen.
  pub const SCREEN_HEIGHT: usize = 160;

  /// The number of pixels on the screen.
  pub const SCREEN_PIXEL_COUNT: usize = Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT;

  /// The Mode 3 VRAM.
  ///
  /// Use `col + row * SCREEN_WIDTH` to get the address of an individual pixel,
  /// or use the helpers provided in this module.
  pub const VRAM: VolBlock<Color, <U256 as Mul<U160>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  /// private iterator over the pixels, two at a time
  const VRAM_BULK: VolBlock<u32, <<U256 as Mul<U160>>::Output as Div<U2>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  /// Reads the pixel at the given (col,row).
  ///
  /// # Failure
  ///
  /// Gives `None` if out of bounds.
  pub fn read_pixel(col: usize, row: usize) -> Option<Color> {
    Self::VRAM
      .get(col + row * Self::SCREEN_WIDTH)
      .map(VolAddress::read)
  }

  /// Writes the pixel at the given (col,row).
  ///
  /// # Failure
  ///
  /// Gives `None` if out of bounds.
  pub fn write_pixel(col: usize, row: usize, color: Color) -> Option<()> {
    Self::VRAM
      .get(col + row * Self::SCREEN_WIDTH)
      .map(|va| va.write(color))
  }

  /// Clears the whole screen to the desired color.
  pub fn clear_to(color: Color) {
    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    for va in Self::VRAM_BULK.iter() {
      va.write(bulk_color)
    }
  }

  /// Clears the whole screen to the desired color using DMA3.
  pub fn dma_clear_to(color: Color) {
    use crate::io::dma::DMA3;

    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    unsafe {
      DMA3::fill32(
        &bulk_color,
        VRAM_BASE_USIZE as *mut u32,
        (Self::SCREEN_PIXEL_COUNT / 2) as u16,
      )
    };
  }
}

//TODO: Mode3 Iter Scanlines / Pixels?
//TODO: Mode3 Line Drawing?

/// Mode 4 is a bitmap mode with 8bpp paletted color.
///
/// * **Width:** 240
/// * **Height:** 160
/// * **Pages:** 2
///
/// VRAM has a minimum write size of 2 bytes at a time, so writing individual
/// palette entries for the pixels is more costly than with the other bitmap
/// modes.
///
/// As with all bitmap modes, the image itself utilizes BG2 for display, so you
/// must have BG2 enabled in addition to being within Mode 4.
pub struct Mode4;
impl Mode4 {
  /// The physical width in pixels of the GBA screen.
  pub const SCREEN_WIDTH: usize = 240;

  /// The physical height in pixels of the GBA screen.
  pub const SCREEN_HEIGHT: usize = 160;

  /// The number of pixels on the screen.
  pub const SCREEN_PIXEL_COUNT: usize = Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT;

  /// Used for bulk clearing operations.
  const SCREEN_U32_COUNT: usize = Self::SCREEN_PIXEL_COUNT / 4;

  // TODO: newtype this?
  const PAGE0_BLOCK8: VolBlock<u8, <U256 as Mul<U160>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  // TODO: newtype this?
  const PAGE1_BLOCK8: VolBlock<u8, <U256 as Mul<U160>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + 0xA000) };

  // TODO: newtype this?
  const PAGE0_BLOCK16: VolBlock<u16, <<U256 as Mul<U160>>::Output as Div<U2>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  // TODO: newtype this?
  const PAGE1_BLOCK16: VolBlock<u16, <<U256 as Mul<U160>>::Output as Div<U2>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + 0xA000) };

  /// private iterator over the page0 pixels, four at a time
  const PAGE0_BULK32: VolBlock<u32, <<U256 as Mul<U160>>::Output as Div<U4>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  /// private iterator over the page1 pixels, four at a time
  const PAGE1_BULK32: VolBlock<u32, <<U256 as Mul<U160>>::Output as Div<U4>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + 0xA000) };

  /// Reads the pixel at the given (col,row).
  ///
  /// # Failure
  ///
  /// Gives `None` if out of bounds.
  pub fn read_pixel(page1: bool, col: usize, row: usize) -> Option<u8> {
    // Note(Lokathor): byte _reads_ from VRAM are okay.
    if page1 {
      Self::PAGE1_BLOCK8
        .get(col + row * Self::SCREEN_WIDTH)
        .map(VolAddress::read)
    } else {
      Self::PAGE0_BLOCK8
        .get(col + row * Self::SCREEN_WIDTH)
        .map(VolAddress::read)
    }
  }

  /// Writes the pixel at the given (col,row).
  ///
  /// # Failure
  ///
  /// Gives `None` if out of bounds.
  pub fn write_pixel(page1: bool, col: usize, row: usize, pal8bpp: u8) -> Option<()> {
    // Note(Lokathor): byte _writes_ to VRAM are not permitted. We must jump
    // through hoops when we attempt to write just a single byte.
    if col < Self::SCREEN_WIDTH && row < Self::SCREEN_HEIGHT {
      let real_index = col + row * Self::SCREEN_WIDTH;
      let rounded_down_index = real_index & !1;
      let address: VolAddress<u16> = unsafe {
        if page1 {
          Self::PAGE1_BLOCK8.index(rounded_down_index).cast()
        } else {
          Self::PAGE0_BLOCK8.index(rounded_down_index).cast()
        }
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

  /// Writes a "wide" pairing of palette entries to the location specified.
  ///
  /// The page is imagined to be a series of `u16` values rather than `u8`
  /// values, allowing you to write two palette entries side by side as a single
  /// write operation.
  pub fn write_wide_pixel(
    page1: bool, wide_col: usize, row: usize, wide_pal8bpp: u16,
  ) -> Option<()> {
    if wide_col < Self::SCREEN_WIDTH / 2 && row < Self::SCREEN_HEIGHT {
      let wide_index = wide_col + row * Self::SCREEN_WIDTH / 2;
      let address: VolAddress<u16> = if page1 {
        Self::PAGE1_BLOCK16.index(wide_index)
      } else {
        Self::PAGE0_BLOCK16.index(wide_index)
      };
      Some(address.write(wide_pal8bpp))
    } else {
      None
    }
  }

  /// Clears the page to the desired color.
  pub fn clear_page_to(page1: bool, pal8bpp: u8) {
    let pal8bpp_32 = pal8bpp as u32;
    let bulk_color = (pal8bpp_32 << 24) | (pal8bpp_32 << 16) | (pal8bpp_32 << 8) | pal8bpp_32;
    for va in (if page1 {
      Self::PAGE1_BULK32
    } else {
      Self::PAGE0_BULK32
    })
    .iter()
    {
      va.write(bulk_color)
    }
  }

  /// Clears the page to the desired color using DMA3.
  pub fn dma_clear_page_to(page1: bool, pal8bpp: u8) {
    use crate::io::dma::DMA3;

    let pal8bpp_32 = pal8bpp as u32;
    let bulk_color = (pal8bpp_32 << 24) | (pal8bpp_32 << 16) | (pal8bpp_32 << 8) | pal8bpp_32;
    let write_target = if page1 {
      VRAM_BASE_USIZE as *mut u32
    } else {
      (VRAM_BASE_USIZE + 0xA000) as *mut u32
    };
    unsafe { DMA3::fill32(&bulk_color, write_target, Self::SCREEN_U32_COUNT as u16) };
  }
}

//TODO: Mode4 Iter Scanlines / Pixels?
//TODO: Mode4 Line Drawing?

/// Mode 5 is a bitmap mode with full color and reduced resolution.
///
/// * **Width:** 160
/// * **Height:** 128
/// * **Pages:** 2
///
/// Because of the reduced resolution, we're allowed two pages for display.
///
/// As with all bitmap modes, the image itself utilizes BG2 for display, so you
/// must have BG2 enabled in addition to being within Mode 3.
pub struct Mode5;
impl Mode5 {
  /// The physical width in pixels of the GBA screen.
  pub const SCREEN_WIDTH: usize = 160;

  /// The physical height in pixels of the GBA screen.
  pub const SCREEN_HEIGHT: usize = 128;

  /// The number of pixels on the screen.
  pub const SCREEN_PIXEL_COUNT: usize = Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT;

  /// Used for bulk clearing operations.
  const SCREEN_U32_COUNT: usize = Self::SCREEN_PIXEL_COUNT / 2;

  // TODO: newtype this?
  const PAGE0_BLOCK: VolBlock<Color, <U160 as Mul<U128>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  // TODO: newtype this?
  const PAGE1_BLOCK: VolBlock<Color, <U160 as Mul<U128>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + 0xA000) };

  /// private iterator over the page0 pixels, four at a time
  const PAGE0_BULK32: VolBlock<u32, <<U160 as Mul<U128>>::Output as Div<U2>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE) };

  /// private iterator over the page1 pixels, four at a time
  const PAGE1_BULK32: VolBlock<u32, <<U160 as Mul<U128>>::Output as Div<U2>>::Output> =
    unsafe { VolBlock::new(VRAM_BASE_USIZE + 0xA000) };

  /// Reads the pixel at the given (col,row).
  ///
  /// # Failure
  ///
  /// Gives `None` if out of bounds.
  pub fn read_pixel(page1: bool, col: usize, row: usize) -> Option<Color> {
    if page1 {
      Self::PAGE1_BLOCK
        .get(col + row * Self::SCREEN_WIDTH)
        .map(VolAddress::read)
    } else {
      Self::PAGE0_BLOCK
        .get(col + row * Self::SCREEN_WIDTH)
        .map(VolAddress::read)
    }
  }

  /// Writes the pixel at the given (col,row).
  ///
  /// # Failure
  ///
  /// Gives `None` if out of bounds.
  pub fn write_pixel(page1: bool, col: usize, row: usize, color: Color) -> Option<()> {
    if page1 {
      Self::PAGE1_BLOCK
        .get(col + row * Self::SCREEN_WIDTH)
        .map(|va| va.write(color))
    } else {
      Self::PAGE0_BLOCK
        .get(col + row * Self::SCREEN_WIDTH)
        .map(|va| va.write(color))
    }
  }

  /// Clears the whole screen to the desired color.
  pub fn clear_page_to(page1: bool, color: Color) {
    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    for va in (if page1 {
      Self::PAGE1_BULK32
    } else {
      Self::PAGE0_BULK32
    })
    .iter()
    {
      va.write(bulk_color)
    }
  }

  /// Clears the whole screen to the desired color using DMA3.
  pub fn dma_clear_page_to(page1: bool, color: Color) {
    use crate::io::dma::DMA3;

    let color32 = color.0 as u32;
    let bulk_color = color32 << 16 | color32;
    let write_target = if page1 {
      VRAM_BASE_USIZE as *mut u32
    } else {
      (VRAM_BASE_USIZE + 0xA000) as *mut u32
    };
    unsafe { DMA3::fill32(&bulk_color, write_target, Self::SCREEN_U32_COUNT as u16) };
  }
}

//TODO: Mode5 Iter Scanlines / Pixels?
//TODO: Mode5 Line Drawing?
