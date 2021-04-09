use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct BackgroundControl(u16);
impl BackgroundControl {
  const_new!();
  bitfield_int!(u16; 0..=1: u8, priority, with_priority, set_priority);
  bitfield_int!(u16; 2..=3: u8, char_base_block, with_char_base_block, set_char_base_block);
  bitfield_bool!(u16; 6, mosaic, with_mosaic, set_mosaic);
  bitfield_bool!(u16; 7, is_8bpp, with_is_8bpp, set_is_8bpp);
  bitfield_int!(u16; 8..=12: u8, screen_base_block, with_screen_base_block, set_screen_base_block);
  bitfield_bool!(u16; 13, affine_overflow_wrapped, with_affine_overflow_wrapped, set_affine_overflow_wrapped);
  bitfield_int!(u16; 14..=15: u8, screen_size, with_screen_size, set_screen_size);
}
