use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayStatus(u16);
impl DisplayStatus {
  bitfield_bool!(u16; 0, is_vblank, with_is_vblank, set_is_vblank);
  bitfield_bool!(u16; 1, is_hblank, with_is_hblank, set_is_hblank);
  bitfield_bool!(u16; 2, is_vcounter, with_is_vcounter, set_is_vcounter);
  bitfield_bool!(u16; 3, vblank_irq_enabled, with_vblank_irq_enabled, set_vblank_irq_enabled);
  bitfield_bool!(u16; 4, hblank_irq_enabled, with_hblank_irq_enabled, set_hblank_irq_enabled);
  bitfield_bool!(u16; 5, vcounter_irq_enabled, with_vcounter_irq_enabled, set_vcounter_irq_enabled);
  bitfield_int!(u16; 8..=15: u8, vcount, with_vcount, set_vcount);
}
