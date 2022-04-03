use super::*;

/// Stores the values to sample a number in `0 .. N`
///
/// Making one of these performs a division operation. In comparison,
/// [`Gen32::next_bounded`] will avoid needing to do a division much of the
/// time. Thus, unless you need to sample repeatedly from a specific bounded
/// range, simply calling `next_bounded` directly might be more efficient.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundedRandU16 {
  /// number of possible outputs. outputs will be in `0 .. count`
  count: u16,
  /// Multiplication threshold thing.
  ///
  /// <https://arxiv.org/abs/1805.10941>
  threshold: u16,
}
impl BoundedRandU16 {
  /// Constructs a new `BoundedRandU32`.
  ///
  /// ## Panics
  /// If the count is 0.
  #[inline]
  pub const fn new(count: u16) -> Self {
    let threshold = count.wrapping_neg() % count;
    Self { count, threshold }
  }

  /// Constructs a new `BoundedRandU32`, or `None` on failure.
  ///
  /// ## Failure
  /// If the count is 0.
  #[inline]
  pub const fn try_new(count: u16) -> Option<Self> {
    if count > 0 {
      Some(Self::new(count))
    } else {
      None
    }
  }

  /// The number of possible outputs.
  #[inline]
  pub const fn count(self) -> u16 {
    self.count
  }

  /// Given a `u32`, place it into this bounded range.
  ///
  /// ## Failure
  /// * If the value is such that it doesn't fit evenly it is rejected.
  #[inline]
  pub const fn place_in_range(self, val: u16) -> Option<u16> {
    let mul: u32 = (val as u32).wrapping_mul(self.count as u32);
    let low_part: u16 = mul as u16;
    if low_part < self.threshold {
      None
    } else {
      //debug_assert!(((mul >> 32) as u32) < self.count());
      Some((mul >> 16) as u16)
    }
  }

  /// Given a gen, sample from the gen until `place_in_range` succeeds.
  #[inline]
  pub fn sample<G: Gen32 + ?Sized>(self, gen: &mut G) -> u16 {
    loop {
      if let Some(output) = self.place_in_range(gen.next_u16()) {
        return output;
      }
    }
  }
}
