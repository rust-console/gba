//!

use core::{ffi::c_void, ptr::addr_of};

use bitfrob::{u16_get_bit, u16_with_bit, u16_with_value};

use crate::{
  asm_runtime::nop,
  dma::{DmaControl, DmaSrcAddr},
  mmio::{
    DMA3_CONTROL, DMA3_DESTINATION, DMA3_SOURCE, DMA3_TRANSFER_COUNT,
    MODE3_VRAM,
  },
};

/// A color value.
///
/// This is a bit-packed linear RGB color value with 5 bits per channel:
/// ```text
/// 0bX_BBBBB_GGGGG_RRRRR
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);
impl Color {
  /// Total black
  pub const BLACK: Color = Color(0b0_00000_00000_00000);
  /// Full red
  pub const RED: Color = Color(0b0_00000_00000_11111);
  /// Full green (lime green)
  pub const GREEN: Color = Color(0b0_00000_11111_00000);
  /// Full yellow
  pub const YELLOW: Color = Color(0b0_00000_11111_11111);
  /// Full blue (dark blue)
  pub const BLUE: Color = Color(0b0_11111_00000_00000);
  /// Full magenta (pinkish purple)
  pub const MAGENTA: Color = Color(0b0_11111_00000_11111);
  /// Full cyan (bright light blue)
  pub const CYAN: Color = Color(0b0_11111_11111_00000);
  /// Full white
  pub const WHITE: Color = Color(0b0_11111_11111_11111);

  /// Constructs a new color value from the given channel values.
  #[inline]
  #[must_use]
  pub const fn from_rgb(r: u16, g: u16, b: u16) -> Self {
    Self(r & 0b11111 | (g & 0b11111) << 5 | (b & 0b11111) << 10)
  }
}

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
#[derive(Debug, Clone, Copy, Default)]
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
  pub const fn with_hblank_oam_free(self, free: bool) -> Self {
    Self(u16_with_bit(5, self.0, free))
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
  pub const fn with_obj_vram_1d(self, vram_1d: bool) -> Self {
    Self(u16_with_bit(6, self.0, vram_1d))
  }
  /// When this is set, the display is forced to be blank (white pixels)
  ///
  /// The display won't access any video memory, allowing you to set things up
  /// without any stalls, and the player won't see any intermediate results.
  #[inline]
  pub const fn with_forced_blank(self, forced: bool) -> Self {
    Self(u16_with_bit(7, self.0, forced))
  }
  /// Display background layer 0.
  #[inline]
  pub const fn with_bg0(self, bg0: bool) -> Self {
    Self(u16_with_bit(8, self.0, bg0))
  }
  /// Display background layer 1.
  #[inline]
  pub const fn with_bg1(self, bg1: bool) -> Self {
    Self(u16_with_bit(9, self.0, bg1))
  }
  /// Display background layer 2.
  #[inline]
  pub const fn with_bg2(self, bg2: bool) -> Self {
    Self(u16_with_bit(10, self.0, bg2))
  }
  /// Display background layer 3.
  #[inline]
  pub const fn with_bg3(self, bg3: bool) -> Self {
    Self(u16_with_bit(11, self.0, bg3))
  }
  /// Display the objects.
  #[inline]
  pub const fn with_objects(self, objects: bool) -> Self {
    Self(u16_with_bit(12, self.0, objects))
  }
  /// Use window 0 as part of the window effect.
  #[inline]
  pub const fn with_win0(self, win0: bool) -> Self {
    Self(u16_with_bit(13, self.0, win0))
  }
  /// Use window 1 as part of the window effect.
  #[inline]
  pub const fn with_win1(self, win1: bool) -> Self {
    Self(u16_with_bit(14, self.0, win1))
  }
  /// Use the object window as part of the window effect.
  #[inline]
  pub const fn with_object_window(self, object_window: bool) -> Self {
    Self(u16_with_bit(15, self.0, object_window))
  }
}

/// Gives info about the display state and sets display interrupts.
#[derive(Debug, Clone, Copy, Default)]
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

/// Data for a 4-bit-per-pixel tile.
///
/// The tile is 8 pixels wide and 8 pixels tall. Each pixel is 4 bits, giving an
/// index within the palbank for this tile's visual element. An index of 0 is a
/// "transparent" pixel. For alignment purposes, all the data is bit packed as
/// `u32` values.
///
/// Generally, you are expected to make tile art on your development machine in
/// some way, and then pack it into your ROM as a static value. The data is then
/// copied from ROM into the correct VRAM location at runtime. You are not
/// expected to manipulate particular pixels within a tile at runtime.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Tile4bpp(pub [u32; 8]);

/// Data for a 8-bit-per-pixel tile.
///
/// The tile is 8 pixels wide and 8 pixels tall. Each pixel is 8 bits, giving an
/// index within the full palette for this tile's visual element. An index of 0
/// is a "transparent" pixel. For alignment purposes, all the data is bit packed
/// as `u32` values.
///
/// Generally, you are expected to make tile art on your development machine in
/// some way, and then pack it into your ROM as a static value. The data is then
/// copied from ROM into the correct VRAM location at runtime. You are not
/// expected to manipulate particular pixels within a tile at runtime.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Tile8bpp(pub [u32; 16]);

pub struct Mode3;
impl Mode3 {
  pub const WIDTH: u16 = 240;
  pub const HEIGHT: u16 = 160;

  /// Clears the Mode 3 bitmap background to the given color.
  pub fn dma3_clear_to(self, color: Color) {
    let c: u32 = color.0 as u32 | ((color.0 as u32) << 16);
    let src: *const c_void = addr_of!(c).cast();
    unsafe {
      // clear any previous setting
      DMA3_CONTROL.write(DmaControl::new());
      // configure and run
      debug_assert!((src as usize) < 0x0800_0000);
      DMA3_SOURCE.write(src);
      DMA3_DESTINATION.write(MODE3_VRAM.as_usize() as _);
      DMA3_TRANSFER_COUNT.write(Self::WIDTH * Self::HEIGHT / 2);
      DMA3_CONTROL.write(
        DmaControl::new()
          .with_u32_transfer(true)
          .with_src_addr(DmaSrcAddr::Fixed)
          .with_enabled(true),
      );
      nop();
      nop();
      nop();
    };
  }
}
