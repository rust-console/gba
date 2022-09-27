use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum TimerPrescale {
  #[default]
  _1 = 0,
  _64 = 1,
  _256 = 2,
  _1024 = 3,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TimerControl(u16);
impl TimerControl {
  pub_const_fn_new_zeroed!();
  u16_enum_field!(0 - 1: TimerPrescale, prescale, with_prescale);
  u16_bool_field!(2, chained, with_chained);
  u16_bool_field!(6, overflow_irq, with_overflow_irq);
  u16_bool_field!(7, enabled, with_enabled);
}
