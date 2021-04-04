use crate::mmio_types::*;

use voladdress::*;

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

// Question: should we combine these or is it fine to have a bunch of u8?

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

// todo: mode 0
// todo: mode 1
// todo: mode 2
// todo: mode 4
// todo: mode 5

pub const MODE3_BITMAP: VolBlock<Color, Safe, Safe, { 240 * 160 }> =
  unsafe { VolBlock::new(0x0600_0000) };
