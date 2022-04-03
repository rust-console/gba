use voladdress::{Safe, VolAddress};

use crate::macros::{const_new, u16_bool_field};

mod gba_cell;
pub use gba_cell::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IrqBits(pub(crate) u16);
impl IrqBits {
  const_new!();
  u16_bool_field!(0, vblank, with_vblank);
  u16_bool_field!(1, hblank, with_hblank);
  u16_bool_field!(2, vcounter, with_vcounter);
  u16_bool_field!(3, timer0, with_timer0);
  u16_bool_field!(4, timer1, with_timer1);
  u16_bool_field!(5, timer2, with_timer2);
  u16_bool_field!(6, timer3, with_timer3);
  u16_bool_field!(7, serial, with_serial);
  u16_bool_field!(8, dma0, with_dma0);
  u16_bool_field!(9, dma1, with_dma1);
  u16_bool_field!(10, dma2, with_dma2);
  u16_bool_field!(11, dma3, with_dma3);
  u16_bool_field!(12, keypad, with_keypad);
  u16_bool_field!(13, game_pak, with_game_pak);

  pub const V_BLANK: Self = Self::new().with_vblank(true);
}

/// Interrupts Enabled
///
/// The interrupt handler will only be called for interrupts that are enabled.
pub const IE: VolAddress<IrqBits, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0200) };

/// Interrupt Master Enable
///
/// This lets interrupts happen (or not).
pub const IME: VolAddress<bool, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0208) };

/// A Rust function that can be run when an interrupt occurs.
///
/// * The argument is the flags of which interrupts should be handled.
///
/// The assembly runtime's handler will acknowledge all incoming interrupts
/// before calling your function.
pub type RustIrqFn = extern "C" fn(IrqBits);

/// This allows you to set a new Rust IRQ handler.
///
/// This rust function will be called by the assembly runtime on each interrupt.
/// If you set the rust handler to `None` the assembly runtime will skip the
/// call.
#[inline]
pub fn set_irq_handler(opt_fn: Option<RustIrqFn>) {
  // Safety: We declare this within the assembly runtime.
  extern "C" {
    static RUST_IRQ_HANDLER: GbaCell<Option<RustIrqFn>>;
  }
  //
  unsafe { RUST_IRQ_HANDLER.write(opt_fn) }
}
