use crate::macros::{const_new, u16_bool_field, u16_value_field};
use voladdress::{Safe, VolAddress};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayStatus(u16);
impl DisplayStatus {
  const_new!();
  u16_bool_field!(0, is_vblank, with_is_vblank);
  u16_bool_field!(1, is_hblank, with_is_hblank);
  u16_bool_field!(2, is_vcounter_match, with_is_vcounter_match);
  u16_bool_field!(3, vblank_irq, with_vblank_irq);
  u16_bool_field!(4, hblank_irq, with_hblank_irq);
  u16_bool_field!(5, vcounter_irq, with_vcounter_irq);
  u16_value_field!(8 - 15, vcounter_setting, with_vcounter_setting);
}
pub const DISPSTAT: VolAddress<DisplayStatus, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0004) };
