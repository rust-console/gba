use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayStatus(u16);
impl DisplayStatus {
  const_new!();
  bitfield_bool!(u16; 0, is_vblank, with_is_vblank, set_is_vblank);
  bitfield_bool!(u16; 1, is_hblank, with_is_hblank, set_is_hblank);
  bitfield_bool!(u16; 2, is_vcount, with_is_vcount, set_is_vcount);
  bitfield_bool!(u16; 3, vblank_irq_enabled, with_vblank_irq_enabled, set_vblank_irq_enabled);
  bitfield_bool!(u16; 4, hblank_irq_enabled, with_hblank_irq_enabled, set_hblank_irq_enabled);
  bitfield_bool!(u16; 5, vcount_irq_enabled, with_vcount_irq_enabled, set_vcount_irq_enabled);
  bitfield_int!(u16; 8..=15: u16, vcount, with_vcount, set_vcount);
}
