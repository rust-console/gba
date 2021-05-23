use crate::prelude::*;

use voladdress::*;

pub mod mode3;

// TODO: modules for the other video modes

/// [DISPCNT](https://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol)
pub const DISPCNT: VolAddress<DisplayControl, Safe, Safe> = unsafe { VolAddress::new(0x0400_0000) };

/// [DISPSTAT](https://problemkaputt.de/gbatek.htm#lcdiointerruptsandstatus)
pub const DISPSTAT: VolAddress<DisplayStatus, Safe, Safe> = unsafe { VolAddress::new(0x0400_0004) };

/// [VCOUNT](https://problemkaputt.de/gbatek.htm#lcdiointerruptsandstatus)
pub const VCOUNT: VolAddress<u8, Safe, ()> = unsafe { VolAddress::new(0x0400_0006) };

/// [BG0CNT](https://problemkaputt.de/gbatek.htm#lcdiobgcontrol)
pub const BG0CNT: VolAddress<BackgroundControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0008) };

/// [BG1CNT](https://problemkaputt.de/gbatek.htm#lcdiobgcontrol)
pub const BG1CNT: VolAddress<BackgroundControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_000A) };

/// [BG2CNT](https://problemkaputt.de/gbatek.htm#lcdiobgcontrol)
pub const BG2CNT: VolAddress<BackgroundControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_000C) };

/// [BG3CNT](https://problemkaputt.de/gbatek.htm#lcdiobgcontrol)
pub const BG3CNT: VolAddress<BackgroundControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_000E) };

/// [BG0HOFS](https://problemkaputt.de/gbatek.htm#lcdiobgscrolling)
pub const BG0HOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_0010) };
/// [BG0VOFS](https://problemkaputt.de/gbatek.htm#lcdiobgscrolling)
pub const BG0VOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_0012) };

/// [BG1HOFS](https://problemkaputt.de/gbatek.htm#lcdiobgscrolling)
pub const BG1HOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_0014) };
/// [BG1VOFS](https://problemkaputt.de/gbatek.htm#lcdiobgscrolling)
pub const BG1VOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_0016) };

/// [BG2HOFS](https://problemkaputt.de/gbatek.htm#lcdiobgscrolling)
pub const BG2HOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_0018) };
/// [BG2VOFS](https://problemkaputt.de/gbatek.htm#lcdiobgscrolling)
pub const BG2VOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_001A) };

/// [BG3HOFS](https://problemkaputt.de/gbatek.htm#lcdiobgscrolling)
pub const BG3HOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_001C) };
/// [BG3VOFS](https://problemkaputt.de/gbatek.htm#lcdiobgscrolling)
pub const BG3VOFS: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_001E) };

/// [BG2PA](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG2PA: VolAddress<i16, (), Safe> = unsafe { VolAddress::new(0x0400_0020) };
/// [BG2PB](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG2PB: VolAddress<i16, (), Safe> = unsafe { VolAddress::new(0x0400_0022) };
/// [BG2PC](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG2PC: VolAddress<i16, (), Safe> = unsafe { VolAddress::new(0x0400_0024) };
/// [BG2PD](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG2PD: VolAddress<i16, (), Safe> = unsafe { VolAddress::new(0x0400_0026) };

/// [BG2X](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG2X: VolAddress<i32, (), Safe> = unsafe { VolAddress::new(0x0400_0028) };
/// [BG2Y](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG2Y: VolAddress<i32, (), Safe> = unsafe { VolAddress::new(0x0400_002C) };

/// [BG3PA](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG3PA: VolAddress<i16, (), Safe> = unsafe { VolAddress::new(0x0400_0030) };
/// [BG3PB](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG3PB: VolAddress<i16, (), Safe> = unsafe { VolAddress::new(0x0400_0032) };
/// [BG3PC](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG3PC: VolAddress<i16, (), Safe> = unsafe { VolAddress::new(0x0400_0034) };
/// [BG3PD](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG3PD: VolAddress<i16, (), Safe> = unsafe { VolAddress::new(0x0400_0036) };

/// [BG3X](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG3X: VolAddress<i32, (), Safe> = unsafe { VolAddress::new(0x0400_0038) };
/// [BG3Y](https://problemkaputt.de/gbatek.htm#lcdiobgrotationscaling)
pub const BG3Y: VolAddress<i32, (), Safe> = unsafe { VolAddress::new(0x0400_003C) };

/// [WIN0H](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN0H_RIGHT: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x0400_0040) };
/// [WIN0H](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN0H_LEFT: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x0400_0041) };

/// [WIN0H](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN1H_RIGHT: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x0400_0042) };
/// [WIN0H](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN1H_LEFT: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x0400_0043) };

/// [WIN0V](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN0V_BOTTOM: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x0400_0044) };
/// [WIN0V](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN0V_TOP: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x0400_0045) };

/// [WIN1V](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN1V_BOTTOM: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x0400_0046) };
/// [WIN1V](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN1V_TOP: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x0400_0047) };

/// [WININ](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN_IN_0: VolAddress<WindowEnable, Safe, Safe> = unsafe { VolAddress::new(0x0400_0048) };
/// [WININ](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN_IN_1: VolAddress<WindowEnable, Safe, Safe> = unsafe { VolAddress::new(0x0400_0049) };
/// [WINOUT](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN_OUT: VolAddress<WindowEnable, Safe, Safe> = unsafe { VolAddress::new(0x0400_004A) };
/// [WINOUT](https://problemkaputt.de/gbatek.htm#lcdiowindowfeature)
pub const WIN_IN_OBJ: VolAddress<WindowEnable, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_004B) };

/// [MOSAIC](https://problemkaputt.de/gbatek.htm#lcdiomosaicfunction)
pub const MOSAIC_BG: VolAddress<MosaicSize, (), Safe> = unsafe { VolAddress::new(0x0400_004C) };
/// [MOSAIC](https://problemkaputt.de/gbatek.htm#lcdiomosaicfunction)
pub const MOSAIC_OBJ: VolAddress<MosaicSize, (), Safe> = unsafe { VolAddress::new(0x0400_004D) };

/// [BLDCNT](https://problemkaputt.de/gbatek.htm#lcdiocolorspecialeffects)
pub const BLDCNT: VolAddress<BlendControl, Safe, Safe> = unsafe { VolAddress::new(0x0400_0050) };

/// [BLDALPHA](https://problemkaputt.de/gbatek.htm#lcdiocolorspecialeffects)
pub const BLDALPHA_A: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x0400_0052) };

/// [BLDALPHA](https://problemkaputt.de/gbatek.htm#lcdiocolorspecialeffects)
pub const BLDALPHA_B: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x0400_0053) };

/// [BLDY](https://problemkaputt.de/gbatek.htm#lcdiocolorspecialeffects)
pub const BLDY: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x0400_0054) };

/// [SOUND1CNT_L](https://problemkaputt.de/gbatek.htm#gbasoundchannel1tonesweep)
pub const TONE1_SWEEP: VolAddress<ToneSweep, Safe, Safe> = unsafe { VolAddress::new(0x0400_0060) };
/// [SOUND1CNT_H](https://problemkaputt.de/gbatek.htm#gbasoundchannel1tonesweep)
pub const TONE1_DUTY_LEN_ENV: VolAddress<ToneDutyLenEnv, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0062) };
/// [SOUND1CNT_X](https://problemkaputt.de/gbatek.htm#gbasoundchannel1tonesweep)
pub const TONE1_FREQ_CNT: VolAddress<ToneFrequencyControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0064) };

/// [SOUND2CNT_L](https://problemkaputt.de/gbatek.htm#gbasoundchannel2tone)
pub const TONE2_DUTY_LEN_ENV: VolAddress<ToneDutyLenEnv, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0068) };
/// [SOUND2CNT_H](https://problemkaputt.de/gbatek.htm#gbasoundchannel2tone)
pub const TONE2_FREQ_CNT: VolAddress<ToneFrequencyControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_006C) };

/// [SOUND3CNT_L](https://problemkaputt.de/gbatek.htm#gbasoundchannel3waveoutput)
pub const WAVE_CONTROL: VolAddress<WaveControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0070) };
/// [SOUND3CNT_H](https://problemkaputt.de/gbatek.htm#gbasoundchannel3waveoutput)
pub const WAVE_LEN_VOLUME: VolAddress<WaveLenVolume, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0072) };
/// [SOUND3CNT_X](https://problemkaputt.de/gbatek.htm#gbasoundchannel3waveoutput)
pub const WAVE_FREQ_CNT: VolAddress<WaveFrequencyControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0074) };
/// [WAVE_RAM](https://problemkaputt.de/gbatek.htm#gbasoundchannel3waveoutput)
pub const WAVE_RAM: VolBlock<u32, Safe, Safe, 4> = unsafe { VolBlock::new(0x0400_0090) };

/// [SOUND4CNT_L](https://problemkaputt.de/gbatek.htm#gbasoundchannel4noise)
pub const NOISE_LEN_ENV: VolAddress<NoiseLenEnv, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0078) };
/// [SOUND4CNT_H](https://problemkaputt.de/gbatek.htm#gbasoundchannel4noise)
pub const NOISE_FREQ_CNT: VolAddress<NoiseFrequencyControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_007C) };

/// [SOUNDCNT_L](https://problemkaputt.de/gbatek.htm#gbasoundcontrolregisters)
pub const SOUND_CONTROL: VolAddress<SoundControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0080) };
/// [SOUNDCNT_X](https://problemkaputt.de/gbatek.htm#gbasoundcontrolregisters)
pub const SOUND_STATUS: VolAddress<SoundStatus, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0084) };
/// [SOUNDBIAS](https://problemkaputt.de/gbatek.htm#gbasoundcontrolregisters)
pub const SOUND_BIAS: VolAddress<SoundBias, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0088) };

/// [SOUNDCNT_H](https://problemkaputt.de/gbatek.htm#gbasoundcontrolregisters) (R/W fields)
pub const FIFO_CONTROL: VolAddress<FifoControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0082) };
/// [SOUNDCNT_H](https://problemkaputt.de/gbatek.htm#gbasoundcontrolregisters) (write-only fields)
pub const FIFO_RESET: VolAddress<FifoReset, (), Safe> =
  unsafe { VolAddress::new(0x0400_0082) };
/// [FIFO_A](https://problemkaputt.de/gbatek.htm#gbasoundchannelaandbdmasound)
pub const FIFO_A: VolAddress<u32, (), Safe> = unsafe { VolAddress::new(0x0400_00A0) };
/// [FIFO_B](https://problemkaputt.de/gbatek.htm#gbasoundchannelaandbdmasound)
pub const FIFO_B: VolAddress<u32, (), Safe> = unsafe { VolAddress::new(0x0400_00A4) };

/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 0 Source Address (W) (internal memory)
pub const DMA0SAD: VolAddress<usize, (), Unsafe> = unsafe { VolAddress::new(0x0400_00B0) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 0 Destination Address (W) (internal memory)
pub const DMA0DAD: VolAddress<usize, (), Unsafe> = unsafe { VolAddress::new(0x0400_00B4) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 0 Word Count (W) (14 bit, 1..4000h)
pub const DMA0CNT_L: VolAddress<u16, (), Unsafe> = unsafe { VolAddress::new(0x0400_00B8) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 0 Control (R/W)
pub const DMA0CNT_H: VolAddress<DmaControl, Safe, Unsafe> = unsafe { VolAddress::new(0x0400_00BA) };

/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 1 Source Address (W) (any memory)
pub const DMA1SAD: VolAddress<usize, (), Unsafe> = unsafe { VolAddress::new(0x0400_00BC) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 1 Destination Address (W) (internal memory)
pub const DMA1DAD: VolAddress<usize, (), Unsafe> = unsafe { VolAddress::new(0x0400_00C0) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 1 Word Count (W) (14 bit, 1..4000h)
pub const DMA1CNT_L: VolAddress<u16, (), Unsafe> = unsafe { VolAddress::new(0x0400_00C4) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 1 Control (R/W)
pub const DMA1CNT_H: VolAddress<DmaControl, Safe, Unsafe> = unsafe { VolAddress::new(0x0400_00C6) };

/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 2 Source Address (W) (any memory)
pub const DMA2SAD: VolAddress<usize, (), Unsafe> = unsafe { VolAddress::new(0x0400_00C8) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 2 Destination Address (W) (internal memory)
pub const DMA2DAD: VolAddress<usize, (), Unsafe> = unsafe { VolAddress::new(0x0400_00CC) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 2 Word Count (W) (14 bit, 1..4000h)
pub const DMA2CNT_L: VolAddress<u16, (), Unsafe> = unsafe { VolAddress::new(0x0400_00D0) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 2 Control (R/W)
pub const DMA2CNT_H: VolAddress<DmaControl, Safe, Unsafe> = unsafe { VolAddress::new(0x0400_00D2) };

/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 3 Source Address (W) (any memory)
pub const DMA3SAD: VolAddress<usize, (), Unsafe> = unsafe { VolAddress::new(0x0400_00D4) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 3 Destination Address (W) (any memory)
pub const DMA3DAD: VolAddress<usize, (), Unsafe> = unsafe { VolAddress::new(0x0400_00D8) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 3 Word Count (W) (16 bit, 1..10000h)
pub const DMA3CNT_L: VolAddress<u16, (), Unsafe> = unsafe { VolAddress::new(0x0400_00DC) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 3 Control (R/W)
pub const DMA3CNT_H: VolAddress<DmaControl, Safe, Unsafe> = unsafe { VolAddress::new(0x0400_00DE) };

/// [TM0CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER0_COUNTER: VolAddress<u16, Safe, ()> = unsafe { VolAddress::new(0x0400_0100) };
/// [TM1CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER1_COUNTER: VolAddress<u16, Safe, ()> = unsafe { VolAddress::new(0x0400_0104) };
/// [TM2CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER2_COUNTER: VolAddress<u16, Safe, ()> = unsafe { VolAddress::new(0x0400_0108) };
/// [TM3CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER3_COUNTER: VolAddress<u16, Safe, ()> = unsafe { VolAddress::new(0x0400_010C) };

/// [TM0CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER0_RELOAD: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_0100) };
/// [TM1CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER1_RELOAD: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_0104) };
/// [TM2CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER2_RELOAD: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_0108) };
/// [TM3CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER3_RELOAD: VolAddress<u16, (), Safe> = unsafe { VolAddress::new(0x0400_010C) };

/// [TM0CNT_H](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER0_CONTROL: VolAddress<TimerControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0102) };
/// [TM1CNT_H](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER1_CONTROL: VolAddress<TimerControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0106) };
/// [TM2CNT_H](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER2_CONTROL: VolAddress<TimerControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_010A) };
/// [TM3CNT_H](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER3_CONTROL: VolAddress<TimerControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_010E) };

/// [SIOCNT](https://problemkaputt.de/gbatek.htm#gbacommunicationports)
pub const SIOCNT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_0128) };
/// [SIODATA8](https://problemkaputt.de/gbatek.htm#gbacommunicationports)
pub const SIODATA8: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x400_012A) };
/// [RCNT](https://problemkaputt.de/gbatek.htm#gbacommunicationports)
pub const RCNT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x0400_0134) };

/// [KEYINPUT](https://problemkaputt.de/gbatek.htm#gbakeypadinput)
pub const KEYINPUT: VolAddress<KeysLowActive, Safe, ()> = unsafe { VolAddress::new(0x0400_0130) };
/// [KEYCNT](https://problemkaputt.de/gbatek.htm#gbakeypadinput)
pub const KEYCNT: VolAddress<KeyInterruptControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0130) };

/// Points to the (A32) user interrupt handler function.
pub const USER_IRQ_HANDLER: VolAddress<Option<unsafe extern "C" fn()>, Safe, Unsafe> =
  unsafe { VolAddress::new(0x0300_7FFC) };
/// "Interrupt Master Enable", [IME](https://problemkaputt.de/gbatek.htm#gbainterruptcontrol)
pub const IME: VolAddress<bool, Safe, Unsafe> = unsafe { VolAddress::new(0x0400_0208) };
/// "Interrupts Enabled", [IE](https://problemkaputt.de/gbatek.htm#gbainterruptcontrol)
pub const IE: VolAddress<InterruptFlags, Safe, Unsafe> = unsafe { VolAddress::new(0x0400_0200) };
/// Shows which interrupts are pending.
///
/// [IF](https://problemkaputt.de/gbatek.htm#gbainterruptcontrol) (reading)
pub const IRQ_PENDING: VolAddress<InterruptFlags, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0202) };
/// Acknowledges an interrupt as having been handled.
///
/// [IF](https://problemkaputt.de/gbatek.htm#gbainterruptcontrol) (writing)
pub const IRQ_ACKNOWLEDGE: VolAddress<InterruptFlags, (), Safe> =
  unsafe { VolAddress::new(0x0400_0202) };
/// Use this during [`IntrWait`] and [`VBlankIntrWait`] interrupt handling.
///
/// You should:
/// * read the current value
/// * set any additional interrupt bits that you wish to mark as handled (do not
///   clear any currently set bits!)
/// * write the new value back to this register
///
/// ```no_run
/// # use crate::prelude::*;
/// // to acknowledge a vblank interrupt
/// let current = INTR_WAIT_ACKNOWLEDGE.read();
/// unsafe { INTR_WAIT_ACKNOWLEDGE.write(current.with_vblank(true)) };
/// ```
///
/// [GBATEK: IntrWait](https://problemkaputt.de/gbatek.htm#bioshaltfunctions)
pub const INTR_WAIT_ACKNOWLEDGE: VolAddress<InterruptFlags, Safe, Unsafe> = unsafe {
  // Note(Lokathor): This uses a mirrored location that's closer to the main IO
  // Control memory region so that LLVM has a better chance of being able to
  // just do an offset read/write from an address that's already in a register.
  // The "base" address of this location is 0x0300_7FF8, so some documents may
  // refer to that value instead.
  VolAddress::new(0x0300_FFF8)
};

pub const BACKDROP_COLOR: VolAddress<Color, Safe, Safe> = unsafe { VolAddress::new(0x0500_0000) };

pub const BG_PALETTE: VolBlock<Color, Safe, Safe, 256> = unsafe { VolBlock::new(0x0500_0000) };

pub const OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256> = unsafe { VolBlock::new(0x0500_0200) };

pub const OAM_ATTR0: VolSeries<ObjAttr0, Safe, Safe, 128, 8> =
  unsafe { VolSeries::new(0x0700_0000) };
pub const OAM_ATTR1: VolSeries<ObjAttr1, Safe, Safe, 128, 8> =
  unsafe { VolSeries::new(0x0700_0002) };
pub const OAM_ATTR2: VolSeries<ObjAttr2, Safe, Safe, 128, 8> =
  unsafe { VolSeries::new(0x0700_0004) };

pub const OAM_PA: VolSeries<i16, Safe, Safe, 32, 0x20> = unsafe { VolSeries::new(0x0700_0006) };
pub const OAM_PB: VolSeries<i16, Safe, Safe, 32, 0x20> = unsafe { VolSeries::new(0x0700_000E) };
pub const OAM_PC: VolSeries<i16, Safe, Safe, 32, 0x20> = unsafe { VolSeries::new(0x0700_0016) };
pub const OAM_PD: VolSeries<i16, Safe, Safe, 32, 0x20> = unsafe { VolSeries::new(0x0700_001E) };
