use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field};

/// A function you want called during an interrupt.
pub type IrqFn = unsafe extern "C" fn(IrqBits);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IrqBits(u16);
impl IrqBits {
  pub_const_fn_new_zeroed!();
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
  u16_bool_field!(13, gamepak, with_gamepak);

  pub const VBLANK: Self = Self::new().with_vblank(true);
  pub const HBLANK: Self = Self::new().with_hblank(true);
  pub const VCOUNTER: Self = Self::new().with_vcounter(true);
  pub const TIMER0: Self = Self::new().with_timer0(true);
  pub const TIMER1: Self = Self::new().with_timer1(true);
  pub const TIMER2: Self = Self::new().with_timer2(true);
  pub const TIMER3: Self = Self::new().with_timer3(true);
  pub const SERIAL: Self = Self::new().with_serial(true);
  pub const DMA0: Self = Self::new().with_dma0(true);
  pub const DMA1: Self = Self::new().with_dma1(true);
  pub const DMA2: Self = Self::new().with_dma2(true);
  pub const DMA3: Self = Self::new().with_dma3(true);
  pub const KEYPAD: Self = Self::new().with_keypad(true);
  pub const GAMEPAK: Self = Self::new().with_gamepak(true);

  #[inline]
  #[must_use]
  pub const fn to_u16(self) -> u16 {
    self.0
  }
}

// TODO: might want to support bit ops. But it's not super important right now
// since they can't be implented as const traits yet anyway.
