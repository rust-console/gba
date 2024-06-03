//! Module for screen-related types and functions.

use bitfrob::{u16_get_bit, u16_with_bit, u16_with_value};

use crate::mmio::MODE3_VRAM;

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

/// Background layer control data.
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct BackgroundControl(u16);
impl BackgroundControl {
  /// Makes a new, empty value.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// Sets the background's priority sorting.
  ///
  /// Lower priority backgrounds are closer to the viewer, and will appear in
  /// front other objects that have *higher* priority, and in front of
  /// backgrounds of *higher* priority. If two backgrounds have the same
  /// priority the lower numbered background is shown.
  #[inline]
  pub const fn with_priority(self, priority: u8) -> Self {
    Self(u16_with_value(0, 1, self.0, priority as u16))
  }
  /// The base charblock value for this background.
  #[inline]
  pub const fn with_charblock(self, charblock: u8) -> Self {
    Self(u16_with_value(2, 3, self.0, charblock as u16))
  }
  /// If this background uses the mosaic effect.
  #[inline]
  pub const fn with_mosaic(self, mosaic: bool) -> Self {
    Self(u16_with_bit(6, self.0, mosaic))
  }
  /// Sets the background to 8-bits-per-pixel
  #[inline]
  pub const fn with_bpp8(self, bpp8: bool) -> Self {
    Self(u16_with_bit(7, self.0, bpp8))
  }
  /// Sets the screenblock which lays out these tiles.
  #[inline]
  pub const fn with_screenblock(self, screenblock: u8) -> Self {
    Self(u16_with_value(8, 12, self.0, screenblock as u16))
  }
  /// If affine pixels that go out of the background's area
  #[inline]
  pub const fn with_affine_wrap(self, wrap: bool) -> Self {
    Self(u16_with_bit(13, self.0, wrap))
  }
  /// The background's size.
  #[inline]
  pub const fn with_size(self, size: u8) -> Self {
    Self(u16_with_value(14, 15, self.0, size as u16))
  }
}

/// Textual tile mode entry.
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct TextEntry(u16);
impl TextEntry {
  /// Makes a new, empty value.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// The tile ID
  #[inline]
  pub const fn with_tile(self, id: u16) -> Self {
    Self(u16_with_value(0, 9, self.0, id))
  }
  /// If the tile should be horizontally flipped
  #[inline]
  pub const fn with_hflip(self, hflip: bool) -> Self {
    Self(u16_with_bit(10, self.0, hflip))
  }
  /// If the tile should be vertically flipped.
  #[inline]
  pub const fn with_vflip(self, vflip: bool) -> Self {
    Self(u16_with_bit(11, self.0, vflip))
  }
  /// The palbank for this tile.
  ///
  /// Only used if the background is set for 4bpp.
  #[inline]
  pub const fn with_palbank(self, palbank: u16) -> Self {
    Self(u16_with_value(12, 15, self.0, palbank))
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
pub struct Tile4(pub [u32; 8]);

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
pub struct Tile8(pub [u32; 16]);

/// A zero-sized type that gives a namespace for Mode 3 related things.
#[derive(Clone, Copy)]
pub struct Mode3;
impl Mode3 {
  /// Width, in pixels, of the Mode 3 bitmap.
  pub const WIDTH_USIZE: usize = 240;

  /// Height, in pixels, of the Mode 3 bitmap.
  pub const HEIGHT_USIZE: usize = 160;

  /// The size, in bytes, of one scanline of the Mode 3 bitmap.
  pub const BYTES_PER_ROW: usize =
    core::mem::size_of::<[Color; Mode3::WIDTH_USIZE]>();

  /// The size, in bytes, of the whole Mode 3 bitmap.
  pub const BYTES_TOTAL: usize = Self::BYTES_PER_ROW * Self::HEIGHT_USIZE;

  /// Clears the entire bitmap to a color of your choosing.
  #[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
  #[cfg_attr(feature = "on_gba", link_section = ".iwram.mode3.clear_to")]
  pub fn clear_to(color: Color) {
    on_gba_or_unimplemented!(unsafe {
      let x: u32 = color.0 as u32 | ((color.0 as u32) << 16);
      // now we spam out that `u32`, 10 stm per loop, 8 times per stm.
      core::arch::asm!(
        "1:",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "stm  {ptr}!, {{r0-r5,r7-r8}}",
        "subs {count}, {count}, #1",
        "bne 1b",

        // The assembler will give us a warning (that we can't easily disable)
        // if the reg_list for `stm` doesn't give the registers in order from
        // low to high, so we just manually pick registers. The count register
        // and the pointer register can be anything else.
        in("r0") x,
        in("r1") x,
        in("r2") x,
        in("r3") x,
        in("r4") x,
        in("r5") x,
        in("r7") x,
        in("r8") x,
        count = inout(reg) 240 => _,
        ptr = inout(reg) crate::mmio::MODE3_VRAM.as_usize() => _,
        options(nostack),
      )
    });
  }

  /// Fills the given rectangle, clipped to the bounds of the bitmap.
  #[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
  pub fn fill_rect_clipped(
    x: u16, y: u16, width: u16, height: u16, color: Color,
  ) {
    on_gba_or_unimplemented!(
      let x_start = x.min(Self::WIDTH_USIZE as u16);
      let x_end = x.saturating_add(width).min(Self::WIDTH_USIZE as u16);
      let x_count = x_end - x_start;
      let y_start = y.min(Self::HEIGHT_USIZE as u16);
      let y_end = y.saturating_add(height).min(Self::HEIGHT_USIZE as u16);
      let y_count = y_end - y_start;
      // base
      let mut p = MODE3_VRAM.as_usize() as *mut Color;
      // go to start y
      p = unsafe { p.byte_add(Self::BYTES_PER_ROW * (y_start as u16 as usize)) };
      // go to start x
      p = unsafe { p.add(x_start as u16 as usize) };
      let mut y_remaining = y_count;
      while y_remaining > 0 {
        let mut within_row = p;
        let mut x_remaining = x_count;
        while x_remaining > 0 {
          unsafe { within_row.write_volatile(color) };
          within_row = unsafe { within_row.add(1) };
          x_remaining -= 1;
        }
        p = unsafe { p.byte_add(Self::BYTES_PER_ROW) };
        y_remaining -= 1;
      }
    );
  }
}
