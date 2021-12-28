use crate::prelude::Color;

use core::arch::asm;
use voladdress::*;

pub const WIDTH: usize = 240;

pub const HEIGHT: usize = 160;

pub const BITMAP: VolBlock<Color, Safe, Safe, { WIDTH * HEIGHT }> =
  unsafe { VolBlock::new(0x0600_0000) };

pub const fn bitmap_xy(x: usize, y: usize) -> VolAddress<Color, Safe, Safe> {
  BITMAP.index(y * WIDTH + x)
}

pub fn dma3_clear_to(color: Color) {
  use crate::prelude::{
    DestAddrControl, DmaControl, DmaStartTiming, SrcAddrControl, DMA3CNT_H, DMA3CNT_L, DMA3DAD,
    DMA3SAD,
  };
  let wide_color: u32 = color.0 as u32 | ((color.0 as u32) << 16);
  unsafe {
    DMA3SAD.write(&wide_color as *const _ as usize);
    DMA3DAD.write(0x0600_0000);
    const MODE3_WORD_COUNT: u16 = (WIDTH * HEIGHT * 2 / 4_usize) as u16;
    DMA3CNT_L.write(MODE3_WORD_COUNT);
    const CTRL: DmaControl = DmaControl::new()
      .with_dest_addr(DestAddrControl::Increment)
      .with_src_addr(SrcAddrControl::Fixed)
      .with_transfer_u32(true)
      .with_start_time(DmaStartTiming::Immediately)
      .with_enabled(true);
    DMA3CNT_H.write(CTRL);
    asm!(
      "
      nop
      nop
      ",
      options(nostack),
    );
  }
}
