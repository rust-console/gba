//! Module for Background controls

use super::*;

newtype! {
  BackgroundControlSetting, u16
}
impl BackgroundControlSetting {
  pub const fn from_screen_base_block(screen_base_block: u16) -> Self {
    BackgroundControlSetting(screen_base_block << 8) // TODO: mask this for correctness
  }
}

pub struct BG0;
impl BG0 {
  pub const BG0CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_0008) };
}
