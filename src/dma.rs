use crate::macros::{const_new, u16_bool_field, u16_enum_field};
use voladdress::{Safe, Unsafe, VolAddress};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DmaControl(u16);
impl DmaControl {
  const_new!();
  u16_enum_field!(5 - 6: DestAddrControl, dest_addr, with_dest_addr);
  u16_enum_field!(7 - 8: SrcAddrControl, src_addr, with_src_addr);
  u16_bool_field!(9, dma_repeat, with_dma_repeat);
  u16_bool_field!(10, transfer_u32, with_transfer_u32);
  u16_bool_field!(11, drq_from_game_pak, with_drq_from_game_pak);
  u16_enum_field!(12 - 13: DmaStartTiming, start_time, with_start_time);
  u16_bool_field!(14, irq_when_done, with_irq_when_done);
  u16_bool_field!(15, enabled, with_enabled);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DestAddrControl {
  Increment = 0 << 5,
  Decrement = 1 << 5,
  Fixed = 2 << 5,
  IncrementReload = 3 << 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SrcAddrControl {
  Increment = 0 << 7,
  Decrement = 1 << 7,
  Fixed = 2 << 7,
  /// Never use this value, it is only to guard against UB bit patterns
  Prohibited = 3 << 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DmaStartTiming {
  Immediately = 0 << 12,
  VBlank = 1 << 12,
  HBlank = 2 << 12,
  Special = 3 << 12,
}

/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 0 Source Address (W) (internal memory)
pub const DMA0SAD: VolAddress<usize, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00B0) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 0 Destination Address (W) (internal memory)
pub const DMA0DAD: VolAddress<usize, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00B4) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 0 Word Count (W) (14 bit, 1..4000h)
pub const DMA0CNT_L: VolAddress<u16, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00B8) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 0 Control (R/W)
pub const DMA0CNT_H: VolAddress<DmaControl, Safe, Unsafe> =
  unsafe { VolAddress::new(0x0400_00BA) };

/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 1 Source Address (W) (any memory)
pub const DMA1SAD: VolAddress<usize, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00BC) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 1 Destination Address (W) (internal memory)
pub const DMA1DAD: VolAddress<usize, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00C0) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 1 Word Count (W) (14 bit, 1..4000h)
pub const DMA1CNT_L: VolAddress<u16, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00C4) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 1 Control (R/W)
pub const DMA1CNT_H: VolAddress<DmaControl, Safe, Unsafe> =
  unsafe { VolAddress::new(0x0400_00C6) };

/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 2 Source Address (W) (any memory)
pub const DMA2SAD: VolAddress<usize, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00C8) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 2 Destination Address (W) (internal memory)
pub const DMA2DAD: VolAddress<usize, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00CC) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 2 Word Count (W) (14 bit, 1..4000h)
pub const DMA2CNT_L: VolAddress<u16, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00D0) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 2 Control (R/W)
pub const DMA2CNT_H: VolAddress<DmaControl, Safe, Unsafe> =
  unsafe { VolAddress::new(0x0400_00D2) };

/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 3 Source Address (W) (any memory)
pub const DMA3SAD: VolAddress<usize, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00D4) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 3 Destination Address (W) (any memory)
pub const DMA3DAD: VolAddress<usize, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00D8) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 3 Word Count (W) (16 bit, 1..10000h)
pub const DMA3CNT_L: VolAddress<u16, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00DC) };
/// [DMA](https://problemkaputt.de/gbatek.htm#gbadmatransfers) 3 Control (R/W)
pub const DMA3CNT_H: VolAddress<DmaControl, Safe, Unsafe> =
  unsafe { VolAddress::new(0x0400_00DE) };
