//! Module that holds stuff for the color blending ability.

use super::*;

/// Color Special Effects Selection (R/W)
pub const BLDCNT: VolAddress<ColorEffectSetting> = unsafe { VolAddress::new_unchecked(0x400_0050) };

newtype! {
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
  ColorSpecialEffect = u16,
  None = 0,
  AlphaBlending = 1,
  BrightnessIncrease = 2,
  BrightnessDecrease = 3,
}

/// Alpha Blending Coefficients (R/W) (not W)
pub const BLDALPHA: VolAddress<AlphaBlendingSetting> = unsafe { VolAddress::new_unchecked(0x400_0052) };

newtype! {
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
pub const BLDY: VolAddress<BrightnessSetting> = unsafe { VolAddress::new_unchecked(0x400_0054) };

newtype! {
  BrightnessSetting, u32
}

impl BrightnessSetting {
  phantom_fields! {
    self.0: u32,
    evy_coefficient: 0-4,
  }
}
