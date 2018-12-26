//! Module for Background controls

use super::*;

newtype! {
  /// A newtype over the various display control options that you have on a GBA.
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  BackgroundControlSetting, u16
}
impl BackgroundControlSetting {
  pub const fn from_screen_base_block(screen_base_block: u16) -> Self {
    BackgroundControlSetting(screen_base_block << 8) // TODO: mask this for correctness
  }

  //

  pub const PRIORITY_MASK: u16 = 0b11 << 0;
  pub const fn priority(self) -> u16 {
    self.0 & Self::PRIORITY_MASK
  }
  pub const fn with_priority(self, priority: u16) -> Self {
    BackgroundControlSetting((self.0 & !Self::PRIORITY_MASK) | priority)
  }

  pub const CHARACTER_BASE_BLOCK_MASK: u16 = 0b11 << 2;
  pub const fn character_base_block(self) -> u16 {
    (self.0 & Self::CHARACTER_BASE_BLOCK_MASK) >> 2
  }
  pub const fn with_character_base_block(self, character_base_block: u16) -> Self {
    BackgroundControlSetting((self.0 & !Self::CHARACTER_BASE_BLOCK_MASK) | ((character_base_block << 2) & Self::CHARACTER_BASE_BLOCK_MASK))
  }
}

pub const BG0CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_0008) };
pub const BG1CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_000A) };
pub const BG2CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_000C) };
pub const BG3CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new_unchecked(0x400_000E) };
