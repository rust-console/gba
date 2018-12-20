//! The module for all things relating to the IO Register portion of the GBA's
//! memory map.
//!
//! Here we define many constants for the volatile pointers to the various IO
//! registers. Each raw register constant is named according to the name given
//! to it in GBATEK's [GBA I/O
//! Map](http://problemkaputt.de/gbatek.htm#gbaiomap). They program in C, and so
//! of course all the names terrible and missing as many vowels as possible.
//! However, being able to look it up online is the most important thing here,
//! so oh well.
//!
//! In addition to the const `VolatilePtr` values, we will over time be adding
//! safe wrappers around each register, including newtypes and such so that you
//! can easily work with whatever each specific register is doing.

// TODO(lokathor): IO Register newtypes.

use gba_proc_macro::register_bit;

use super::*;

/// LCD Control. Read/Write.
///
/// * [gbatek entry](http://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol)
pub const DISPCNT: VolAddress<DisplayControlSetting> = unsafe { VolAddress::new_unchecked(0x400_0000) };

newtype!(
  /// A newtype over the various display control options that you have on a GBA.
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  DisplayControlSetting,
  u16
);

#[allow(missing_docs)]
impl DisplayControlSetting {
  pub const BG_MODE_MASK: u16 = 0b111;

  pub fn mode(self) -> DisplayControlMode {
    match self.0 & Self::BG_MODE_MASK {
      0 => DisplayControlMode::Tiled0,
      1 => DisplayControlMode::Tiled1,
      2 => DisplayControlMode::Tiled2,
      3 => DisplayControlMode::Bitmap3,
      4 => DisplayControlMode::Bitmap4,
      5 => DisplayControlMode::Bitmap5,
      _ => unreachable!(),
    }
  }
  pub fn set_mode(&mut self, new_mode: DisplayControlMode) {
    self.0 &= !Self::BG_MODE_MASK;
    self.0 |= match new_mode {
      DisplayControlMode::Tiled0 => 0,
      DisplayControlMode::Tiled1 => 1,
      DisplayControlMode::Tiled2 => 2,
      DisplayControlMode::Bitmap3 => 3,
      DisplayControlMode::Bitmap4 => 4,
      DisplayControlMode::Bitmap5 => 5,
    };
  }

  register_bit!(CGB_MODE_BIT, u16, 0b1000, cgb_mode);
  register_bit!(PAGE_SELECT_BIT, u16, 0b1_0000, page1_enabled);
  register_bit!(HBLANK_INTERVAL_FREE_BIT, u16, 0b10_0000, hblank_interval_free);
  register_bit!(OBJECT_MEMORY_1D, u16, 0b100_0000, object_memory_1d);
  register_bit!(FORCE_BLANK_BIT, u16, 0b1000_0000, force_blank);
  register_bit!(DISPLAY_BG0_BIT, u16, 0b1_0000_0000, display_bg0);
  register_bit!(DISPLAY_BG1_BIT, u16, 0b10_0000_0000, display_bg1);
  register_bit!(DISPLAY_BG2_BIT, u16, 0b100_0000_0000, display_bg2);
  register_bit!(DISPLAY_BG3_BIT, u16, 0b1000_0000_0000, display_bg3);
  register_bit!(DISPLAY_OBJECT_BIT, u16, 0b1_0000_0000_0000, display_object);
  register_bit!(DISPLAY_WINDOW0_BIT, u16, 0b10_0000_0000_0000, display_window0);
  register_bit!(DISPLAY_WINDOW1_BIT, u16, 0b100_0000_0000_0000, display_window1);
  register_bit!(OBJECT_WINDOW_BIT, u16, 0b1000_0000_0000_0000, display_object_window);
}

/// The six display modes available on the GBA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayControlMode {
  /// This basically allows for the most different things at once (all layers,
  /// 1024 tiles, two palette modes, etc), but you can't do affine
  /// transformations.
  Tiled0,
  /// This is a mix of `Tile0` and `Tile2`: BG0 and BG1 run as if in `Tiled0`,
  /// and BG2 runs as if in `Tiled2`.
  Tiled1,
  /// This allows affine transformations, but only uses BG2 and BG3.
  Tiled2,
  /// This is the basic bitmap draw mode. The whole screen is a single bitmap.
  /// Uses BG2 only.
  Bitmap3,
  /// This uses _paletted color_ so that there's enough space to have two pages
  /// at _full resolution_, allowing page flipping. Uses BG2 only.
  Bitmap4,
  /// This uses _reduced resolution_ so that there's enough space to have two
  /// pages with _full color_, allowing page flipping. Uses BG2 only.
  Bitmap5,
}

/// Assigns the given display control setting.
pub fn set_display_control(setting: DisplayControlSetting) {
  DISPCNT.write(setting);
}
/// Obtains the current display control setting.
pub fn display_control() -> DisplayControlSetting {
  DISPCNT.read()
}

/// Vertical Counter (LY)
pub const VCOUNT: VolAddress<u16> = unsafe { VolAddress::new_unchecked(0x400_0006) };

/// Obtains the current VCount value.
pub fn vcount() -> u16 {
  VCOUNT.read()
}

/// Performs a busy loop until VBlank starts.
pub fn wait_until_vblank() {
  // TODO: make this the better version with BIOS and interrupts and such.
  while vcount() < SCREEN_HEIGHT as u16 {}
}

/// Performs a busy loop until VDraw starts.
pub fn wait_until_vdraw() {
  // TODO: make this the better version with BIOS and interrupts and such.
  while vcount() >= SCREEN_HEIGHT as u16 {}
}
