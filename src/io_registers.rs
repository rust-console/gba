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

use super::*;

/// LCD Control. Read/Write.
///
/// * [gbatek entry](http://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol)
pub const DISPCNT: VolatilePtr<u16> = VolatilePtr(0x4000000 as *mut u16);

/// Undocumented - Green Swap
pub const GREEN_SWAP: VolatilePtr<u16> = VolatilePtr(0x4000002 as *mut u16);

/// General LCD Status (STAT,LYC)
pub const DISPSTAT: VolatilePtr<u16> = VolatilePtr(0x4000004 as *mut u16);

/// Vertical Counter (LY)
pub const VCOUNT: VolatilePtr<u16> = VolatilePtr(0x4000006 as *mut u16);

/// BG0 Control
pub const BG0CNT: VolatilePtr<u16> = VolatilePtr(0x4000008 as *mut u16);

/// BG1 Control
pub const BG1CNT: VolatilePtr<u16> = VolatilePtr(0x400000A as *mut u16);

/// BG2 Control
pub const BG2CNT: VolatilePtr<u16> = VolatilePtr(0x400000C as *mut u16);

/// BG3 Control
pub const BG3CNT: VolatilePtr<u16> = VolatilePtr(0x400000E as *mut u16);

/// BG0 X-Offset
pub const BG0HOFS: VolatilePtr<u16> = VolatilePtr(0x4000010 as *mut u16);

/// BG0 Y-Offset
pub const BG0VOFS: VolatilePtr<u16> = VolatilePtr(0x4000012 as *mut u16);

/// BG1 X-Offset
pub const BG1HOFS: VolatilePtr<u16> = VolatilePtr(0x4000014 as *mut u16);

/// BG1 Y-Offset
pub const BG1VOFS: VolatilePtr<u16> = VolatilePtr(0x4000016 as *mut u16);

/// BG2 X-Offset
pub const BG2HOFS: VolatilePtr<u16> = VolatilePtr(0x4000018 as *mut u16);

/// BG2 Y-Offset
pub const BG2VOFS: VolatilePtr<u16> = VolatilePtr(0x400001A as *mut u16);

/// BG3 X-Offset
pub const BG3HOFS: VolatilePtr<u16> = VolatilePtr(0x400001C as *mut u16);

/// BG3 Y-Offset
pub const BG3VOFS: VolatilePtr<u16> = VolatilePtr(0x400001E as *mut u16);

/// BG2 Rotation/Scaling Parameter A (dx)
pub const BG2PA: VolatilePtr<u16> = VolatilePtr(0x4000020 as *mut u16);

/// BG2 Rotation/Scaling Parameter B (dmx)
pub const BG2PB: VolatilePtr<u16> = VolatilePtr(0x4000022 as *mut u16);

/// BG2 Rotation/Scaling Parameter C (dy)
pub const BG2PC: VolatilePtr<u16> = VolatilePtr(0x4000024 as *mut u16);

/// BG2 Rotation/Scaling Parameter D (dmy)
pub const BG2PD: VolatilePtr<u16> = VolatilePtr(0x4000026 as *mut u16);

/// BG2 Reference Point X-Coordinate
pub const BG2X: VolatilePtr<u32> = VolatilePtr(0x4000028 as *mut u32);

/// BG2 Reference Point Y-Coordinate
pub const BG2Y: VolatilePtr<u32> = VolatilePtr(0x400002C as *mut u32);

/// BG3 Rotation/Scaling Parameter A (dx)
pub const BG3PA: VolatilePtr<u16> = VolatilePtr(0x4000030 as *mut u16);

/// BG3 Rotation/Scaling Parameter B (dmx)
pub const BG3PB: VolatilePtr<u16> = VolatilePtr(0x4000032 as *mut u16);

/// BG3 Rotation/Scaling Parameter C (dy)
pub const BG3PC: VolatilePtr<u16> = VolatilePtr(0x4000034 as *mut u16);

/// BG3 Rotation/Scaling Parameter D (dmy)
pub const BG3PD: VolatilePtr<u16> = VolatilePtr(0x4000036 as *mut u16);

/// BG3 Reference Point X-Coordinate
pub const BG3X: VolatilePtr<u32> = VolatilePtr(0x4000038 as *mut u32);

/// BG3 Reference Point Y-Coordinate
pub const BG3Y: VolatilePtr<u32> = VolatilePtr(0x400003C as *mut u32);

/// Window 0 Horizontal Dimensions
pub const WIN0H: VolatilePtr<u16> = VolatilePtr(0x4000040 as *mut u16);

/// Window 1 Horizontal Dimensions
pub const WIN1H: VolatilePtr<u16> = VolatilePtr(0x4000042 as *mut u16);

/// Window 0 Vertical Dimensions
pub const WIN0V: VolatilePtr<u16> = VolatilePtr(0x4000044 as *mut u16);

/// Window 1 Vertical Dimensions
pub const WIN1V: VolatilePtr<u16> = VolatilePtr(0x4000046 as *mut u16);

/// Inside of Window 0 and 1
pub const WININ: VolatilePtr<u16> = VolatilePtr(0x4000048 as *mut u16);

/// Inside of OBJ Window & Outside of Windows
pub const WINOUT: VolatilePtr<u16> = VolatilePtr(0x400004A as *mut u16);

/// Mosaic Size
pub const MOSAIC: VolatilePtr<u16> = VolatilePtr(0x400004C as *mut u16);

/// Color Special Effects Selection
pub const BLDCNT: VolatilePtr<u16> = VolatilePtr(0x4000050 as *mut u16);

/// Alpha Blending Coefficients
pub const BLDALPHA: VolatilePtr<u16> = VolatilePtr(0x4000052 as *mut u16);

/// Brightness (Fade-In/Out) Coefficient
pub const BLDY: VolatilePtr<u16> = VolatilePtr(0x4000054 as *mut u16);

/// Channel 1 Sweep register       (NR10)
pub const UND1CNT_L: VolatilePtr<u16> = VolatilePtr(0x4000060 as *mut u16);

/// Channel 1 Duty/Length/Envelope (NR11, NR12)
pub const UND1CNT_H: VolatilePtr<u16> = VolatilePtr(0x4000062 as *mut u16);

/// Channel 1 Frequency/Control    (NR13, NR14)
pub const UND1CNT_X: VolatilePtr<u16> = VolatilePtr(0x4000064 as *mut u16);

/// Channel 2 Duty/Length/Envelope (NR21, NR22)
pub const UND2CNT_L: VolatilePtr<u16> = VolatilePtr(0x4000068 as *mut u16);

/// Channel 2 Frequency/Control    (NR23, NR24)
pub const UND2CNT_H: VolatilePtr<u16> = VolatilePtr(0x400006C as *mut u16);

/// Channel 3 Stop/Wave RAM select (NR30)
pub const UND3CNT_L: VolatilePtr<u16> = VolatilePtr(0x4000070 as *mut u16);

/// Channel 3 Length/Volume        (NR31, NR32)
pub const UND3CNT_H: VolatilePtr<u16> = VolatilePtr(0x4000072 as *mut u16);

/// Channel 3 Frequency/Control    (NR33, NR34)
pub const UND3CNT_X: VolatilePtr<u16> = VolatilePtr(0x4000074 as *mut u16);

/// Channel 4 Length/Envelope      (NR41, NR42)
pub const UND4CNT_L: VolatilePtr<u16> = VolatilePtr(0x4000078 as *mut u16);

/// Channel 4 Frequency/Control    (NR43, NR44)
pub const UND4CNT_H: VolatilePtr<u16> = VolatilePtr(0x400007C as *mut u16);

/// Control Stereo/Volume/Enable   (NR50, NR51)
pub const UNDCNT_L: VolatilePtr<u16> = VolatilePtr(0x4000080 as *mut u16);

/// Control Mixing/DMA Control
pub const UNDCNT_H: VolatilePtr<u16> = VolatilePtr(0x4000082 as *mut u16);

/// Control Sound on/off           (NR52)
pub const UNDCNT_X: VolatilePtr<u16> = VolatilePtr(0x4000084 as *mut u16);

/// Sound PWM Control
pub const UNDBIAS: VolatilePtr<u16> = VolatilePtr(0x4000088 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM0_L: VolatilePtr<u16> = VolatilePtr(0x4000090 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM0_H: VolatilePtr<u16> = VolatilePtr(0x4000092 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM1_L: VolatilePtr<u16> = VolatilePtr(0x4000094 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM1_H: VolatilePtr<u16> = VolatilePtr(0x4000096 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM2_L: VolatilePtr<u16> = VolatilePtr(0x4000098 as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM2_H: VolatilePtr<u16> = VolatilePtr(0x400009A as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM3_L: VolatilePtr<u16> = VolatilePtr(0x400009C as *mut u16);

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM3_H: VolatilePtr<u16> = VolatilePtr(0x400009E as *mut u16);

/// Channel A FIFO, Data 0-3
pub const FIFO_A: VolatilePtr<u32> = VolatilePtr(0x40000A0 as *mut u32);

/// Channel B FIFO, Data 0-3
pub const FIFO_B: VolatilePtr<u32> = VolatilePtr(0x40000A4 as *mut u32);

/// DMA 0 Source Address
pub const DMA0SAD: VolatilePtr<u32> = VolatilePtr(0x40000B0 as *mut u32);

/// DMA 0 Destination Address
pub const DMA0DAD: VolatilePtr<u32> = VolatilePtr(0x40000B4 as *mut u32);

/// DMA 0 Word Count
pub const DMA0CNT_L: VolatilePtr<u16> = VolatilePtr(0x40000B8 as *mut u16);

/// DMA 0 Control
pub const DMA0CNT_H: VolatilePtr<u16> = VolatilePtr(0x40000BA as *mut u16);

/// DMA 1 Source Address
pub const DMA1SAD: VolatilePtr<u32> = VolatilePtr(0x40000BC as *mut u32);

/// DMA 1 Destination Address
pub const DMA1DAD: VolatilePtr<u32> = VolatilePtr(0x40000C0 as *mut u32);

/// DMA 1 Word Count
pub const DMA1CNT_L: VolatilePtr<u16> = VolatilePtr(0x40000C4 as *mut u16);

/// DMA 1 Control
pub const DMA1CNT_H: VolatilePtr<u16> = VolatilePtr(0x40000C6 as *mut u16);

/// DMA 2 Source Address
pub const DMA2SAD: VolatilePtr<u32> = VolatilePtr(0x40000C8 as *mut u32);

/// DMA 2 Destination Address
pub const DMA2DAD: VolatilePtr<u32> = VolatilePtr(0x40000CC as *mut u32);

/// DMA 2 Word Count
pub const DMA2CNT_L: VolatilePtr<u16> = VolatilePtr(0x40000D0 as *mut u16);

/// DMA 2 Control
pub const DMA2CNT_H: VolatilePtr<u16> = VolatilePtr(0x40000D2 as *mut u16);

/// DMA 3 Source Address
pub const DMA3SAD: VolatilePtr<u32> = VolatilePtr(0x40000D4 as *mut u32);

/// DMA 3 Destination Address
pub const DMA3DAD: VolatilePtr<u32> = VolatilePtr(0x40000D8 as *mut u32);

/// DMA 3 Word Count
pub const DMA3CNT_L: VolatilePtr<u16> = VolatilePtr(0x40000DC as *mut u16);

/// DMA 3 Control
pub const DMA3CNT_H: VolatilePtr<u16> = VolatilePtr(0x40000DE as *mut u16);

/// Timer 0 Counter/Reload
pub const TM0CNT_L: VolatilePtr<u16> = VolatilePtr(0x4000100 as *mut u16);

/// Timer 0 Control
pub const TM0CNT_H: VolatilePtr<u16> = VolatilePtr(0x4000102 as *mut u16);

/// Timer 1 Counter/Reload
pub const TM1CNT_L: VolatilePtr<u16> = VolatilePtr(0x4000104 as *mut u16);

/// Timer 1 Control
pub const TM1CNT_H: VolatilePtr<u16> = VolatilePtr(0x4000106 as *mut u16);

/// Timer 2 Counter/Reload
pub const TM2CNT_L: VolatilePtr<u16> = VolatilePtr(0x4000108 as *mut u16);

/// Timer 2 Control
pub const TM2CNT_H: VolatilePtr<u16> = VolatilePtr(0x400010A as *mut u16);

/// Timer 3 Counter/Reload
pub const TM3CNT_L: VolatilePtr<u16> = VolatilePtr(0x400010C as *mut u16);

/// Timer 3 Control
pub const TM3CNT_H: VolatilePtr<u16> = VolatilePtr(0x400010E as *mut u16);

/// SIO Data (Normal-32bit Mode; shared with below)
pub const SIODATA32: VolatilePtr<u32> = VolatilePtr(0x4000120 as *mut u32);

/// SIO Data 0 (Parent)    (Multi-Player Mode)
pub const SIOMULTI0: VolatilePtr<u16> = VolatilePtr(0x4000120 as *mut u16);

/// SIO Data 1 (1st Child) (Multi-Player Mode)
pub const SIOMULTI1: VolatilePtr<u16> = VolatilePtr(0x4000122 as *mut u16);

/// SIO Data 2 (2nd Child) (Multi-Player Mode)
pub const SIOMULTI2: VolatilePtr<u16> = VolatilePtr(0x4000124 as *mut u16);

/// SIO Data 3 (3rd Child) (Multi-Player Mode)
pub const SIOMULTI3: VolatilePtr<u16> = VolatilePtr(0x4000126 as *mut u16);

/// SIO Control Register
pub const SIOCNT: VolatilePtr<u16> = VolatilePtr(0x4000128 as *mut u16);

/// D SIO Data (Local of MultiPlayer; shared below)
pub const SIOMLT_SEN: VolatilePtr<u16> = VolatilePtr(0x400012A as *mut u16);

/// SIO Data (Normal-8bit and UART Mode)
pub const SIODATA8: VolatilePtr<u16> = VolatilePtr(0x400012A as *mut u16);

/// Key Status
pub const KEYINPUT: VolatilePtr<u16> = VolatilePtr(0x4000130 as *mut u16);

/// Key Interrupt Control
pub const KEYCNT: VolatilePtr<u16> = VolatilePtr(0x4000132 as *mut u16);

/// SIO Mode Select/General Purpose Data
pub const RCNT: VolatilePtr<u16> = VolatilePtr(0x4000134 as *mut u16);

/// SIO JOY Bus Control
pub const JOYCNT: VolatilePtr<u16> = VolatilePtr(0x4000140 as *mut u16);

/// SIO JOY Bus Receive Data
pub const JOY_RECV: VolatilePtr<u32> = VolatilePtr(0x4000150 as *mut u32);

/// SIO JOY Bus Transmit Data
pub const JOY_TRANS: VolatilePtr<u32> = VolatilePtr(0x4000154 as *mut u32);

/// SIO JOY Bus Receive Status
pub const JOYSTAT: VolatilePtr<u16> = VolatilePtr(0x4000158 as *mut u16);

/// Interrupt Enable Register
pub const IE: VolatilePtr<u16> = VolatilePtr(0x4000200 as *mut u16);

/// Interrupt Request Flags / IRQ Acknowledge
pub const IF: VolatilePtr<u16> = VolatilePtr(0x4000202 as *mut u16);

/// Game Pak Waitstate Control
pub const WAITCNT: VolatilePtr<u16> = VolatilePtr(0x4000204 as *mut u16);

/// Interrupt Master Enable Register
pub const IME: VolatilePtr<u16> = VolatilePtr(0x4000208 as *mut u16);

/// Undocumented - Post Boot Flag
pub const POSTFLG: VolatilePtr<u8> = VolatilePtr(0x4000300 as *mut u8);

/// Undocumented - Power Down Control
pub const HALTCNT: VolatilePtr<u8> = VolatilePtr(0x4000301 as *mut u8);
