/// The GBA uses 5-bit-per-channel colors.
///
/// ```txt
/// 0bX_BBBBB_GGGGG_RRRRR
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);
unsafe impl bytemuck::Zeroable for Color {}
unsafe impl bytemuck::Pod for Color {}
impl Color {
  pub const fn from_rgb(r: u16, g: u16, b: u16) -> Self {
    Self(r | (g << 5) | (b << 10))
  }
}
impl From<u16> for Color {
  fn from(u: u16) -> Self {
    Self(u)
  }
}
impl From<Color> for u16 {
  fn from(c: Color) -> Self {
    c.0
  }
}
