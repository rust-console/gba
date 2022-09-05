//! Contains all the MMIO address definitions for the GBA's components.
//!
//! This module contains *only* the MMIO addresses. The data type definitions
//! for each MMIO control value are stored in the appropriate other modules such
//! as [`video`](crate::video), [`interrupts`](crate::interrupts), etc.

// Note(Lokathor): This macro lets us stick each address at the start of the
// definition, which lets us easily keep each declaration in address order.
macro_rules! def_mmio {
  ($addr:literal = $name:ident : $t:ty $(; $comment:expr )?) => {
    $(#[doc = $comment])?
    #[allow(missing_docs)]
    pub const $name: $t = unsafe { <$t>::new($addr) };
  };
}

// Note(Lokathor): this is a dumb hacky module trick so that rustfmt won't touch
// these declarations. Each declaration should be on a single line, even if the
// lines get long.
pub use shut_up_rustfmt::*;
#[rustfmt::skip]
mod shut_up_rustfmt {
  use core::ffi::c_void;
  use voladdress::{Safe, Unsafe, VolAddress, VolBlock};
  use crate::{
    interrupts::IrqBits,
    video::{BackgroundControl, Color, DisplayControl, DisplayStatus},
  };

  def_mmio!(0x0400_0000 = DISPCNT: VolAddress<DisplayControl, Safe, Safe>; "Display Control");
  def_mmio!(0x0400_0004 = DISPSTAT: VolAddress<DisplayStatus, Safe, Safe>; "Display Status");
  def_mmio!(0x0400_0006 = VCOUNT: VolAddress<u16, Safe, ()>; "Vertical Counter");
  def_mmio!(0x0400_0008 = BG0CNT: VolAddress<BackgroundControl, Safe, Safe>; "Background 0 Control");
  def_mmio!(0x0400_000A = BG1CNT: VolAddress<BackgroundControl, Safe, Safe>; "Background 1 Control");
  def_mmio!(0x0400_000C = BG2CNT: VolAddress<BackgroundControl, Safe, Safe>; "Background 2 Control");
  def_mmio!(0x0400_000E = BG3CNT: VolAddress<BackgroundControl, Safe, Safe>; "Background 3 Control");
  def_mmio!(0x0400_0010 = BG0HOFS: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0012 = BG0VOFS: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0014 = BG1HOFS: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0016 = BG1VOFS: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0018 = BG2HOFS: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_001A = BG2VOFS: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_001C = BG3HOFS: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_001E = BG3VOFS: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0020 = BG2PA: VolAddress<i16, (), Safe>);
  def_mmio!(0x0400_0022 = BG2PB: VolAddress<i16, (), Safe>);
  def_mmio!(0x0400_0024 = BG2PC: VolAddress<i16, (), Safe>);
  def_mmio!(0x0400_0026 = BG2PD: VolAddress<i16, (), Safe>);
  def_mmio!(0x0400_0028 = BG2X: VolAddress<i32, (), Safe>);
  def_mmio!(0x0400_002C = BG2Y: VolAddress<i32, (), Safe>);
  def_mmio!(0x0400_0030 = BG3PA: VolAddress<i16, (), Safe>);
  def_mmio!(0x0400_0032 = BG3PB: VolAddress<i16, (), Safe>);
  def_mmio!(0x0400_0034 = BG3PC: VolAddress<i16, (), Safe>);
  def_mmio!(0x0400_0036 = BG3PD: VolAddress<i16, (), Safe>);
  def_mmio!(0x0400_0038 = BG3X: VolAddress<i32, (), Safe>);
  def_mmio!(0x0400_003C = BG3Y: VolAddress<i32, (), Safe>);
  def_mmio!(0x0400_0040 = WIN0H: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0042 = WIN1H: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0044 = WIN0V: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0046 = WIN1V: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0048 = WININ: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_004A = WINOUT: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_004C = MOSAIC: VolAddress<u16, (), Safe>);
  def_mmio!(0x0400_0050 = BLDCNT: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0052 = BLDALPHA: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0054 = BLDY: VolAddress<u16, (), Safe>);

  def_mmio!(0x0400_0060 = SOUND1CNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0062 = SOUND1CNT_H: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0064 = SOUND1CNT_X: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0068 = SOUND2CNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_006C = SOUND2CNT_H: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0070 = SOUND3CNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0072 = SOUND3CNT_H: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0074 = SOUND3CNT_X: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0078 = SOUND4CNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_007C = SOUND4CNT_H: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0080 = SOUNDCNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0082 = SOUNDCNT_H: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0084 = SOUNDCNT_X: VolAddress<u8, Safe, Safe>);
  def_mmio!(0x0400_0088 = SOUNDBIAS: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0090 = WAVE_RAM: VolBlock<u16, Safe, Safe, 8>);
  def_mmio!(0x0400_00A0 = FIFO_A: VolAddress<u32, Safe, ()>);
  def_mmio!(0x0400_00A4 = FIFO_B: VolAddress<u32, Safe, ()>);

  def_mmio!(0x0400_00B0 = DMA0SAD: VolAddress<*const c_void, (), Unsafe>);
  def_mmio!(0x0400_00B4 = DMA0DAD: VolAddress<*mut c_void, (), Unsafe>);
  def_mmio!(0x0400_00B8 = DMA0CNT_L: VolAddress<u16, (), Unsafe>);
  def_mmio!(0x0400_00BA = DMA0CNT_H: VolAddress<u16, Safe, Unsafe>);
  def_mmio!(0x0400_00BC = DMA1SAD: VolAddress<*const c_void, (), Unsafe>);
  def_mmio!(0x0400_00C0 = DMA1DAD: VolAddress<*mut c_void, (), Unsafe>);
  def_mmio!(0x0400_00C4 = DMA1CNT_L: VolAddress<u16, (), Unsafe>);
  def_mmio!(0x0400_00C6 = DMA1CNT_H: VolAddress<u16, Safe, Unsafe>);
  def_mmio!(0x0400_00C8 = DMA2SAD: VolAddress<*const c_void, (), Unsafe>);
  def_mmio!(0x0400_00CC = DMA2DAD: VolAddress<*mut c_void, (), Unsafe>);
  def_mmio!(0x0400_00D0 = DMA2CNT_L: VolAddress<u16, (), Unsafe>);
  def_mmio!(0x0400_00D2 = DMA2CNT_H: VolAddress<u16, Safe, Unsafe>);
  def_mmio!(0x0400_00D4 = DMA3SAD: VolAddress<*const c_void, (), Unsafe>);
  def_mmio!(0x0400_00D8 = DMA3DAD: VolAddress<*mut c_void, (), Unsafe>);
  def_mmio!(0x0400_00DC = DMA3CNT_L: VolAddress<u16, (), Unsafe>);
  def_mmio!(0x0400_00DE = DMA3CNT_H: VolAddress<u16, Safe, Unsafe>);

  def_mmio!(0x0400_0100 = TM0CNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0102 = TM0CNT_H: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0104 = TM1CNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0106 = TM1CNT_H: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0108 = TM2CNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_010A = TM2CNT_H: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_010C = TM3CNT_L: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_010E = TM3CNT_H: VolAddress<u16, Safe, Safe>);

  def_mmio!(0x0400_0120 = SIODATA32: VolAddress<u32, Safe, Safe>);
  def_mmio!(0x0400_0120 = SIOMULTI0: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0122 = SIOMULTI1: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0124 = SIOMULTI2: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0126 = SIOMULTI3: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0128 = SIOCNT: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_012A = SIOMLT_SEND: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_012A = SIODATA8: VolAddress<u8, Safe, Safe>);

  def_mmio!(0x0400_0130 = KEYINPUT: VolAddress<u16, Safe, ()>);
  def_mmio!(0x0400_0132 = KEYCNT: VolAddress<u16, Safe, Safe>);

  def_mmio!(0x0400_0134 = RCNT: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0140 = JOYCNT: VolAddress<u16, Safe, Safe>);
  def_mmio!(0x0400_0150 = JOY_RECV: VolAddress<u32, Safe, Safe>);
  def_mmio!(0x0400_0154 = JOY_TRANS: VolAddress<u32, Safe, Safe>);
  def_mmio!(0x0400_0158 = JOYSTAT: VolAddress<u8, Safe, Safe>);

  def_mmio!(0x0400_0200 = IE: VolAddress<IrqBits, Safe, Safe>);
  def_mmio!(0x0400_0202 = IF: VolAddress<IrqBits, Safe, Unsafe>);
  def_mmio!(0x0400_0204 = WAITCNT: VolAddress<u16, Safe, Unsafe>);
  def_mmio!(0x0400_0208 = IME: VolAddress<bool, Safe, Safe>);

  def_mmio!(0x0500_0000 = BACKDROP_COLOR: VolAddress<Color, Safe, Safe>; "Color that's shown when no BG or OBJ draws to a pixel");
  def_mmio!(0x0500_0000 = BG_PALETTE: VolBlock<Color, Safe, Safe, 256>);
  def_mmio!(0x0500_2000 = OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256>);

  def_mmio!(0x0600_0000 = MODE3_BITMAP: VolBlock<Color, Safe, Safe, {240 * 160}>);
}
