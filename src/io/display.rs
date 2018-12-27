//! Contains types and definitions for display related IO registers.

use super::*;

/// LCD Control. Read/Write.
///
/// The "force vblank" bit is always set when your rust code first executes.
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
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  DisplayControlSetting,
  u16
);

#[allow(missing_docs)]
impl DisplayControlSetting {
  bool_bits!(
    u16,
    [
      (3, cgb_mode),
      (4, frame1),
      (5, hblank_interval_free),
      (6, oam_memory_1d),
      (7, force_vblank),
      (8, bg0),
      (9, bg1),
      (10, bg2),
      (11, bg3),
      (12, obj),
      (13, win0),
      (14, win1),
      (15, obj_window)
    ]
  );

  multi_bits!(u16, [(0, 3, mode, DisplayMode, Mode0, Mode1, Mode2, Mode3, Mode4, Mode5)]);
}

/// The six display modes available on the GBA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DisplayMode {
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

/// Display Status and IRQ Control.
pub const DISPSTAT: VolAddress<DisplayStatusSetting> = unsafe { VolAddress::new_unchecked(0x400_0004) };

newtype!(
  /// A newtype over display status and interrupt control values.
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  DisplayStatusSetting,
  u16
);

#[allow(missing_docs)]
impl DisplayStatusSetting {
  bool_bits!(
    u16,
    [
      (0, vblank_flag),
      (1, hblank_flag),
      (2, vcounter_flag),
      (3, vblank_irq_enable),
      (4, hblank_irq_enable),
      (5, vcounter_irq_enable),
    ]
  );

  multi_bits!(u16, [(8, 8, vcount_setting)]);
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
