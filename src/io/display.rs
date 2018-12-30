//! Contains types and definitions for display related IO registers.

use super::*;

/// LCD Control. Read/Write.
///
/// The "force vblank" bit is always set when your Rust code first executes.
pub const DISPCNT: VolAddress<DisplayControlSetting> = unsafe { VolAddress::new_unchecked(0x400_0000) };

newtype!(
  /// Setting for the display control register.
  ///
  /// * 0-2: `DisplayMode`
  /// * 3: CGB mode flag
  /// * 4: Display frame 1 (Modes 4/5 only)
  /// * 5: "hblank interval free", allows full access to OAM during hblank
  /// * 6: Object tile memory 1-dimensional
  /// * 7: Force vblank
  /// * 8: Display bg0 layer
  /// * 9: Display bg1 layer
  /// * 10: Display bg2 layer
  /// * 11: Display bg3 layer
  /// * 12: Display objects layer
  /// * 13: Window 0 display
  /// * 14: Window 1 display
  /// * 15: Object window
  DisplayControlSetting,
  u16
);

#[allow(missing_docs)]
impl DisplayControlSetting {
  phantom_fields! {
    self.0: u16,
    mode: 0-2=DisplayMode<Mode0, Mode1, Mode2, Mode3, Mode4, Mode5>,
    cgb_mode: 3,
    frame1: 4,
    hblank_interval_free: 5,
    oam_memory_1d: 6,
    force_vblank: 7,
    bg0: 8,
    bg1: 9,
    bg2: 10,
    bg3: 11,
    obj: 12,
    win0: 13,
    win1: 14,
    obj_window: 15,
  }
}

newtype_enum! {
  /// The six display modes available on the GBA.
  DisplayMode = u16,
  /// * Affine: No
  /// * Layers: 0/1/2/3
  /// * Size(px): 256x256 to 512x512
  /// * Tiles: 1024
  /// * Palette Modes: 4bpp or 8bpp
  Mode0 = 0,
  /// * BG0 / BG1: As Mode0
  /// * BG2: As Mode2
  Mode1 = 1,
  /// * Affine: Yes
  /// * Layers: 2/3
  /// * Size(px): 128x128 to 1024x1024
  /// * Tiles: 256
  /// * Palette Modes: 8bpp
  Mode2 = 2,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 240x160 (1 page)
  /// * Bitmap
  /// * Full Color
  Mode3 = 3,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 240x160 (2 pages)
  /// * Bitmap
  /// * Palette Modes: 8bpp
  Mode4 = 4,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 160x128 (2 pages)
  /// * Bitmap
  /// * Full Color
  Mode5 = 5,
}

/// Assigns the given display control setting.
pub fn set_display_control(setting: DisplayControlSetting) {
  DISPCNT.write(setting);
}
/// Obtains the current display control setting.
pub fn display_control() -> DisplayControlSetting {
  DISPCNT.read()
}

/// Display Status and IRQ Control. Read/Write.
pub const DISPSTAT: VolAddress<DisplayStatusSetting> = unsafe { VolAddress::new_unchecked(0x400_0004) };

newtype!(
  /// A newtype over display status and interrupt control values.
  DisplayStatusSetting,
  u16
);

impl DisplayStatusSetting {
  phantom_fields! {
    self.0: u16,
    vblank_flag: 0,
    hblank_flag: 1,
    vcounter_flag: 2,
    vblank_irq_enable: 3,
    hblank_irq_enable: 4,
    vcounter_irq_enable: 5,
    vcount_setting: 8-15,
  }
}

/// Vertical Counter (LY). Read only.
///
/// Gives the current scanline that the display controller is working on. If
/// this is at or above the `VBLANK_SCANLINE` value then the display controller
/// is in a "vertical blank" period.
pub const VCOUNT: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0006) };

/// If the `VCOUNT` register reads equal to or above this then you're in vblank.
pub const VBLANK_SCANLINE: u16 = 160;

/// Obtains the current `VCOUNT` value.
pub fn vcount() -> u16 {
  VCOUNT.read()
}

/// Performs a busy loop until VBlank starts.
pub fn spin_until_vblank() {
  // TODO: make this the better version with BIOS and interrupts and such.
  while vcount() < VBLANK_SCANLINE {}
}

/// Performs a busy loop until VDraw starts.
pub fn spin_until_vdraw() {
  // TODO: make this the better version with BIOS and interrupts and such.
  while vcount() >= VBLANK_SCANLINE {}
}

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
  MosaicSetting, u16
}
impl MosaicSetting {
  phantom_fields! {
    self.0: u16,
    bg_horizontal_inc: 0-3,
    bg_vertical_inc: 4-7,
    obj_horizontal_inc: 8-11,
    obj_vertical_inc: 12-15,
  }
}
