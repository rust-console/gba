use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimerControl(u8);
impl TimerControl {
  const_new!();
  bitfield_int!(u8; 0..=1: u8, prescaler_selection, with_prescaler_selection, set_prescaler_selection);
  bitfield_bool!(u8; 2, chained_counting, with_chained_counting, set_chained_counting);
  bitfield_bool!(u8; 6, irq_on_overflow, with_irq_on_overflow, set_irq_on_overflow);
  bitfield_bool!(u8; 7, enabled, with_enabled, set_enabled);
}
