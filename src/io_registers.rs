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

use gba_proc_macro::{newtype, register_bit};

use super::*;

/// LCD Control. Read/Write.
///
/// * [gbatek entry](http://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol)
pub const DISPCNT: VolatilePtr<u16> = VolatilePtr(0x400_0000 as *mut u16);

newtype!(
  DisplayControlSetting,
  u16,
  "A newtype over the various display control options that you have on a GBA."
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
  unsafe {
    DISPCNT.write(setting.0);
  }
}
/// Obtains the current display control setting.
pub fn display_control() -> DisplayControlSetting {
  unsafe { DisplayControlSetting(DISPCNT.read()) }
}

/// General LCD Status (STAT,LYC)
pub const DISPSTAT: VolatilePtr<u16> = VolatilePtr(0x400_0004 as *mut u16);

/// Vertical Counter (LY)
pub const VCOUNT: VolatilePtr<u16> = VolatilePtr(0x400_0006 as *mut u16);

/// Obtains the current VCount value.
pub fn vcount() -> u16 {
  unsafe { VCOUNT.read() }
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

/// BG0 Control
pub const BG0CNT: VolatilePtr<u16> = VolatilePtr(0x400_0008 as *mut u16);

/// BG1 Control
pub const BG1CNT: VolatilePtr<u16> = VolatilePtr(0x400_000A as *mut u16);

/// BG2 Control
pub const BG2CNT: VolatilePtr<u16> = VolatilePtr(0x400_000C as *mut u16);

/// BG3 Control
pub const BG3CNT: VolatilePtr<u16> = VolatilePtr(0x400_000E as *mut u16);

/// BG0 X-Offset
pub const BG0HOFS: VolatilePtr<u16> = VolatilePtr(0x400_0010 as *mut u16);

/// BG0 Y-Offset
pub const BG0VOFS: VolatilePtr<u16> = VolatilePtr(0x400_0012 as *mut u16);

/// BG1 X-Offset
pub const BG1HOFS: VolatilePtr<u16> = VolatilePtr(0x400_0014 as *mut u16);

/// BG1 Y-Offset
pub const BG1VOFS: VolatilePtr<u16> = VolatilePtr(0x400_0016 as *mut u16);

/// BG2 X-Offset
pub const BG2HOFS: VolatilePtr<u16> = VolatilePtr(0x400_0018 as *mut u16);

/// BG2 Y-Offset
pub const BG2VOFS: VolatilePtr<u16> = VolatilePtr(0x400_001A as *mut u16);

/// BG3 X-Offset
pub const BG3HOFS: VolatilePtr<u16> = VolatilePtr(0x400_001C as *mut u16);

/// BG3 Y-Offset
pub const BG3VOFS: VolatilePtr<u16> = VolatilePtr(0x400_001E as *mut u16);

/// BG2 Rotation/Scaling Parameter A (dx)
pub const BG2PA: VolatilePtr<u16> = VolatilePtr(0x400_0020 as *mut u16);

/// BG2 Rotation/Scaling Parameter B (dmx)
pub const BG2PB: VolatilePtr<u16> = VolatilePtr(0x400_0022 as *mut u16);

/// BG2 Rotation/Scaling Parameter C (dy)
pub const BG2PC: VolatilePtr<u16> = VolatilePtr(0x400_0024 as *mut u16);

/// BG2 Rotation/Scaling Parameter D (dmy)
pub const BG2PD: VolatilePtr<u16> = VolatilePtr(0x400_0026 as *mut u16);

/// BG2 Reference Point X-Coordinate
pub const BG2X: VolatilePtr<u32> = VolatilePtr(0x400_0028 as *mut u32);

/// BG2 Reference Point Y-Coordinate
pub const BG2Y: VolatilePtr<u32> = VolatilePtr(0x400_002C as *mut u32);

/// BG3 Rotation/Scaling Parameter A (dx)
pub const BG3PA: VolatilePtr<u16> = VolatilePtr(0x400_0030 as *mut u16);

/// BG3 Rotation/Scaling Parameter B (dmx)
pub const BG3PB: VolatilePtr<u16> = VolatilePtr(0x400_0032 as *mut u16);

/// BG3 Rotation/Scaling Parameter C (dy)
pub const BG3PC: VolatilePtr<u16> = VolatilePtr(0x400_0034 as *mut u16);

/// BG3 Rotation/Scaling Parameter D (dmy)
pub const BG3PD: VolatilePtr<u16> = VolatilePtr(0x400_0036 as *mut u16);

/// BG3 Reference Point X-Coordinate
pub const BG3X: VolatilePtr<u32> = VolatilePtr(0x400_0038 as *mut u32);

/// BG3 Reference Point Y-Coordinate
pub const BG3Y: VolatilePtr<u32> = VolatilePtr(0x400_003C as *mut u32);

/// Window 0 Horizontal Dimensions
pub const WIN0H: VolatilePtr<u16> = VolatilePtr(0x400_0040 as *mut u16);

/// Window 1 Horizontal Dimensions
pub const WIN1H: VolatilePtr<u16> = VolatilePtr(0x400_0042 as *mut u16);

/// Window 0 Vertical Dimensions
pub const WIN0V: VolatilePtr<u16> = VolatilePtr(0x400_0044 as *mut u16);

/// Window 1 Vertical Dimensions
pub const WIN1V: VolatilePtr<u16> = VolatilePtr(0x400_0046 as *mut u16);

/// Inside of Window 0 and 1
pub const WININ: VolatilePtr<u16> = VolatilePtr(0x400_0048 as *mut u16);

/// Inside of OBJ Window & Outside of Windows
pub const WINOUT: VolatilePtr<u16> = VolatilePtr(0x400_004A as *mut u16);

/// Mosaic Size
pub const MOSAIC: VolatilePtr<u16> = VolatilePtr(0x400_004C as *mut u16);

/// Color Special Effects Selection
pub const BLDCNT: VolatilePtr<u16> = VolatilePtr(0x400_0050 as *mut u16);

/// Alpha Blending Coefficients
pub const BLDALPHA: VolatilePtr<u16> = VolatilePtr(0x400_0052 as *mut u16);

/// Brightness (Fade-In/Out) Coefficient
pub const BLDY: VolatilePtr<u16> = VolatilePtr(0x400_0054 as *mut u16);

/// Channel 1 Sweep register       (NR10)
pub const UND1CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_0060 as *mut u16);

/// Channel 1 Duty/Length/Envelope (NR11, NR12)
pub const UND1CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_0062 as *mut u16);

/// Channel 1 Frequency/Control    (NR13, NR14)
pub const UND1CNT_X: VolatilePtr<u16> = VolatilePtr(0x400_0064 as *mut u16);

/// Channel 2 Duty/Length/Envelope (NR21, NR22)
pub const UND2CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_0068 as *mut u16);

/// Channel 2 Frequency/Control    (NR23, NR24)
pub const UND2CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_006C as *mut u16);

/// Channel 3 Stop/Wave RAM select (NR30)
pub const UND3CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_0070 as *mut u16);

/// Channel 3 Length/Volume        (NR31, NR32)
pub const UND3CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_0072 as *mut u16);

/// Channel 3 Frequency/Control    (NR33, NR34)
pub const UND3CNT_X: VolatilePtr<u16> = VolatilePtr(0x400_0074 as *mut u16);

/// Channel 4 Length/Envelope      (NR41, NR42)
pub const UND4CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_0078 as *mut u16);

/// Channel 4 Frequency/Control    (NR43, NR44)
pub const UND4CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_007C as *mut u16);

/// Control Stereo/Volume/Enable   (NR50, NR51)
pub const UNDCNT_L: VolatilePtr<u16> = VolatilePtr(0x400_0080 as *mut u16);

/// Control Mixing/DMA Control
pub const UNDCNT_H: VolatilePtr<u16> = VolatilePtr(0x400_0082 as *mut u16);

/// Control Sound on/off           (NR52)
pub const UNDCNT_X: VolatilePtr<u16> = VolatilePtr(0x400_0084 as *mut u16);

/// Sound PWM Control
pub const UNDBIAS: VolatilePtr<u16> = VolatilePtr(0x400_0088 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM0_L: VolatilePtr<u16> = VolatilePtr(0x400_0090 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM0_H: VolatilePtr<u16> = VolatilePtr(0x400_0092 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM1_L: VolatilePtr<u16> = VolatilePtr(0x400_0094 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM1_H: VolatilePtr<u16> = VolatilePtr(0x400_0096 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM2_L: VolatilePtr<u16> = VolatilePtr(0x400_0098 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM2_H: VolatilePtr<u16> = VolatilePtr(0x400_009A as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM3_L: VolatilePtr<u16> = VolatilePtr(0x400_009C as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM3_H: VolatilePtr<u16> = VolatilePtr(0x400_009E as *mut u16);

/// Channel A FIFO, Data 0-3
pub const FIFO_A: VolatilePtr<u32> = VolatilePtr(0x400_00A0 as *mut u32);

/// Channel B FIFO, Data 0-3
pub const FIFO_B: VolatilePtr<u32> = VolatilePtr(0x400_00A4 as *mut u32);

/// DMA 0 Source Address
pub const DMA0SAD: VolatilePtr<u32> = VolatilePtr(0x400_00B0 as *mut u32);

/// DMA 0 Destination Address
pub const DMA0DAD: VolatilePtr<u32> = VolatilePtr(0x400_00B4 as *mut u32);

/// DMA 0 Word Count
pub const DMA0CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_00B8 as *mut u16);

/// DMA 0 Control
pub const DMA0CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_00BA as *mut u16);

/// DMA 1 Source Address
pub const DMA1SAD: VolatilePtr<u32> = VolatilePtr(0x400_00BC as *mut u32);

/// DMA 1 Destination Address
pub const DMA1DAD: VolatilePtr<u32> = VolatilePtr(0x400_00C0 as *mut u32);

/// DMA 1 Word Count
pub const DMA1CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_00C4 as *mut u16);

/// DMA 1 Control
pub const DMA1CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_00C6 as *mut u16);

/// DMA 2 Source Address
pub const DMA2SAD: VolatilePtr<u32> = VolatilePtr(0x400_00C8 as *mut u32);

/// DMA 2 Destination Address
pub const DMA2DAD: VolatilePtr<u32> = VolatilePtr(0x400_00CC as *mut u32);

/// DMA 2 Word Count
pub const DMA2CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_00D0 as *mut u16);

/// DMA 2 Control
pub const DMA2CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_00D2 as *mut u16);

/// DMA 3 Source Address
pub const DMA3SAD: VolatilePtr<u32> = VolatilePtr(0x400_00D4 as *mut u32);

/// DMA 3 Destination Address
pub const DMA3DAD: VolatilePtr<u32> = VolatilePtr(0x400_00D8 as *mut u32);

/// DMA 3 Word Count
pub const DMA3CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_00DC as *mut u16);

/// DMA 3 Control
pub const DMA3CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_00DE as *mut u16);

/// Timer 0 Counter/Reload
pub const TM0CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_0100 as *mut u16);

/// Timer 0 Control
pub const TM0CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_0102 as *mut u16);

/// Timer 1 Counter/Reload
pub const TM1CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_0104 as *mut u16);

/// Timer 1 Control
pub const TM1CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_0106 as *mut u16);

/// Timer 2 Counter/Reload
pub const TM2CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_0108 as *mut u16);

/// Timer 2 Control
pub const TM2CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_010A as *mut u16);

/// Timer 3 Counter/Reload
pub const TM3CNT_L: VolatilePtr<u16> = VolatilePtr(0x400_010C as *mut u16);

/// Timer 3 Control
pub const TM3CNT_H: VolatilePtr<u16> = VolatilePtr(0x400_010E as *mut u16);

/// SIO Data (Normal-32bit Mode; shared with below)
pub const SIODATA32: VolatilePtr<u32> = VolatilePtr(0x400_0120 as *mut u32);

/// SIO Data 0 (Parent)    (Multi-Player Mode)
pub const SIOMULTI0: VolatilePtr<u16> = VolatilePtr(0x400_0120 as *mut u16);

/// SIO Data 1 (1st Child) (Multi-Player Mode)
pub const SIOMULTI1: VolatilePtr<u16> = VolatilePtr(0x400_0122 as *mut u16);

/// SIO Data 2 (2nd Child) (Multi-Player Mode)
pub const SIOMULTI2: VolatilePtr<u16> = VolatilePtr(0x400_0124 as *mut u16);

/// SIO Data 3 (3rd Child) (Multi-Player Mode)
pub const SIOMULTI3: VolatilePtr<u16> = VolatilePtr(0x400_0126 as *mut u16);

/// SIO Control Register
pub const SIOCNT: VolatilePtr<u16> = VolatilePtr(0x400_0128 as *mut u16);

/// D SIO Data (Local of MultiPlayer; shared below)
pub const SIOMLT_SEN: VolatilePtr<u16> = VolatilePtr(0x400_012A as *mut u16);

/// SIO Data (Normal-8bit and UART Mode)
pub const SIODATA8: VolatilePtr<u16> = VolatilePtr(0x400_012A as *mut u16);

/// Key Status
pub const KEYINPUT: VolatilePtr<u16> = VolatilePtr(0x400_0130 as *mut u16);

/// A "tribool" value helps us interpret the arrow pad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[allow(missing_docs)]
pub enum TriBool {
  Minus = -1,
  Neutral = 0,
  Plus = 1,
}

newtype!(KeyInputSetting, u16, "A newtype over the key input state of the GBA");

#[allow(missing_docs)]
impl KeyInputSetting {
  register_bit!(A_BIT, u16, 1, a_pressed);
  register_bit!(B_BIT, u16, 1 << 1, b_pressed);
  register_bit!(SELECT_BIT, u16, 1 << 2, select_pressed);
  register_bit!(START_BIT, u16, 1 << 3, start_pressed);
  register_bit!(RIGHT_BIT, u16, 1 << 4, right_pressed);
  register_bit!(LEFT_BIT, u16, 1 << 5, left_pressed);
  register_bit!(UP_BIT, u16, 1 << 6, up_pressed);
  register_bit!(DOWN_BIT, u16, 1 << 7, down_pressed);
  register_bit!(R_BIT, u16, 1 << 8, r_pressed);
  register_bit!(L_BIT, u16, 1 << 9, l_pressed);

  /// Takes the difference between these keys and another set of keys.
  pub fn difference(self, other: KeyInputSetting) -> KeyInputSetting {
    KeyInputSetting(self.0 ^ other.0)
  }

  /// Gives the arrow pad value as a tribool, with Plus being increased column
  /// value (right).
  pub fn column_direction(self) -> TriBool {
    if self.right_pressed() {
      TriBool::Plus
    } else if self.left_pressed() {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }

  /// Gives the arrow pad value as a tribool, with Plus being increased row
  /// value (down).
  pub fn row_direction(self) -> TriBool {
    if self.down_pressed() {
      TriBool::Plus
    } else if self.up_pressed() {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }
}

/// Gets the current state of the keys
pub fn key_input() -> KeyInputSetting {
  // Note(Lokathor): The 10 used bits are "low when pressed" style, but the 6
  // unused bits are always low, so we XOR with this mask to get a result where
  // the only active bits are currently pressed keys.
  unsafe { KeyInputSetting(KEYINPUT.read() ^ 0b0000_0011_1111_1111) }
}

/// Key Interrupt Control
pub const KEYCNT: VolatilePtr<u16> = VolatilePtr(0x400_0132 as *mut u16);

/// SIO Mode Select/General Purpose Data
pub const RCNT: VolatilePtr<u16> = VolatilePtr(0x400_0134 as *mut u16);

/// SIO JOY Bus Control
pub const JOYCNT: VolatilePtr<u16> = VolatilePtr(0x400_0140 as *mut u16);

/// SIO JOY Bus Receive Data
pub const JOY_RECV: VolatilePtr<u32> = VolatilePtr(0x400_0150 as *mut u32);

/// SIO JOY Bus Transmit Data
pub const JOY_TRANS: VolatilePtr<u32> = VolatilePtr(0x400_0154 as *mut u32);

/// SIO JOY Bus Receive Status
pub const JOYSTAT: VolatilePtr<u16> = VolatilePtr(0x400_0158 as *mut u16);

/// Interrupt Enable Register
pub const IE: VolatilePtr<u16> = VolatilePtr(0x400_0200 as *mut u16);

/// Interrupt Request Flags / IRQ Acknowledge
pub const IF: VolatilePtr<u16> = VolatilePtr(0x400_0202 as *mut u16);

/// Game Pak Waitstate Control
pub const WAITCNT: VolatilePtr<u16> = VolatilePtr(0x400_0204 as *mut u16);

/// Interrupt Master Enable Register
pub const IME: VolatilePtr<u16> = VolatilePtr(0x400_0208 as *mut u16);
