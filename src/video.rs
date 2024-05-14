//!

use bitfrob::{u16_with_bit, u16_with_value};

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
  #[inline]
  pub const fn with_hblank_oam_free(self, frame1: bool) -> Self {
    Self(u16_with_bit(5, self.0, frame1))
  }
  #[inline]
  pub const fn with_obj_vram_1d(self, frame1: bool) -> Self {
    Self(u16_with_bit(6, self.0, frame1))
  }
  #[inline]
  pub const fn with_forced_blank(self, frame1: bool) -> Self {
    Self(u16_with_bit(7, self.0, frame1))
  }
  #[inline]
  pub const fn with_bg0(self, frame1: bool) -> Self {
    Self(u16_with_bit(8, self.0, frame1))
  }
  #[inline]
  pub const fn with_bg1(self, frame1: bool) -> Self {
    Self(u16_with_bit(9, self.0, frame1))
  }
  #[inline]
  pub const fn with_bg2(self, frame1: bool) -> Self {
    Self(u16_with_bit(10, self.0, frame1))
  }
  #[inline]
  pub const fn with_bg3(self, frame1: bool) -> Self {
    Self(u16_with_bit(11, self.0, frame1))
  }
  #[inline]
  pub const fn with_objects(self, frame1: bool) -> Self {
    Self(u16_with_bit(12, self.0, frame1))
  }
  #[inline]
  pub const fn with_win0(self, frame1: bool) -> Self {
    Self(u16_with_bit(13, self.0, frame1))
  }
  #[inline]
  pub const fn with_win1(self, frame1: bool) -> Self {
    Self(u16_with_bit(14, self.0, frame1))
  }
  #[inline]
  pub const fn with_object_window(self, frame1: bool) -> Self {
    Self(u16_with_bit(15, self.0, frame1))
  }
}
