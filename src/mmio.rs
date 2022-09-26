#![cfg_attr(rustfmt, rustfmt::skip)]

//! Contains all the MMIO address definitions for the GBA's components.
//!
//! This module contains *only* the MMIO addresses. The data type definitions
//! for each MMIO control value are stored in the appropriate other modules such
//! as [`video`](crate::video), [`interrupts`](crate::interrupts), etc.
//! 
//! In general, the docs for each address are quite short. If you want to
//! understand how a subsystem of the GBA works, you should read the docs for
//! that system's module, and the data type used by the address.
//! 
//! The GBATEK names (and thus mGBA names) are used for the MMIO addresses by
//! default. However, in some cases (eg: sound) the GBATEK naming is excessively
//! cryptic, and so new names have been created. Whenever a new name is used,
//! the GBATEK name is still listed as a doc alias for that address. If
//! necessary you can just search the GBATEK name in the rustdoc search bar and
//! the search results will show you the new name.
//! 
//! ## Safety
//! 
//! While it's safe to use this crate's data type definitions anywhere (they're
//! just wrappers for ints), the MMIO declarations in this module **must not**
//! be used outside of a GBA. The read and write safety of each address are
//! declared assuming that code is running on a GBA. On any other platform, the
//! declarations are simply incorrect.

use core::{ffi::c_void, mem::size_of};
use bitfrob::u8x2;
use voladdress::{Safe, Unsafe, VolAddress, VolBlock, VolSeries};
use crate::{
  interrupts::IrqBits,
  video::{
    BackgroundControl, Color, DisplayControl, DisplayStatus, WindowInside,
    WindowOutside, Mosaic, BlendControl, Tile4, ObjAttr0, ObjAttr1, ObjAttr2, Tile8, TextEntry
  },
  dma::DmaControl,
  sound::{
    SweepControl, TonePattern, ToneFrequency, WaveBank, WaveLenVolume, WaveFrequency, NoiseLenEnvelope, NoiseFrequency, LeftRightVolume, SoundMix, SoundEnable, SoundBias
  },
  timers::TimerControl, keys::{KeyInput, KeyControl},
};

// Note(Lokathor): This macro lets us stick each address at the start of the
// definition, which lets us easily keep each declaration in address order.
macro_rules! def_mmio {
  ($addr:literal = $name:ident : $t:ty $(; $comment:expr )?) => {
    // redirect a call **without** an alias list to just pass an empty alias list
    def_mmio!($addr = $name/[]: $t $(; $comment)? );
  };
  ($addr:literal = $name:ident / [ $( $alias:literal ),* ]: $t:ty $(; $comment:expr )?) => {
    $(#[doc = $comment])?
    $(#[doc(alias = $alias)])*
    #[allow(missing_docs)]
    pub const $name: $t = unsafe { <$t>::new($addr) };
  };
}

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
def_mmio!(0x0400_0028 = BG2X/["BG2X_L", "BG2X_H"]: VolAddress<i32, (), Safe>; "Background 2 X Reference Point, 8-bits fractional (affine/bitmap modes)");
def_mmio!(0x0400_002C = BG2Y/["BG2Y_L", "BG2Y_H"]: VolAddress<i32, (), Safe>; "Background 2 Y Reference Point, 8-bits fractional (affine/bitmap modes)");

def_mmio!(0x0400_0030 = BG3PA: VolAddress<i16, (), Safe>; "Background 3 Param A (affine mode)");
def_mmio!(0x0400_0032 = BG3PB: VolAddress<i16, (), Safe>; "Background 3 Param B (affine mode)");
def_mmio!(0x0400_0034 = BG3PC: VolAddress<i16, (), Safe>; "Background 3 Param C (affine mode)");
def_mmio!(0x0400_0036 = BG3PD: VolAddress<i16, (), Safe>; "Background 3 Param D (affine mode)");
def_mmio!(0x0400_0038 = BG3X/["BG3X_L", "BG3X_H"]: VolAddress<i32, (), Safe>; "Background 3 X Reference Point, 8-bits fractional (affine/bitmap modes)");
def_mmio!(0x0400_003C = BG3Y/["BG3Y_L", "BG3Y_H"]: VolAddress<i32, (), Safe>; "Background 3 Y Reference Point, 8-bits fractional (affine/bitmap modes)");

def_mmio!(0x0400_0040 = WIN0H: VolAddress<u8x2, (), Safe>; "Window 0 Horizontal: high=left, low=(right+1)");
def_mmio!(0x0400_0042 = WIN1H: VolAddress<u8x2, (), Safe>; "Window 1 Horizontal: high=left, low=(right+1)");
def_mmio!(0x0400_0044 = WIN0V: VolAddress<u8x2, (), Safe>; "Window 0 Vertical: high=top, low=(bottom+1)");
def_mmio!(0x0400_0046 = WIN1V: VolAddress<u8x2, (), Safe>; "Window 1 Vertical: high=top, low=(bottom+1)");
def_mmio!(0x0400_0048 = WININ: VolAddress<WindowInside, Safe, Safe>; "Controls the inside Windows 0 and 1");
def_mmio!(0x0400_004A = WINOUT: VolAddress<WindowOutside, Safe, Safe>; "Controls inside the object window and outside of windows");

def_mmio!(0x0400_004C = MOSAIC: VolAddress<Mosaic, (), Safe>; "Sets the intensity of all mosaic effects");
def_mmio!(0x0400_0050 = BLDCNT: VolAddress<BlendControl, Safe, Safe>; "Sets color blend effects");
def_mmio!(0x0400_0052 = BLDALPHA: VolAddress<u8x2, Safe, Safe>;"Sets EVA(low) and EVB(high) alpha blend coefficients, allows `0..=16`, in 1/16th units");
def_mmio!(0x0400_0054 = BLDY: VolAddress<u8, (), Safe>;"Sets EVY brightness blend coefficient, allows `0..=16`, in 1/16th units");

// Sound

def_mmio!(0x0400_0060 = TONE1_SWEEP/["SOUND1CNT_L","NR10"]: VolAddress<SweepControl, Safe, Safe>; "Tone 1 Sweep");
def_mmio!(0x0400_0062 = TONE1_PATTERN/["SOUND1CNT_H","NR11","NR12"]: VolAddress<TonePattern, Safe, Safe>; "Tone 1 Duty/Len/Envelope");
def_mmio!(0x0400_0064 = TONE1_FREQUENCY/["SOUND1CNT_X","NR13","NR14"]: VolAddress<ToneFrequency, Safe, Safe>; "Tone 1 Frequency/Control");

def_mmio!(0x0400_0068 = TONE2_PATTERN/["SOUND2CNT_L","NR21","NR22"]: VolAddress<TonePattern, Safe, Safe>; "Tone 2 Duty/Len/Envelope");
def_mmio!(0x0400_006C = TONE2_FREQUENCY/["SOUND2CNT_H","NR23","NR24"]: VolAddress<ToneFrequency, Safe, Safe>; "Tone 2 Frequency/Control");

def_mmio!(0x0400_0070 = WAVE_BANK/["SOUND3CNT_L","NR30"]: VolAddress<WaveBank, Safe, Safe>; "Wave banking controls");
def_mmio!(0x0400_0072 = WAVE_LEN_VOLUME/["SOUND3CNT_H","NR31","NR32"]: VolAddress<WaveLenVolume, Safe, Safe>; "Wave Length/Volume");
def_mmio!(0x0400_0074 = WAVE_FREQ/["SOUND3CNT_X","NR33","NR34"]: VolAddress<WaveFrequency, Safe, Safe>; "Wave Frequency/Control");

def_mmio!(0x0400_0078 = NOISE_LEN_ENV/["SOUND4CNT_L","NR41","NR42"]: VolAddress<NoiseLenEnvelope, Safe, Safe>; "Noise Length/Envelope");
def_mmio!(0x0400_007C = NOISE_FREQ/["SOUND4CNT_H","NR43","NR44"]: VolAddress<NoiseFrequency, Safe, Safe>; "Noise Frequency/Control");

def_mmio!(0x0400_0080 = LEFT_RIGHT_VOLUME/["SOUNDCNT_L","NR50","NR51"]: VolAddress<LeftRightVolume, Safe, Safe>;"Left/Right sound control (but GBAs only have one speaker each).");
def_mmio!(0x0400_0082 = SOUND_MIX/["SOUNDCNT_H"]: VolAddress<SoundMix, Safe, Safe>;"Mixes sound sources out to the left and right");
def_mmio!(0x0400_0084 = SOUND_ENABLED/["SOUNDCNT_X"]: VolAddress<SoundEnable, Safe, Safe>;"Sound active flags (r), as well as the sound primary enable (rw).");
def_mmio!(0x0400_0088 = SOUNDBIAS: VolAddress<SoundBias, Safe, Safe>;"Provides a bias to set the 'middle point' of sound output.");

def_mmio!(0x0400_0090 = WAVE_RAM/["WAVE_RAM0_L","WAVE_RAM0_H","WAVE_RAM1_L","WAVE_RAM1_H","WAVE_RAM2_L","WAVE_RAM2_H","WAVE_RAM3_L","WAVE_RAM3_H"]: VolBlock<u32, Safe, Safe, 4>; "Wave memory, `u4`, plays MSB/LSB per byte.");
def_mmio!(0x0400_00A0 = FIFO_A/["FIFO_A_L", "FIFO_A_H"]: VolAddress<u32, (), Safe>; "Pushes 4 `i8` samples into the Sound A buffer.\n\nThe buffer is 32 bytes max, playback is LSB first.");
def_mmio!(0x0400_00A4 = FIFO_B/["FIFO_B_L", "FIFO_B_H"]: VolAddress<u32, (), Safe>; "Pushes 4 `i8` samples into the Sound B buffer.\n\nThe buffer is 32 bytes max, playback is LSB first.");

// DMA

def_mmio!(0x0400_00B0 = DMA0_SRC/["DMA0SAD"]: VolAddress<*const c_void, (), Unsafe>; "DMA0 Source Address (internal memory only)");
def_mmio!(0x0400_00B4 = DMA0_DEST/["DMA0DAD"]: VolAddress<*mut c_void, (), Unsafe>; "DMA0 Destination Address (internal memory only)");
def_mmio!(0x0400_00B8 = DMA0_COUNT/["DMA0CNT_L"]: VolAddress<u16, (), Unsafe>; "DMA0 Transfer Count (14-bit, 0=max)");
def_mmio!(0x0400_00BA = DMA0_CONTROL/["DMA0_CNT_H"]: VolAddress<DmaControl, Safe, Unsafe>; "DMA0 Control Bits");

def_mmio!(0x0400_00BC = DMA1_SRC/["DMA1SAD"]: VolAddress<*const c_void, (), Unsafe>; "DMA1 Source Address (non-SRAM memory)");
def_mmio!(0x0400_00C0 = DMA1_DEST/["DMA1DAD"]: VolAddress<*mut c_void, (), Unsafe>; "DMA1 Destination Address (internal memory only)");
def_mmio!(0x0400_00C4 = DMA1_COUNT/["DMA1CNT_L"]: VolAddress<u16, (), Unsafe>; "DMA1 Transfer Count (14-bit, 0=max)");
def_mmio!(0x0400_00C6 = DMA1_CONTROL/["DMA1_CNT_H"]: VolAddress<DmaControl, Safe, Unsafe>; "DMA1 Control Bits");

def_mmio!(0x0400_00C8 = DMA2_SRC/["DMA2SAD"]: VolAddress<*const c_void, (), Unsafe>; "DMA2 Source Address (non-SRAM memory)");
def_mmio!(0x0400_00CC = DMA2_DEST/["DMA2DAD"]: VolAddress<*mut c_void, (), Unsafe>; "DMA2 Destination Address (internal memory only)");
def_mmio!(0x0400_00D0 = DMA2_COUNT/["DMA2CNT_L"]: VolAddress<u16, (), Unsafe>; "DMA2 Transfer Count (14-bit, 0=max)");
def_mmio!(0x0400_00D2 = DMA2_CONTROL/["DMA2_CNT_H"]: VolAddress<DmaControl, Safe, Unsafe>; "DMA2 Control Bits");

def_mmio!(0x0400_00D4 = DMA3_SRC/["DMA3SAD"]: VolAddress<*const c_void, (), Unsafe>; "DMA3 Source Address (non-SRAM memory)");
def_mmio!(0x0400_00D8 = DMA3_DEST/["DMA3DAD"]: VolAddress<*mut c_void, (), Unsafe>; "DMA3 Destination Address (non-SRAM memory)");
def_mmio!(0x0400_00DC = DMA3_COUNT/["DMA3CNT_L"]: VolAddress<u16, (), Unsafe>; "DMA3 Transfer Count (16-bit, 0=max)");
def_mmio!(0x0400_00DE = DMA3_CONTROL/["DMA3_CNT_H"]: VolAddress<DmaControl, Safe, Unsafe>; "DMA3 Control Bits");

// Timers

def_mmio!(0x0400_0100 = TIMER0_COUNT/["TM0CNT_L"]: VolAddress<u16, Safe, ()>; "Timer 0 Count read");
def_mmio!(0x0400_0100 = TIMER0_RELOAD/["TM0CNT_L"]: VolAddress<u16, (), Safe>; "Timer 0 Reload write");
def_mmio!(0x0400_0102 = TIMER0_CONTROL/["TM0CNT_H"]: VolAddress<TimerControl, Safe, Safe>; "Timer 0 control");

def_mmio!(0x0400_0100 = TIMER1_COUNT/["TM1CNT_L"]: VolAddress<u16, Safe, ()>; "Timer 1 Count read");
def_mmio!(0x0400_0100 = TIMER1_RELOAD/["TM1CNT_L"]: VolAddress<u16, (), Safe>; "Timer 1 Reload write");
def_mmio!(0x0400_0102 = TIMER1_CONTROL/["TM1CNT_H"]: VolAddress<TimerControl, Safe, Safe>; "Timer 1 control");

def_mmio!(0x0400_0100 = TIMER2_COUNT/["TM2CNT_L"]: VolAddress<u16, Safe, ()>; "Timer 2 Count read");
def_mmio!(0x0400_0100 = TIMER2_RELOAD/["TM2CNT_L"]: VolAddress<u16, (), Safe>; "Timer 2 Reload write");
def_mmio!(0x0400_0102 = TIMER2_CONTROL/["TM2CNT_H"]: VolAddress<TimerControl, Safe, Safe>; "Timer 2 control");

def_mmio!(0x0400_0100 = TIMER3_COUNT/["TM3CNT_L"]: VolAddress<u16, Safe, ()>; "Timer 3 Count read");
def_mmio!(0x0400_0100 = TIMER3_RELOAD/["TM3CNT_L"]: VolAddress<u16, (), Safe>; "Timer 3 Reload write");
def_mmio!(0x0400_0102 = TIMER3_CONTROL/["TM3CNT_H"]: VolAddress<TimerControl, Safe, Safe>; "Timer 3 control");

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

def_mmio!(0x0400_0130 = KEYINPUT: VolAddress<KeyInput, Safe, ()>; "Key state data.");
def_mmio!(0x0400_0132 = KEYCNT: VolAddress<KeyControl, Safe, Safe>; "Key control to configure the key interrupt.");

// Serial (part 2)

def_mmio!(0x0400_0134 = RCNT: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0140 = JOYCNT: VolAddress<u16, Safe, Safe>);
def_mmio!(0x0400_0150 = JOY_RECV: VolAddress<u32, Safe, Safe>);
def_mmio!(0x0400_0154 = JOY_TRANS: VolAddress<u32, Safe, Safe>);
def_mmio!(0x0400_0158 = JOYSTAT: VolAddress<u8, Safe, Safe>);

// Interrupts

def_mmio!(0x0400_0200 = IE: VolAddress<IrqBits, Safe, Safe>; "Interrupts Enabled: sets which interrupts will be accepted when a subsystem fires an interrupt");
def_mmio!(0x0400_0202 = IF: VolAddress<IrqBits, Safe, Safe>; "Interrupts Flagged: reads which interrupts are pending, writing bit(s) will clear a pending interrupt.");
def_mmio!(0x0400_0204 = WAITCNT: VolAddress<u16, Safe, Unsafe>; "Wait state control for interfacing with the ROM (can make reading the ROM give garbage when it's mis-configured)");
def_mmio!(0x0400_0208 = IME: VolAddress<bool, Safe, Safe>; "Interrupt Master Enable: Allows turning on/off all interrupts with a single access.");

// Palette RAM (PALRAM)

def_mmio!(0x0500_0000 = BACKDROP_COLOR: VolAddress<Color, Safe, Safe>; "Color that's shown when no BG or OBJ draws to a pixel");
def_mmio!(0x0500_0000 = BG_PALETTE: VolBlock<Color, Safe, Safe, 256>; "Background tile palette entries.");
def_mmio!(0x0500_2000 = OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256>; "Object tile palette entries.");

// Video RAM (VRAM)

def_mmio!(0x0600_0000 = CHARBLOCK0_4BPP: VolBlock<Tile4, Safe, Safe, 512>; "Charblock 0, 4bpp view (512 tiles).");
def_mmio!(0x0600_4000 = CHARBLOCK1_4BPP: VolBlock<Tile4, Safe, Safe, 512>; "Charblock 1, 4bpp view (512 tiles).");
def_mmio!(0x0600_8000 = CHARBLOCK2_4BPP: VolBlock<Tile4, Safe, Safe, 512>; "Charblock 2, 4bpp view (512 tiles).");
def_mmio!(0x0600_C000 = CHARBLOCK3_4BPP: VolBlock<Tile4, Safe, Safe, 512>; "Charblock 3, 4bpp view (512 tiles).");

def_mmio!(0x0600_0000 = CHARBLOCK0_8BPP: VolBlock<Tile8, Safe, Safe, 512>; "Charblock 0, 8bpp view (256 tiles).");
def_mmio!(0x0600_4000 = CHARBLOCK1_8BPP: VolBlock<Tile8, Safe, Safe, 512>; "Charblock 1, 8bpp view (256 tiles).");
def_mmio!(0x0600_8000 = CHARBLOCK2_8BPP: VolBlock<Tile8, Safe, Safe, 512>; "Charblock 2, 8bpp view (256 tiles).");
def_mmio!(0x0600_C000 = CHARBLOCK3_8BPP: VolBlock<Tile8, Safe, Safe, 512>; "Charblock 3, 8bpp view (256 tiles).");

pub type TextScreenBlock = VolBlock<TextEntry, Safe, Safe, {8*8}>;
/// ## Panics
/// * Must be in range `0..=31`
#[inline]
#[must_use]
pub const fn text_screenblock(index: usize) -> TextScreenBlock {
  assert!(index < 32);
  unsafe { VolBlock::new(0x0600_0000 + index * size_of::<[TextEntry;8*8]>()) }
}

pub type AffineScreenBlock0 = VolBlock<u8, Safe, Safe, {16*16}>;
pub type AffineScreenBlock1 = VolBlock<u8, Safe, Safe, {32*32}>;
pub type AffineScreenBlock2 = VolBlock<u8, Safe, Safe, {64*64}>;
pub type AffineScreenBlock3 = VolBlock<u8, Safe, Safe, {128*128}>;

def_mmio!(0x0600_0000 = MODE3_BITMAP: VolBlock<Color, Safe, Safe, {240 * 160}>; "Mode 3 bitmap, 240x160.");

def_mmio!(0x0600_0000 = MODE4_FRAME0: VolBlock<u8x2, Safe, Safe, {(240/2) * 160}>; "Mode 4 indexmap, frame 0, (240/2)x160.");
def_mmio!(0x0600_A000 = MODE4_FRAME1: VolBlock<u8x2, Safe, Safe, {(240/2) * 160}>; "Mode 4 indexmap, frame 1, (240/2)x160.");

def_mmio!(0x0600_0000 = MODE5_FRAME0: VolBlock<Color, Safe, Safe, {160 * 128}>; "Mode 5 bitmap, frame 0, 160x128.");
def_mmio!(0x0600_A000 = MODE5_FRAME1: VolBlock<Color, Safe, Safe, {160 * 128}>; "Mode 5 bitmap, frame 1, 160x128.");

def_mmio!(0x0601_0000 = OBJ_TILES: VolBlock<Tile4, Safe, Safe, 1024>; "Object tiles. In bitmap modes, only indices 512..=1023 are available.");

// Object Attribute Memory (OAM)

def_mmio!(0x0700_0000 = OBJ_ATTR0: VolSeries<ObjAttr0, Safe, Safe, 128, {size_of::<[u16;4]>()}>);
def_mmio!(0x0700_0002 = OBJ_ATTR1: VolSeries<ObjAttr1, Safe, Safe, 128, {size_of::<[u16;4]>()}>);
def_mmio!(0x0700_0004 = OBJ_ATTR2: VolSeries<ObjAttr2, Safe, Safe, 128, {size_of::<[u16;4]>()}>);

def_mmio!(0x0700_0006 = AFFINE_PARAM_A: VolSeries<i16, Safe, Safe, 32, {size_of::<[u16;16]>()}>);
def_mmio!(0x0700_000E = AFFINE_PARAM_B: VolSeries<i16, Safe, Safe, 32, {size_of::<[u16;16]>()}>);
def_mmio!(0x0700_0016 = AFFINE_PARAM_C: VolSeries<i16, Safe, Safe, 32, {size_of::<[u16;16]>()}>);
def_mmio!(0x0700_001E = AFFINE_PARAM_D: VolSeries<i16, Safe, Safe, 32, {size_of::<[u16;16]>()}>);
