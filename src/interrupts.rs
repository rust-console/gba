use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IrqBits(u16);
impl IrqBits {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, vblank);
  u16_bool_field!(1, hblank);
  u16_bool_field!(2, vcounter);
  u16_bool_field!(3, timer0);
  u16_bool_field!(4, timer1);
  u16_bool_field!(5, timer2);
  u16_bool_field!(6, timer3);
  u16_bool_field!(7, serial);
  u16_bool_field!(8, dma0);
  u16_bool_field!(9, dma1);
  u16_bool_field!(10, dma2);
  u16_bool_field!(11, dma3);
  u16_bool_field!(12, keypad);
  u16_bool_field!(13, gamepak);
}
