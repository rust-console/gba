use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum DestAddrControl {
  #[default]
  Increment = 0 << 5,
  Decrement = 1 << 5,
  Fixed = 2 << 5,
  IncReload = 3 << 5,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SrcAddrControl {
  #[default]
  Increment = 0 << 7,
  Decrement = 1 << 7,
  Fixed = 2 << 7,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum DmaStartTime {
  #[default]
  Immediate = 0 << 12,
  VBlank = 1 << 12,
  HBlank = 2 << 12,
  Special = 3 << 12,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DmaControl(u16);
impl DmaControl {
  pub_const_fn_new_zeroed!();
  u16_enum_field!(
    5 - 6: DestAddrControl,
    dest_addr_control,
    with_dest_addr_control
  );
  u16_enum_field!(
    7 - 8: SrcAddrControl,
    src_addr_control,
    with_src_addr_control
  );
  u16_bool_field!(9, repeat, with_repeat);
  u16_bool_field!(10, transfer_32bit, with_transfer_32bit);
  u16_enum_field!(12 - 13: DmaStartTime, start_time, with_start_time);
  u16_bool_field!(14, irq_after, with_irq_after);
  u16_bool_field!(15, enabled, with_enabled);

  #[inline]
  #[must_use]
  pub const fn to_u16(self) -> u16 {
    self.0
  }
}

/// Uses `stm` to set all parts of a DMA as a single instruction.
///
/// * `src` address for the transfer
/// * `dest` address for the transfer
/// * `count_ctrl` is the count in the low half and control in the upper half
/// * `dma_id` is 0, 1, 2, or 3 (this is debug asserted).
///
/// After setting the DMA, it won't activate for a minimum of 2 CPU cycles.
#[inline]
pub unsafe fn stm_dma(
  dma_id: usize, src: *const u8, dest: *mut u8, count_ctrl: u32,
) {
  debug_assert!(dma_id < 4);
  let dma_addr = 0x0400_00B0 + dma_id * 0xC;
  core::arch::asm!(
    "stm r0, {{r1, r2, r3}}",
    in("r0") dma_addr,
    in("r1") src,
    in("r2") dest,
    in("r3") count_ctrl,
    options(nostack, preserves_flags)
  );
}
