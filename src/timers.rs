use crate::macros::{const_new, u8_bool_field, u8_value_field};
use voladdress::{Safe, VolAddress};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimerControl(u8);
impl TimerControl {
  const_new!();
  u8_value_field!(0 - 1, prescaler_selection, with_prescaler_selection);
  u8_bool_field!(2, chained_counting, with_chained_counting);
  u8_bool_field!(6, irq_on_overflow, with_irq_on_overflow);
  u8_bool_field!(7, enabled, with_enabled);
}

/// [TM0CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER0_COUNTER: VolAddress<u16, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0100) };
/// [TM1CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER1_COUNTER: VolAddress<u16, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0104) };
/// [TM2CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER2_COUNTER: VolAddress<u16, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0108) };
/// [TM3CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER3_COUNTER: VolAddress<u16, Safe, ()> =
  unsafe { VolAddress::new(0x0400_010C) };

/// [TM0CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER0_RELOAD: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_0100) };
/// [TM1CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER1_RELOAD: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_0104) };
/// [TM2CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER2_RELOAD: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_0108) };
/// [TM3CNT_L](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER3_RELOAD: VolAddress<u16, (), Safe> =
  unsafe { VolAddress::new(0x0400_010C) };

/// [TM0CNT_H](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER0_CONTROL: VolAddress<TimerControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0102) };
/// [TM1CNT_H](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER1_CONTROL: VolAddress<TimerControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0106) };
/// [TM2CNT_H](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER2_CONTROL: VolAddress<TimerControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_010A) };
/// [TM3CNT_H](https://problemkaputt.de/gbatek.htm#gbatimers)
pub const TIMER3_CONTROL: VolAddress<TimerControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_010E) };
