#![no_std]
#![feature(start)]
#![feature(underscore_const_names)]

#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $old_name:ident) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}

#[macro_export]
macro_rules! const_assert {
  ($condition:expr) => {
    #[deny(const_err)]
    #[allow(dead_code)]
    const _: usize = 0 - !$condition as usize;
  };
}

/// Constructs an RGB value with a `const_assert!` that the input is in range.
#[macro_export]
macro_rules! const_rgb {
  ($r:expr, $g:expr, $b:expr) => {{
    const_assert!($r <= 31);
    const_assert!($g <= 31);
    const_assert!($b <= 31);
    Color::new($r, $g, $b)
  }};
}

// TODO: kill this
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<T>(pub *mut T);
impl<T> VolatilePtr<T> {
  pub unsafe fn read(&self) -> T {
    core::ptr::read_volatile(self.0)
  }
  pub unsafe fn write(&self, data: T) {
    core::ptr::write_volatile(self.0, data);
  }
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.wrapping_offset(count))
  }
}

newtype! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  Color, u16
}

impl Color {
  /// Combines the Red, Blue, and Green provided into a single color value.
  pub const fn new(red: u16, green: u16, blue: u16) -> Color {
    Color(blue << 10 | green << 5 | red)
  }
}

newtype! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  DisplayControlSetting, u16
}

pub const DISPLAY_CONTROL: VolatilePtr<DisplayControlSetting> = VolatilePtr(0x0400_0000 as *mut DisplayControlSetting);
pub const JUST_MODE3: DisplayControlSetting = DisplayControlSetting(3);
pub const JUST_BG2: DisplayControlSetting = DisplayControlSetting(0b100_0000_0000);
pub const JUST_MODE3_AND_BG2: DisplayControlSetting = DisplayControlSetting(JUST_MODE3.0 | JUST_BG2.0);

pub struct Mode3;
impl Mode3 {
  const SCREEN_WIDTH: isize = 240;
  const PIXELS: VolatilePtr<Color> = VolatilePtr(0x600_0000 as *mut Color);

  pub unsafe fn draw_pixel_unchecked(col: isize, row: isize, color: Color) {
    Self::PIXELS.offset(col + row * Self::SCREEN_WIDTH).write(color);
  }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    DISPLAY_CONTROL.write(JUST_MODE3_AND_BG2);
    Mode3::draw_pixel_unchecked(120, 80, const_rgb!(31, 0, 0));
    Mode3::draw_pixel_unchecked(136, 80, const_rgb!(0, 31, 0));
    Mode3::draw_pixel_unchecked(120, 96, const_rgb!(0, 0, 31));
    loop {}
  }
}
