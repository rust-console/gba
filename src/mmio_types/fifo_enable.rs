use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct FifoEnable(u32);
impl FifoEnable {
  const_new!();
  bitfield_bool!(u32; 7, enabled, with_enabled, set_enabled);
}