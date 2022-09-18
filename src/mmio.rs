#![cfg_attr(rustfmt, rustfmt::skip)]

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

use core::{ffi::c_void, mem::size_of};
use bitfrob::u8x2;
use voladdress::{Safe, Unsafe, VolAddress, VolBlock, VolSeries};
use crate::{
  interrupts::IrqBits,
  video::{BackgroundControl, Color, DisplayControl, DisplayStatus, WindowInside, WindowOutside, Mosaic, BlendControl, Tile4, ObjAttr0, ObjAttr1, ObjAttr2}, dma::DmaControl,
};

// Video

def_mmio!(0x0400_0000 = DISPCNT: VolAddress<DisplayControl, Safe, Safe>; "Display Control");
def_mmio!(0x0400_0004 = DISPSTAT: VolAddress<DisplayStatus, Safe, Safe>; "Display Status");
def_mmio!(0x0400_0006 = VCOUNT: VolAddress<u16, Safe, ()>; "Vertical Counter");
def_mmio!(0x0400_0008 = BG0CNT: VolAddress<BackgroundControl, Safe, Safe>; "Background 0 Control");
def_mmio!(0x0400_000A = BG1CNT: VolAddress<BackgroundControl, Safe, Safe>; "Background 1 Control");
def_mmio!(0x0400_000C = BG2CNT: VolAddress<BackgroundControl, Safe, Safe>; "Background 2 Control");
def_mmio!(0x0400_000E = BG3CNT: VolAddress<BackgroundControl, Safe, Safe>; "Background 3 Control");
def_mmio!(0x0400_0010 = BG0HOFS: VolAddress<u16, (), Safe>; "Background 0 Horizontal Offset, wrapped to `0..=511`, (text mode)");
def_mmio!(0x0400_0012 = BG0VOFS: VolAddress<u16, (), Safe>; "Background 0 Vertical Offset, wrapped to `0..=511`, (text mode)");
def_mmio!(0x0400_0014 = BG1HOFS: VolAddress<u16, (), Safe>; "Background 1 Horizontal Offset, wrapped to `0..=511`, (text mode)");
def_mmio!(0x0400_0016 = BG1VOFS: VolAddress<u16, (), Safe>; "Background 1 Vertical Offset, wrapped to `0..=511`, (text mode)");
def_mmio!(0x0400_0018 = BG2HOFS: VolAddress<u16, (), Safe>; "Background 2 Horizontal Offset, wrapped to `0..=511`, (text mode)");
def_mmio!(0x0400_001A = BG2VOFS: VolAddress<u16, (), Safe>; "Background 2 Vertical Offset, wrapped to `0..=511`, (text mode)");
def_mmio!(0x0400_001C = BG3HOFS: VolAddress<u16, (), Safe>; "Background 3 Horizontal Offset, wrapped to `0..=511`, (text mode)");
def_mmio!(0x0400_001E = BG3VOFS: VolAddress<u16, (), Safe>; "Background 3 Vertical Offset, wrapped to `0..=511`, (text mode)");
def_mmio!(0x0400_0020 = BG2PA: VolAddress<i16, (), Safe>; "Background 2 Param A (affine mode)");
def_mmio!(0x0400_0022 = BG2PB: VolAddress<i16, (), Safe>; "Background 2 Param B (affine mode)");
def_mmio!(0x0400_0024 = BG2PC: VolAddress<i16, (), Safe>; "Background 2 Param C (affine mode)");
def_mmio!(0x0400_0026 = BG2PD: VolAddress<i16, (), Safe>; "Background 2 Param D (affine mode)");
def_mmio!(0x0400_0028 = BG2X: VolAddress<i32, (), Safe>; "Background 2 X Reference Point, 8-bits fractional (affine/bitmap modes)");
def_mmio!(0x0400_002C = BG2Y: VolAddress<i32, (), Safe>; "Background 2 Y Reference Point, 8-bits fractional (affine/bitmap modes)");
def_mmio!(0x0400_0030 = BG3PA: VolAddress<i16, (), Safe>; "Background 3 Param A (affine mode)");
def_mmio!(0x0400_0032 = BG3PB: VolAddress<i16, (), Safe>; "Background 3 Param B (affine mode)");
def_mmio!(0x0400_0034 = BG3PC: VolAddress<i16, (), Safe>; "Background 3 Param C (affine mode)");
def_mmio!(0x0400_0036 = BG3PD: VolAddress<i16, (), Safe>; "Background 3 Param D (affine mode)");
def_mmio!(0x0400_0038 = BG3X: VolAddress<i32, (), Safe>; "Background 3 X Reference Point, 8-bits fractional (affine/bitmap modes)");
def_mmio!(0x0400_003C = BG3Y: VolAddress<i32, (), Safe>; "Background 3 Y Reference Point, 8-bits fractional (affine/bitmap modes)");
def_mmio!(0x0400_0040 = WIN0H: VolAddress<u8x2, (), Safe>; "Window 0 Horizontal: high=left, low=(right+1)");
def_mmio!(0x0400_0042 = WIN1H: VolAddress<u8x2, (), Safe>; "Window 1 Horizontal: high=left, low=(right+1)");
def_mmio!(0x0400_0044 = WIN0V: VolAddress<u8x2, (), Safe>; "Window 0 Vertical: high=top, low=(bottom+1)");
def_mmio!(0x0400_0046 = WIN1V: VolAddress<u8x2, (), Safe>; "Window 1 Vertical: high=top, low=(bottom+1)");
def_mmio!(0x0400_0048 = WININ: VolAddress<WindowInside, Safe, Safe>; "Controls the inside Windows 0 and 1");
def_mmio!(0x0400_004A = WINOUT: VolAddress<WindowOutside, Safe, Safe>; "Controls inside the object window and outside of windows");
def_mmio!(0x0400_004C = MOSAIC: VolAddress<Mosaic, (), Safe>; "Sets the intensity of all mosaic effects");
def_mmio!(0x0400_0050 = BLDCNT: VolAddress<BlendControl, Safe, Safe>; "Sets color blend effects");
def_mmio!(0x0400_0052 = BLDALPHA: VolAddress<u8x2, Safe, Safe>;"Sets EVA(low) and EVB(high) alpha blend coefficients, allows `0..=16`, in 1/16th units");
def_mmio!(0x0400_0054 = BLDY: VolAddress<u16, (), Safe>;"Sets EVY brightness blend coefficient, allows `0..=16`, in 1/16th units");

// Sound

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

// DMA

def_mmio!(0x0400_00B0 = DMA0SAD: VolAddress<*const c_void, (), Unsafe>; "DMA0 Source Address (internal memory only)");
def_mmio!(0x0400_00B4 = DMA0DAD: VolAddress<*mut c_void, (), Unsafe>; "DMA0 Destination Address (internal memory only)");
def_mmio!(0x0400_00B8 = DMA0CNT_L: VolAddress<u16, (), Unsafe>; "DMA0 Transfer Count (14-bit, 0=max)");
def_mmio!(0x0400_00BA = DMA0CNT_H: VolAddress<DmaControl, Safe, Unsafe>; "DMA0 Control Bits");
def_mmio!(0x0400_00BC = DMA1SAD: VolAddress<*const c_void, (), Unsafe>; "DMA1 Source Address (non-SRAM memory)");
def_mmio!(0x0400_00C0 = DMA1DAD: VolAddress<*mut c_void, (), Unsafe>; "DMA1 Destination Address (internal memory only)");
def_mmio!(0x0400_00C4 = DMA1CNT_L: VolAddress<u16, (), Unsafe>; "DMA1 Transfer Count (14-bit, 0=max)");
def_mmio!(0x0400_00C6 = DMA1CNT_H: VolAddress<DmaControl, Safe, Unsafe>; "DMA1 Control Bits");
def_mmio!(0x0400_00C8 = DMA2SAD: VolAddress<*const c_void, (), Unsafe>; "DMA2 Source Address (non-SRAM memory)");
def_mmio!(0x0400_00CC = DMA2DAD: VolAddress<*mut c_void, (), Unsafe>; "DMA2 Destination Address (internal memory only)");
def_mmio!(0x0400_00D0 = DMA2CNT_L: VolAddress<u16, (), Unsafe>; "DMA2 Transfer Count (14-bit, 0=max)");
def_mmio!(0x0400_00D2 = DMA2CNT_H: VolAddress<DmaControl, Safe, Unsafe>; "DMA2 Control Bits");
def_mmio!(0x0400_00D4 = DMA3SAD: VolAddress<*const c_void, (), Unsafe>; "DMA3 Source Address (non-SRAM memory)");
def_mmio!(0x0400_00D8 = DMA3DAD: VolAddress<*mut c_void, (), Unsafe>; "DMA3 Destination Address (non-SRAM memory)");
def_mmio!(0x0400_00DC = DMA3CNT_L: VolAddress<u16, (), Unsafe>; "DMA3 Transfer Count (16-bit, 0=max)");
def_mmio!(0x0400_00DE = DMA3CNT_H: VolAddress<DmaControl, Safe, Unsafe>; "DMA3 Control Bits");

// Timers

def_mmio!(0x0400_0100 = TM0CNT_L: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0102 = TM0CNT_H: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0104 = TM1CNT_L: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0106 = TM1CNT_H: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0108 = TM2CNT_L: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_010A = TM2CNT_H: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_010C = TM3CNT_L: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_010E = TM3CNT_H: VolAddress<u16, Safe, Safe>);

// Serial (part 1)

def_mmio!(0x0400_0120 = SIODATA32: VolAddress<u32, Safe, Safe>);
def_mmio!(0x0400_0120 = SIOMULTI0: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0122 = SIOMULTI1: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0124 = SIOMULTI2: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0126 = SIOMULTI3: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0128 = SIOCNT: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_012A = SIOMLT_SEND: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_012A = SIODATA8: VolAddress<u8, Safe, Safe>);

// Keys

def_mmio!(0x0400_0130 = KEYINPUT: VolAddress<u16, Safe, ()>);
def_mmio!(0x0400_0132 = KEYCNT: VolAddress<u16, Safe, Safe>);

// Serial (part 2)

def_mmio!(0x0400_0134 = RCNT: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0140 = JOYCNT: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0150 = JOY_RECV: VolAddress<u32, Safe, Safe>);
def_mmio!(0x0400_0154 = JOY_TRANS: VolAddress<u32, Safe, Safe>);
def_mmio!(0x0400_0158 = JOYSTAT: VolAddress<u8, Safe, Safe>);

// Interrupts

def_mmio!(0x0400_0200 = IE: VolAddress<IrqBits, Safe, Safe>);
def_mmio!(0x0400_0202 = IF: VolAddress<IrqBits, Safe, Unsafe>);
def_mmio!(0x0400_0204 = WAITCNT: VolAddress<u16, Safe, Unsafe>);
def_mmio!(0x0400_0208 = IME: VolAddress<bool, Safe, Safe>);

// Palette RAM (PALRAM)

def_mmio!(0x0500_0000 = BACKDROP_COLOR: VolAddress<Color, Safe, Safe>; "Color that's shown when no BG or OBJ draws to a pixel");
def_mmio!(0x0500_0000 = BG_PALETTE: VolBlock<Color, Safe, Safe, 256>);
def_mmio!(0x0500_2000 = OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256>);

// Video RAM (VRAM)

def_mmio!(0x0600_0000 = MODE3_BITMAP: VolBlock<Color, Safe, Safe, {240 * 160}>; "Mode 3 bitmap, 240x160.");

def_mmio!(0x0600_0000 = MODE4_FRAME0: VolBlock<u8x2, Safe, Safe, {(240/2) * 160}>; "Mode 3 indexmap, frame 0, (240/2)x160.");
def_mmio!(0x0600_A000 = MODE4_FRAME1: VolBlock<u8x2, Safe, Safe, {(240/2) * 160}>; "Mode 3 indexmap, frame 1, (240/2)x160.");

def_mmio!(0x0600_0000 = MODE5_FRAME0: VolBlock<Color, Safe, Safe, {160 * 128}>; "Mode 3 bitmap, frame 0, 160x128.");
def_mmio!(0x0600_A000 = MODE5_FRAME1: VolBlock<Color, Safe, Safe, {160 * 128}>; "Mode 3 bitmap, frame 1, 160x128.");

def_mmio!(0x0601_0000 = OBJ_TILES: VolBlock<Tile4, Safe, Safe, 1024>; "Object tiles. In bitmap modes, only indices 512..=1023 are available.");

// Object Attribute Memory (OAM)

def_mmio!(0x0700_0000 = OBJ_ATTR0: VolSeries<ObjAttr0, Safe, Safe, 128, {size_of::<[u16;4]>()}>);
def_mmio!(0x0700_0002 = OBJ_ATTR1: VolSeries<ObjAttr1, Safe, Safe, 128, {size_of::<[u16;4]>()}>);
def_mmio!(0x0700_0004 = OBJ_ATTR2: VolSeries<ObjAttr2, Safe, Safe, 128, {size_of::<[u16;4]>()}>);

def_mmio!(0x0700_0006 = AFFINE_PARAM_A: VolSeries<i16, Safe, Safe, 32, {size_of::<[u16;16]>()}>);
def_mmio!(0x0700_000E = AFFINE_PARAM_B: VolSeries<i16, Safe, Safe, 32, {size_of::<[u16;16]>()}>);
def_mmio!(0x0700_0016 = AFFINE_PARAM_C: VolSeries<i16, Safe, Safe, 32, {size_of::<[u16;16]>()}>);
def_mmio!(0x0700_001E = AFFINE_PARAM_D: VolSeries<i16, Safe, Safe, 32, {size_of::<[u16;16]>()}>);
