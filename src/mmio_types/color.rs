use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Color(pub u16);
impl Color {
  const_new!();
  bitfield_int!(u16; 0..=4: u8, red, with_red, set_red);
  bitfield_int!(u16; 5..=9: u8, green, with_green, set_green);
  bitfield_int!(u16; 10..=14: u8, blue, with_blue, set_blue);
}
impl Color {
  pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
    let r = red as u16;
    let g = green as u16;
    let b = blue as u16;
    Self(b << 10 | g << 5 | r)
  }
}
