//! Module for the GBA's Direct Memory Access (DMA) units.
//!
//! ## Basics
//!
//! The GBA has 4 DMA units, numbered 0 through 3. They all work in a similar
//! way, but they each have slightly different limitations and intended use.
//!
//! When any DMA is active, the CPU itself is paused. It won't execute code, and
//! it won't even handle interrupts. Once the DMA is completed normal CPU
//! operations will continue, and only then will any pending interrupts be
//! handled. If you need all interrupts to be handled as quickly as possible
//! (such as serial port interrupts when it's active) then you should not use
//! DMA during that time.
//!
//! Similarly, if more than one DMA is set to be active at the same time, the
//! lower numbered DMA unit "wins" and will perform its operation first, then
//! the higher number DMA will run.
//!
//! The DMA units can transfer data using various configurations. The most
//! common uses of DMA are:
//!
//! 1) Copying large quantities of data, either from ROM into RAM or between two
//!    different regions of RAM. The DMA units are faster at copying data than
//!    even the most efficient CPU-based copy loops.
//! 2) Copying data at special moments. The DMA units can be set to run at
//!    either horizontal or vertical blank time, or when the sound FIFO buffer
//!    runs out.
//!
//! ## Unit Differences
//!
//! * DMA 0 can only transfer between memory on the GBA itself, it cannot access
//!   ROM or SRAM. It's usually used for smaller, very high priority transfers.
//! * DMA 1 and 2 can transfer from ROM memory. These units are intended to be
//!   used with the FIFO sound buffers, so that playback of a sound can happen
//!   smoothly regardless of the current position of the CPU within the program.
//! * DMA 3 can transfer from ROM, and also into ROM if the game cart supports
//!   that. This DMA unit is the one usually used for loading graphical data
//!   from ROM into VRAM.
//!
//! ## Safety
//!
//! Using the DMA units is equivalent to playing around with raw pointers. It
//! must be handled very carefully, or memory corruption can occur.

use bitfrob::{u16_with_bit, u16_with_region};
use voladdress::{Safe, VolRegion};

use crate::{
  mmio::{DMA3_CONTROL, DMA3_DESTINATION, DMA3_SOURCE, DMA3_TRANSFER_COUNT},
  video::Tile4bpp,
};

/// Controls the activity of a DMA unit.
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct DmaControl(u16);
impl DmaControl {
  /// Makes a new, empty value.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// Unwrap the raw bits.
  #[inline]
  pub const fn into_u16(self) -> u16 {
    self.0
  }
  /// Sets the DMA destination address control, see [`DmaDestAddr`]
  #[inline]
  pub const fn with_dest_addr(self, dest: DmaDestAddr) -> Self {
    Self(u16_with_region(5, 6, self.0, dest as u16))
  }
  /// Sets the DMA source address control, see [`DmaSrcAddr`].
  ///
  /// When transferring from ROM, this setting is **ignored** and the DMA unit
  /// will just act as if `Increment` is set.
  #[inline]
  pub const fn with_src_addr(self, src: DmaSrcAddr) -> Self {
    Self(u16_with_region(7, 8, self.0, src as u16))
  }
  /// If the DMA unit should repeat again at the next start timing.
  #[inline]
  pub const fn with_repeat(self, repeat: bool) -> Self {
    Self(u16_with_bit(9, self.0, repeat))
  }
  /// If the DMA unit should transfer `u32` data (otherwise it's `u16)
  #[inline]
  pub const fn with_u32_transfer(self, u32: bool) -> Self {
    Self(u16_with_bit(10, self.0, u32))
  }
  /// Sets the start timing of the DMA activity, see [`DmaStart`]
  #[inline]
  pub const fn with_start_time(self, start: DmaStart) -> Self {
    Self(u16_with_region(12, 13, self.0, start as u16))
  }
  /// If this DMA unit should send an IRQ after it completes the transfer.
  #[inline]
  pub const fn with_irq(self, irq: bool) -> Self {
    Self(u16_with_bit(14, self.0, irq))
  }
  /// If the DMA unit is enabled.
  #[inline]
  pub const fn with_enabled(self, enabled: bool) -> Self {
    Self(u16_with_bit(15, self.0, enabled))
  }
}

/// DMA Destination address settings.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u16)]
pub enum DmaDestAddr {
  /// After each transfer, the destination moves 1 element forward.
  #[default]
  Increment = 0 << 5,
  /// After each transfer, the destination moves 1 element backward.
  Decrement = 1 << 5,
  /// After each transfer, the destination does not change.
  Fixed = 2 << 5,
  /// After each transfer, the destination moves 1 element forward.
  ///
  /// **Also**, when beginning a repeated DMA cycle, the destination address
  /// reloads to the initial value that was set by the user.
  IncReload = 3 << 5,
}
/// DMA Source address settings.
///
/// When transferring from ROM, this setting is **ignored** and the DMA unit
/// will just act as if `Increment` is set.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u16)]
pub enum DmaSrcAddr {
  /// After each transfer, the source address moves 1 element forward.
  #[default]
  Increment = 0 << 7,
  /// After each transfer, the source address moves 1 element backward.
  Decrement = 1 << 7,
  /// After each transfer, the destination does not change.
  Fixed = 2 << 7,
}
/// When the DMA unit should begin transferring data.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u16)]
pub enum DmaStart {
  /// Makes the DMA unit run "right away".
  ///
  /// There's actually a 2 CPU cycle lag between enabling a DMA with
  /// `Immediate` timing and it actually activating.
  #[default]
  Immediate = 0 << 12,
  /// The DMA unit will start at Vertical Blank.
  ///
  /// The DMA will end up running *before* the IRQ handler.
  VBlank = 1 << 12,
  /// The DMA unit will start at Horizontal Blank.
  ///
  /// The DMA will end up running *before* the IRQ handler.
  HBlank = 2 << 12,
  /// The DMA will run with a special timing depending on what DMA unit it is.
  ///
  /// * This cannot be used with DMA 0.
  /// * For DMA 1 and 2 it runs when the FIFO buffer runs out.
  /// * For DMA 3 this is how you do Video Capture (which isn't currently
  ///   supported by this crate).
  Special = 3 << 12,
}

/// Copies `u32` data using DMA 3.
///
/// Works like the
/// [`copy_nonoverlapping`][core::intrinsics::copy_nonoverlapping] function, but
/// it's performed using DMA 3.
///
/// ## Safety
/// * The number of `u32` values to copy must be `<= 0x1_0000` (65,536). This is
///   trivially true under all normal conditions, since this is the size of
///   EWRAM, which is the largest non-ROM memory region.
/// * `src` must be aligned and readable for `count` elements.
/// * `dest` must be aligned and writable for `count` elements.
/// * The two regions must not overlap.
/// * `count` must not be 0.
#[inline(never)]
pub unsafe fn dma3_u32_copy(src: *const u32, dest: *mut u32, count: usize) {
  on_gba_or_unimplemented!(
    const CONTROL: DmaControl =
      DmaControl::new().with_u32_transfer(true).with_enabled(true);
    debug_assert!(count <= u16::MAX as usize);
    unsafe {
      DMA3_SOURCE.write(src.cast());
      DMA3_DESTINATION.write(dest.cast());
      DMA3_TRANSFER_COUNT.write(count as u16);
      DMA3_CONTROL.write(CONTROL);
      // Assumption: because this function is `inline(never)`, the time to
      // return to the caller is enough to ensure that the DMA controls aren't
      // used before the DMA's 2 cycle "immediate activation" delay is over.
    }
  );
}

/// Copies [`Tile4`] data using DMA3.
///
/// ## Panics
/// * The `src` and `dest` must have the same length.
#[inline]
pub fn dma3_copy_tile4(
  src: &[Tile4bpp], dest: VolRegion<Tile4bpp, Safe, Safe>,
) {
  assert_eq!(src.len(), dest.len());
  if src.len() == 0 {
    return;
  }
  // Safety: There's no writable region of memory that has more tiles that the
  // maximum transfer size of DMA3, so the length will never exceed our limit.
  // The requirements for the source to be readable and the destination to be
  // writable are satisfied by the data types of the input values.
  unsafe {
    dma3_u32_copy(src.as_ptr().cast(), dest.as_mut_ptr().cast(), src.len() * 8)
  };
}
