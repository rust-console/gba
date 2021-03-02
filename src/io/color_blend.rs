//! Module that holds stuff for the color blending ability.

use super::*;

/// Color Special Effects Selection (R/W)
pub const BLDCNT: VolAddress<ColorEffectSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0050) };

newtype! {
  /// TODO: docs
  ColorEffectSetting, u16
}

impl ColorEffectSetting {
  phantom_fields! {
    self.0: u16,
    bg0_1st_target_pixel: 0,
    bg1_1st_target_pixel: 1,
    bg2_1st_target_pixel: 2,
    bg3_1st_target_pixel: 3,
    obj_1st_target_pixel: 4,
    backdrop_1st_target_pixel: 5,
    color_special_effect: 6-7=ColorSpecialEffect<None, AlphaBlending, BrightnessIncrease, BrightnessDecrease>,
    bg0_2nd_target_pixel: 8,
    bg1_2nd_target_pixel: 9,
    bg2_2nd_target_pixel: 10,
    bg3_2nd_target_pixel: 11,
    obj_2nd_target_pixel: 12,
    backdrop_2nd_target_pixel: 13,
  }
}

newtype_enum! {
  /// TODO: docs
  ColorSpecialEffect = u16,
  /// TODO: docs
  None = 0,
  /// TODO: docs
  AlphaBlending = 1,
  /// TODO: docs
  BrightnessIncrease = 2,
  /// TODO: docs
  BrightnessDecrease = 3,
}

/// Alpha Blending Coefficients (R/W) (not W)
pub const BLDALPHA: VolAddress<AlphaBlendingSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0052) };

newtype! {
  /// TODO: docs
  AlphaBlendingSetting, u16
}

impl AlphaBlendingSetting {
  phantom_fields! {
    self.0: u16,
    eva_coefficient: 0-4,
    evb_coefficient: 8-12,
  }
}

/// Brightness (Fade-In/Out) Coefficient (W) (not R/W)
pub const BLDY: VolAddress<BrightnessSetting, Safe, Safe> = unsafe { VolAddress::new(0x400_0054) };

newtype! {
  /// TODO: docs
  BrightnessSetting, u32
}

impl BrightnessSetting {
  phantom_fields! {
    self.0: u32,
    evy_coefficient: 0-4,
  }
}
