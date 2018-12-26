//! Contains types and definitions for display related IO registers.

use super::*;

/// LCD Control. Read/Write.
///
/// * [gbatek entry](http://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol)
pub const DISPCNT: VolAddress<DisplayControlSetting> = unsafe { VolAddress::new_unchecked(0x400_0000) };

newtype!(
  /// A newtype over the various display control options that you have on a GBA.
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  DisplayControlSetting, u16
);

#[allow(missing_docs)]
impl DisplayControlSetting {
  bool_bits!(u16,
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

  multi_bits!(
    u16,
    [
      (0, 3, mode, DisplayMode, Tiled0, Tiled1, Tiled2, Bitmap3, Bitmap4, Bitmap5 )
    ]
  );
}

/// The six display modes available on the GBA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DisplayMode {
  /// This basically allows for the most different things at once (all layers,
  /// 1024 tiles, two palette modes, etc), but you can't do affine
  /// transformations.
  Tiled0 = 0,
  /// This is a mix of `Tile0` and `Tile2`: BG0 and BG1 run as if in `Tiled0`,
  /// and BG2 runs as if in `Tiled2`.
  Tiled1 = 1,
  /// This allows affine transformations, but only uses BG2 and BG3.
  Tiled2 = 2,
  /// This is the basic bitmap draw mode. The whole screen is a single bitmap.
  /// Uses BG2 only.
  Bitmap3 = 3,
  /// This uses _paletted color_ so that there's enough space to have two pages
  /// at _full resolution_, allowing page flipping. Uses BG2 only.
  Bitmap4 = 4,
  /// This uses _reduced resolution_ so that there's enough space to have two
  /// pages with _full color_, allowing page flipping. Uses BG2 only.
  Bitmap5 = 5,
}

/// Assigns the given display control setting.
pub fn set_display_control(setting: DisplayControlSetting) {
  DISPCNT.write(setting);
}
/// Obtains the current display control setting.
pub fn display_control() -> DisplayControlSetting {
  DISPCNT.read()
}

/// If the `VCOUNT` register reads equal to or above this then you're in vblank.
pub const VBLANK_SCANLINE: u16 = 160;

/// Vertical Counter (LY). Read only.
///
/// Gives the current scanline that the display controller is working on. If
/// this is at or above the `VBLANK_SCANLINE` value then the display controller
/// is in a "vertical blank" period.
pub const VCOUNT: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0006) };

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
