//!

/// A color value.
///
/// This is a bit-packed linear RGB color value with 5 bits per channel:
/// ```text
/// 0bX_BBBBB_GGGGG_RRRRR
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);

#[cfg(feature = "bytemuck")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "bytemuck")))]
unsafe impl bytemuck::Zeroable for Color {}
#[cfg(feature = "bytemuck")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "bytemuck")))]
unsafe impl bytemuck::Pod for Color {}
