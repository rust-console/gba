//! Module that holds stuff for the Window ability.

use super::*;

/// Window 0 Horizontal Dimensions (W)
pub const WIN0H: VolAddress<HorizontalWindowSetting> = unsafe { VolAddress::new(0x400_0040) };

/// Window 1 Horizontal Dimensions (W)
pub const WIN1H: VolAddress<HorizontalWindowSetting> = unsafe { VolAddress::new(0x400_0042) };

newtype! {
  /// TODO: docs
  HorizontalWindowSetting, u16
}

impl HorizontalWindowSetting {
  phantom_fields! {
    self.0: u16,
    col_end: 0-7,
    col_start: 8-15,
  }
}

/// Window 0 Vertical Dimensions (W)
pub const WIN0V: VolAddress<VerticalWindowSetting> = unsafe { VolAddress::new(0x400_0044) };

/// Window 1 Vertical Dimensions (W)
pub const WIN1V: VolAddress<VerticalWindowSetting> = unsafe { VolAddress::new(0x400_0046) };

newtype! {
  /// TODO: docs
  VerticalWindowSetting, u16
}

impl VerticalWindowSetting {
  phantom_fields! {
    self.0: u16,
    row_end: 0-7,
    row_start: 8-15,
  }
}

/// Control of Inside of Window(s) (R/W)
pub const WININ: VolAddress<InsideWindowSetting> = unsafe { VolAddress::new(0x400_0048) };

newtype! {
  /// TODO: docs
  InsideWindowSetting, u16
}

impl InsideWindowSetting {
  phantom_fields! {
    self.0: u16,
    win0_bg0: 0,
    win0_bg1: 1,
    win0_bg2: 2,
    win0_bg3: 3,
    win0_obj: 4,
    win0_color_special: 5,
    win1_bg0: 8,
    win1_bg1: 9,
    win1_bg2: 10,
    win1_bg3: 11,
    win1_obj: 12,
    win1_color_special: 13,
  }
}

///  Control of Outside of Windows & Inside of OBJ Window (R/W)
pub const WINOUT: VolAddress<OutsideWindowSetting> = unsafe { VolAddress::new(0x400_004A) };

newtype! {
  /// TODO: docs
  OutsideWindowSetting, u16
}

impl OutsideWindowSetting {
  phantom_fields! {
    self.0: u16,
    outside_bg0: 0,
    outside_bg1: 1,
    outside_bg2: 2,
    outside_bg3: 3,
    outside_obj: 4,
    outside_color_special: 5,
    obj_win_bg0: 8,
    obj_win_bg1: 9,
    obj_win_bg2: 10,
    obj_win_bg3: 11,
    obj_win_obj: 12,
    obj_win_color_special: 13,
  }
}
