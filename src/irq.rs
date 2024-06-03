//! Hardware interrupt handling

use super::*;
use crate::gba_cell::GbaCell;

/// The user-provided interrupt request handler function.
#[cfg(feature = "on_gba")]
pub static USER_IRQ_HANDLER: GbaCell<
  Option<unsafe extern "C" fn(crate::irq::IrqBits)>,
> = GbaCell::new(None);

/// Interrupt bit flags.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct IrqBits(pub u16);
impl IrqBits {
  /// The vblank bit.
  pub const VBLANK: Self = Self::new().with_vblank(true);

  /// Makes a new, empty value.
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  /// Vertical-blank
  #[inline]
  #[must_use]
  pub const fn vblank(self) -> bool {
    u16_get_bit(0, self.0)
  }
  /// Horizontal-blank
  #[inline]
  #[must_use]
  pub const fn hblank(self) -> bool {
    u16_get_bit(1, self.0)
  }
  /// Vertical-counter match
  #[inline]
  #[must_use]
  pub const fn vcount(self) -> bool {
    u16_get_bit(2, self.0)
  }
  /// Timer 0 overflow
  #[inline]
  #[must_use]
  pub const fn timer0(self) -> bool {
    u16_get_bit(3, self.0)
  }
  /// Timer 1 overflow
  #[inline]
  #[must_use]
  pub const fn timer1(self) -> bool {
    u16_get_bit(4, self.0)
  }
  /// Timer 2 overflow
  #[inline]
  #[must_use]
  pub const fn timer2(self) -> bool {
    u16_get_bit(5, self.0)
  }
  /// Timer 3 overflow
  #[inline]
  #[must_use]
  pub const fn timer3(self) -> bool {
    u16_get_bit(6, self.0)
  }
  /// Serial port communication
  #[inline]
  #[must_use]
  pub const fn serial(self) -> bool {
    u16_get_bit(7, self.0)
  }
  /// DMA 0 complete
  #[inline]
  #[must_use]
  pub const fn dma0(self) -> bool {
    u16_get_bit(8, self.0)
  }
  /// DMA 1 complete
  #[inline]
  #[must_use]
  pub const fn dma1(self) -> bool {
    u16_get_bit(9, self.0)
  }
  /// DMA 2 complete
  #[inline]
  #[must_use]
  pub const fn dma2(self) -> bool {
    u16_get_bit(10, self.0)
  }
  /// DMA 3 complete
  #[inline]
  #[must_use]
  pub const fn dma3(self) -> bool {
    u16_get_bit(11, self.0)
  }
  /// Keypad match
  #[inline]
  #[must_use]
  pub const fn keypad(self) -> bool {
    u16_get_bit(12, self.0)
  }
  /// Game pak
  #[inline]
  #[must_use]
  pub const fn gamepak(self) -> bool {
    u16_get_bit(13, self.0)
  }

  /// Set the vblank bit.
  #[inline]
  #[must_use]
  pub const fn with_vblank(self, vblank: bool) -> Self {
    Self(u16_with_bit(0, self.0, vblank))
  }
  /// Set the hblank bit.
  #[inline]
  #[must_use]
  pub const fn with_hblank(self, hblank: bool) -> Self {
    Self(u16_with_bit(1, self.0, hblank))
  }
  /// Set the vcount bit.
  #[inline]
  #[must_use]
  pub const fn with_vcount(self, vcount: bool) -> Self {
    Self(u16_with_bit(2, self.0, vcount))
  }
  /// Set the timer0 bit.
  #[inline]
  #[must_use]
  pub const fn with_timer0(self, timer0: bool) -> Self {
    Self(u16_with_bit(3, self.0, timer0))
  }
  /// Set the timer1 bit.
  #[inline]
  #[must_use]
  pub const fn with_timer1(self, timer1: bool) -> Self {
    Self(u16_with_bit(4, self.0, timer1))
  }
  /// Set the timer2 bit.
  #[inline]
  #[must_use]
  pub const fn with_timer2(self, timer2: bool) -> Self {
    Self(u16_with_bit(5, self.0, timer2))
  }
  /// Set the timer3 bit.
  #[inline]
  #[must_use]
  pub const fn with_timer3(self, timer3: bool) -> Self {
    Self(u16_with_bit(6, self.0, timer3))
  }
  /// Set the serial bit.
  #[inline]
  #[must_use]
  pub const fn with_serial(self, serial: bool) -> Self {
    Self(u16_with_bit(7, self.0, serial))
  }
  /// Set the dma0 bit.
  #[inline]
  #[must_use]
  pub const fn with_dma0(self, dma0: bool) -> Self {
    Self(u16_with_bit(8, self.0, dma0))
  }
  /// Set the dma1 bit.
  #[inline]
  #[must_use]
  pub const fn with_dma1(self, dma1: bool) -> Self {
    Self(u16_with_bit(9, self.0, dma1))
  }
  /// Set the dma2 bit.
  #[inline]
  #[must_use]
  pub const fn with_dma2(self, dma2: bool) -> Self {
    Self(u16_with_bit(10, self.0, dma2))
  }
  /// Set the dma3 bit.
  #[inline]
  #[must_use]
  pub const fn with_dma3(self, dma3: bool) -> Self {
    Self(u16_with_bit(11, self.0, dma3))
  }
  /// Set the keypad bit.
  #[inline]
  #[must_use]
  pub const fn with_keypad(self, keypad: bool) -> Self {
    Self(u16_with_bit(12, self.0, keypad))
  }
  /// Set the gamepak bit.
  #[inline]
  #[must_use]
  pub const fn with_gamepak(self, gamepak: bool) -> Self {
    Self(u16_with_bit(13, self.0, gamepak))
  }
}

/// Interrupts Enabled.
///
/// When any sub-system is set to "send" interrupts, that interrupt type must
/// *also* be configured here or it won't actually be "received" by the CPU.
pub const IE: PlainAddr<IrqBits> = unsafe { VolAddress::new(0x0400_0200) };

/// Interrupts Flagged.
///
/// These are the interrupts that are pending, and haven't been handled. Clear a
/// pending interrupt by writing an [`IrqBits`] value with that bit enabled. The
/// assembly runtime handles this automatically, so you don't normally need to
/// interact with `IF` at all.
pub const IF: PlainAddr<IrqBits> = unsafe { VolAddress::new(0x0400_0202) };

/// Interrupt Master Enable
///
/// * When this is set to `true`, hardware interrupts that are flagged will
///   immediately run the interrupt handler.
/// * When this is `false`, any interrupt events that are flagged will be left
///   pending until this is again set to `true`.
///
/// This defaults to `false`.
///
/// Technically there's a two CPU cycle delay between this being written and
/// interrupts actually being enabled/disabled. In practice, it doesn't matter.
pub const IME: PlainAddr<bool> = unsafe { VolAddress::new(0x0400_0208) };
