//! Module for interfacing with the GBA's Direct Memory Access units.
//!
//! The GBA has four DMA units, numbered from 0 to 3. They can be used for
//! extremely efficient memory transfers, and they can also be set to
//! automatically transfer in response to select events.
//!
//! Whenever a DMA unit is active, the CPU does not operate at all. Not even
//! hardware interrupts will occur while a DMA is running. The interrupt will
//! instead happen after the DMA transfer is done. When it's critical that an
//! interrupt be handled exactly on time (such as when using serial interrupts)
//! then you should avoid any large DMA transfers.
//!
//! In any situation when more than one DMA unit would be active at the same
//! time, the lower-numbered DMA unit runs first.
//!
//! Each DMA unit is controlled by 4 different MMIO addresses, as follows
//! (replace `x` with the DMA unit's number):
//! * `DMAx_SRC` and `DMAx_DEST`: source and destination address. DMA 0 can only
//!   use internal memory, DMA 1 and 2 can read from the gamepak but not write
//!   to it, and DMA 3 can even write to the gamepak (when the gamepak itself
//!   supports that). In all cases, SRAM cannot be accessed. The addresses of a
//!   transfer should always be aliged to the element size.
//! * `DMAx_COUNT`: Number of elements to transfer. The number of elements is
//!   either a 14-bit (DMA 0/1/2) or 16-bit (DMA3) number. If the count is set
//!   to 0 then the transfer will instead copy one more than the normal maximum
//!   of that number's range (DMA 0/1/2: 16_384, DMA 3: 65_536).
//! * `DMAx_CONTROL`: Configuration bits for the transfer, see [`DmaControl`].
//!
//! ## Safety
//!
//! The DMA units are the least safe part of the GBA and should be used with
//! caution.
//!
//! Because Rust doesn't have a fully precise memory model, and because LLVM is
//! a little fuzzy about the limits of what a volatile address access can do,
//! you are advised to **not** use DMA to alter any memory that is part of
//! Rust's compilation (stack variables, static variables, etc).
//!
//! You are advised to only use the DMA units to transfer data into VRAM,
//! PALRAM, OAM, and MMIO controls (eg: the FIFO sound buffers).
//!
//! In the future the situation may improve.

use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field};

/// Sets the change in destination address after each transfer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum DestAddrControl {
  /// Increases the address by one element (`addr = addr.add(1)`)
  #[default]
  Increment = 0 << 5,
  /// Decreases the address by one element (`addr = addr.sub(1)`)
  Decrement = 1 << 5,
  /// The address does not change.
  Fixed = 2 << 5,
  /// The address increases by one element per transfer, and also returns to
  /// its initial value when the DMA unit restarts.
  IncReload = 3 << 5,
}

/// Sets the change in source address after each transfer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SrcAddrControl {
  /// Increases the address by one element (`addr = addr.add(1)`)
  #[default]
  Increment = 0 << 7,
  /// Decreases the address by one element (`addr = addr.sub(1)`)
  Decrement = 1 << 7,
  /// The address does not change.
  Fixed = 2 << 7,
}

/// When the DMA unit should start doing work.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum DmaStartTime {
  /// The DMA unit should start immediately.
  ///
  /// When this is used, there is actually a 2 CPU cycle delay before the
  /// transfer begins.
  #[default]
  Immediate = 0 << 12,
  /// Transfer when vertical blank starts.
  VBlank = 1 << 12,
  /// Transfer when horizontal blank starts.
  HBlank = 2 << 12,
  /// Transfer at a special time according to which DMA unit you use this with:
  /// * 0: The `Special` start time is illegal to use with DMA0.
  /// * 1 or 2: When the associated sound FIFO buffer is empty
  /// * 3: Video capture.
  Special = 3 << 12,
}

/// DMA control configuration.
///
/// * `dest_addr_control`: How the destination address changes per element
///   transferred.
/// * `src_addr_control`: How the source address changes per element
///   transferred.
/// * `repeat`: If the DMA should automatically trigger again at the next start
///   time (vblank, hblank, or special). Caution: if you use `repeat` in
///   combination with the `Immediate` start time then the DMA will run over and
///   over and lock up the system.
/// * `transfer_32bit`: When set the DMA will transfer in 32-bit elements.
///   Otherwise, it will transfer in 16-bit elements. In general, you should
///   always transfer using 32-bit units when possible.
/// * `start_time`: When the DMA unit should begin a transfer.
/// * `irq_after`: If the end of the DMA transfer should send a hardware
///   interrupt.
/// * `enabled`: If the DMA unit is active.
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

  /// Unwrap this value into its raw `u16` form.
  #[inline]
  #[must_use]
  pub const fn to_u16(self) -> u16 {
    self.0
  }
}

/// Uses `stm` to set all parts of a DMA as a single instruction.
///
/// * `dma_id` is 0, 1, 2, or 3 (this is debug asserted).
/// * `src` address for the transfer
/// * `dest` address for the transfer
/// * `count_ctrl` is the count in the low half and control in the upper half
#[inline]
#[allow(dead_code)]
// we may make this pub in the future, until then this is basically a note
unsafe fn stm_dma(
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
