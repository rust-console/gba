use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct FifoReset(u16);
impl FifoReset {
  const_new!();
  bitfield_bool!(u16; 11, reset_a, with_reset_fifo_a, set_reset_fifo_a);
  bitfield_bool!(u16; 15, reset_b, with_reset_fifo_b, set_reset_fifo_b);
}
