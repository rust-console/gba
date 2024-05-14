//!

use bitfrob::{u16_get_bit, u16_with_bit, u16_with_value};

/// A color value.
///
/// This is a bit-packed linear RGB color value with 5 bits per channel:
/// ```text
/// 0bX_BBBBB_GGGGG_RRRRR
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);

#[cfg(feature = "bytemuck")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "bytemuck")))]
unsafe impl bytemuck::Zeroable for Color {}
#[cfg(feature = "bytemuck")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "bytemuck")))]
unsafe impl bytemuck::Pod for Color {}

/// Controls the overall background settings.
///
/// The video mode is the most important property here. It controls how most
/// other display-related things will act.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayControl(u16);
impl DisplayControl {
  /// Makes a new, empty value.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// Assigns the background mode.
  ///
  /// ## Panics
  /// There is a debug assert that the `mode` is less than 6.
  #[inline]
  pub const fn with_bg_mode(self, mode: u8) -> Self {
    debug_assert!(mode < 6);
    Self(u16_with_value(0, 2, self.0, mode as u16))
  }
  /// Sets if Frame 1 should be used or not.
  ///
  /// Only has an effect in background modes 4 and 5.
  #[inline]
  pub const fn with_frame1_active(self, frame1: bool) -> Self {
    Self(u16_with_bit(4, self.0, frame1))
  }
  /// During Horizontal-blank, the OAM memory won't be accessed by the display.
  ///
  /// Preventing the display from using OAM during h-blank lets you access the
  /// memory at full speed, and should be done if you want to use DMA with OAM.
  /// However, it gives the display less time for OAM processing, so fewer
  /// objects can be drawn per scanline.
  #[inline]
  pub const fn with_hblank_oam_free(self, frame1: bool) -> Self {
    Self(u16_with_bit(5, self.0, frame1))
  }
  /// Causes the object vram to act as a strictly linear region of tiles.
  ///
  /// If this is *off* (the default), object vram acts like a 2d region that's
  /// 32 tiles wide.
  ///
  /// ```text
  /// +----+----+----+----+...
  /// |  0 |  1 |  2 |  3 |
  /// +----+----+----+----+...
  /// | 32 | 33 | 34 | 35 |
  /// +----+----+----+----+...
  /// | 64 | 65 | 66 | 67 |
  /// +----+----+----+----+...
  /// .    .    .    .    .
  /// .    .    .    .    .
  /// ```
  ///
  /// When an object is more than one tile in size, the object's tiles are
  /// filled from left to right, top row to bottom row. This setting affects the
  /// filling process.
  ///
  /// * In 2d mode, moving "down" a row in the object's tiles also moves down a
  ///   row in the VRAM selection (+32 tile indexes). If an object is 2x2 tiles,
  ///   starting at index 0, then it would use tiles 0, 1, 32, and 33.
  /// * In 1d mode, moving "down" a row in the object's tiles has no special
  ///   effect, and just keeps consuming tiles linearly. If an object is 2x2
  ///   tiles, starting at index 0, then it would use tiles 0, 1, 2, and 3.
  #[inline]
  pub const fn with_obj_vram_1d(self, frame1: bool) -> Self {
    Self(u16_with_bit(6, self.0, frame1))
  }
  /// When this is set, the display is forced to be blank (white pixels)
  ///
  /// The display won't access any video memory, allowing you to set things up
  /// without any stalls, and the player won't see any intermediate results.
  #[inline]
  pub const fn with_forced_blank(self, frame1: bool) -> Self {
    Self(u16_with_bit(7, self.0, frame1))
  }
  /// Display background layer 0.
  #[inline]
  pub const fn with_bg0(self, frame1: bool) -> Self {
    Self(u16_with_bit(8, self.0, frame1))
  }
  /// Display background layer 1.
  #[inline]
  pub const fn with_bg1(self, frame1: bool) -> Self {
    Self(u16_with_bit(9, self.0, frame1))
  }
  /// Display background layer 2.
  #[inline]
  pub const fn with_bg2(self, frame1: bool) -> Self {
    Self(u16_with_bit(10, self.0, frame1))
  }
  /// Display background layer 3.
  #[inline]
  pub const fn with_bg3(self, frame1: bool) -> Self {
    Self(u16_with_bit(11, self.0, frame1))
  }
  /// Display the objects.
  #[inline]
  pub const fn with_objects(self, frame1: bool) -> Self {
    Self(u16_with_bit(12, self.0, frame1))
  }
  /// Use window 0 as part of the window effect.
  #[inline]
  pub const fn with_win0(self, frame1: bool) -> Self {
    Self(u16_with_bit(13, self.0, frame1))
  }
  /// Use window 1 as part of the window effect.
  #[inline]
  pub const fn with_win1(self, frame1: bool) -> Self {
    Self(u16_with_bit(14, self.0, frame1))
  }
  /// Use the object window as part of the window effect.
  #[inline]
  pub const fn with_object_window(self, frame1: bool) -> Self {
    Self(u16_with_bit(15, self.0, frame1))
  }
}

/// Gives info about the display state and sets display interrupts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayStatus(u16);
impl DisplayStatus {
  /// Makes a new, empty value.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// If the display is currently in vertical blank.
  #[inline]
  pub const fn is_currently_vblank(self) -> bool {
    u16_get_bit(0, self.0)
  }
  /// If the display is currently in horizontal blank.
  #[inline]
  pub const fn is_currently_hblank(self) -> bool {
    u16_get_bit(1, self.0)
  }
  /// If the display's vertical count matches the vertical count irq setting.
  #[inline]
  pub const fn is_currently_vcounter(self) -> bool {
    u16_get_bit(2, self.0)
  }
  /// Enables sending vertical blank interrupts.
  #[inline]
  pub const fn with_vblank_irq(self, frame1: bool) -> Self {
    Self(u16_with_bit(3, self.0, frame1))
  }
  /// Enables sending horizontal blank interrupts.
  #[inline]
  pub const fn with_hblank_irq(self, frame1: bool) -> Self {
    Self(u16_with_bit(4, self.0, frame1))
  }
  /// Enables sending vertical count match interrupts.
  #[inline]
  pub const fn with_vcounter_irq(self, frame1: bool) -> Self {
    Self(u16_with_bit(5, self.0, frame1))
  }
  /// The vertical count line to match on.
  #[inline]
  pub const fn with_vcount_irq_line(self, line: u8) -> Self {
    Self(u16_with_value(8, 15, self.0, line as u16))
  }
}
