//! Module for Background controls

use super::*;

// BG0 Control. Read/Write.
pub const BG0CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_0008) };
// BG1 Control. Read/Write.
pub const BG1CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_000A) };
// BG2 Control. Read/Write.
pub const BG2CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_000C) };
// BG3 Control. Read/Write.
pub const BG3CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_000E) };

newtype! {
  /// Allows configuration of a background layer.
  ///
  /// Bits 0-1: BG Priority (lower number is higher priority, like an index)
  /// Bits 2-3: Character Base Block (0 through 3, 16k each)
  /// Bit 6: Mosaic mode
  /// Bit 7: is 8bpp
  /// Bit 8-12: Screen Base Block (0 through 31, 2k each)
  /// Bit 13: Display area overflow wraps (otherwise transparent, affine only)
  /// Bit 14-15: Screen Size
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  BackgroundControlSetting, u16
}
impl BackgroundControlSetting {
  pub const fn from_screen_base_block(screen_base_block: u16) -> Self {
    BackgroundControlSetting((screen_base_block & 31) << 8)
  }

  //

  bool_bits!(u16, [(6, mosaic), (7, is_8bpp), (13, display_overflow_wrapping)]);

  multi_bits!(
    u16,
    [
      (0, 2, bg_priority),
      (2, 2, char_base_block),
      (8, 5, screen_base_block),
      (2, 2, size, BGSize, Zero, One, Two, Three),
    ]
  );
}

/// The size of a background.
///
/// The meaning changes depending on if the background is Text or Affine mode.
///
/// * In text mode, the screen base block determines where to start reading the
///   tile arrangement data (2k). Size Zero gives one screen block of use. Size
///   One and Two cause two of them to be used (horizontally or vertically,
///   respectively). Size Three is four blocks used, [0,1] above and then [2,3]
///   below. Each screen base block used is always a 32x32 tile grid.
/// * In affine mode, the screen base block determines where to start reading
///   data followed by the size of data as shown. The number of tiles varies
///   according to the size used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum BGSize {
  /// * Text: 256x256px (2k)
  /// * Affine: 128x128px (256b)
  Zero = 0,
  /// * Text: 512x256px (4k)
  /// * Affine: 256x256px (1k)
  One = 1,
  /// * Text: 256x512px (4k)
  /// * Affine: 512x512px (4k)
  Two = 2,
  /// * Text: 512x512px (8k)
  /// * Affine: 1024x1024px (16k)
  Three = 3,
}

// BG0 X-Offset. Write only. Text mode only. 9 bits.
pub const BG0HOFS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0010) };
// BG0 Y-Offset. Write only. Text mode only. 9 bits.
pub const BG0VOFS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0012) };

// BG1 X-Offset. Write only. Text mode only. 9 bits.
pub const BG1HOFS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0012) };
// BG1 Y-Offset. Write only. Text mode only. 9 bits.
pub const BG1VOFS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0012) };

// BG2 X-Offset. Write only. Text mode only. 9 bits.
pub const BG2HOFS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0018) };
// BG2 Y-Offset. Write only. Text mode only. 9 bits.
pub const BG2VOFS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_001A) };

// BG3 X-Offset. Write only. Text mode only. 9 bits.
pub const BG3HOFS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_001C) };
// BG3 Y-Offset. Write only. Text mode only. 9 bits.
pub const BG3VOFS: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_001E) };

// TODO: affine registers:
// BG2X_L
// BG2X_H
// BG2Y_L
// BG2Y_H
// BG2PA
// BG2PB
// BG2PC
// BG2PD
// BG3PA
// BG3PB
// BG3PC
// BG3PD

// TODO: windowing
// pub const WIN0H: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0040) };
// pub const WIN1H: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0042) };
// pub const WIN0V: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0044) };
// pub const WIN1V: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0046) };
// pub const WININ: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0048) };
// pub const WINOUT: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_004A) };

/// Global mosaic effect control. Write-only.
pub const MOSAIC: VolAddress<MosaicSetting> = unsafe { VolAddress::new_unchecked(0x400_004C) };

newtype! {
  /// Allows control of the Mosaic effect.
  ///
  /// Values are the _increase_ for each top-left pixel to be duplicated in the
  /// final result. If you want to duplicate some other pixel than the top-left,
  /// you can offset the background or object by an appropriate amount.
  ///
  /// 0) No effect (1+0)
  /// 1) Each pixel becomes 2 pixels (1+1)
  /// 2) Each pixel becomes 3 pixels (1+2)
  /// 3) Each pixel becomes 4 pixels (1+3)
  ///
  /// * Bits 0-3: BG mosaic horizontal increase
  /// * Bits 4-7: BG mosaic vertical increase
  /// * Bits 8-11: Object mosaic horizontal increase
  /// * Bits 12-15: Object mosaic vertical increase
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  MosaicSetting, u16
}
impl MosaicSetting {
  multi_bits!(
    u16,
    [
      (0, 4, bg_horizontal_inc),
      (4, 4, bg_vertical_inc),
      (8, 4, obj_horizontal_inc),
      (12, 4, obj_vertical_inc),
    ]
  );
}

// pub const BLDCNT: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0050) };
// pub const BLDALPHA: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0052) };
// pub const BLDY: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0054) };
