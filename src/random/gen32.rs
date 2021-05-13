use core::convert::{TryFrom, TryInto};

use crate::mmio_types::Color;

/// A Generator with 32 bits of output per step.
pub trait Gen32 {
  /// Generates the next 32 bits of output.
  fn next_u32(&mut self) -> u32;
  /// Generates the next 16-bits of output
  fn next_u16(&mut self) -> u16;
  
  /// Produces a `Color`
  fn next_color(&mut self) -> Color {
    Color(self.next_u16() & 0b0111111111111111)
  }
  /// Produce a `bool`
  #[inline(always)]
  fn next_bool(&mut self) -> bool {
    (self.next_u32() as i32) < 0
  }

  /// Produce a `u8`
  #[inline(always)]
  fn next_u8(&mut self) -> u8 {
    (self.next_u16() >> 8) as u8
  }

  /// Produce a `u64`
  #[inline(always)]
  fn next_u64(&mut self) -> u64 {
    let l = self.next_u32() as u64;
    let h = self.next_u32() as u64;
    h << 32 | l
  }

  /// Gives a value within `0 .. B`
  ///
  /// This is often more efficient than making a
  /// [`BoundedRandU32`](crate::random::BoundedRandU32) if you don't need to use a
  /// specific bound value more than once.
  ///
  /// ## Panics
  /// * If the input is 0.
  #[inline]
  fn next_bounded(&mut self, b: u16) -> u16 {
    assert!(b != 0, "Gen32::next_bounded> Bound must be non-zero.");
    let mut x = self.next_u16() as u32;
    let mut mul = (b as u32).wrapping_mul(x);
    let mut low = mul as u16;
    if low < b {
      let threshold = b.wrapping_neg() % b;
      while low < threshold {
        x = self.next_u32() as u32;
        mul = (b as u32).wrapping_mul(x);
        low = mul as u16;
      }
    }
    let high = (mul >> 16) as u16;
    high
  }

  /// Gets a value out of the slice given (by copy).
  ///
  /// * The default impl will not pick past index `u16::MAX`.
  #[inline(always)]
  fn pick<T>(&mut self, buf: &[T]) -> T
  where
    Self: Sized,
    T: Copy,
  {
    let end: u16 = saturating_usize_as_u16(buf.len());
    buf[usize::try_from(self.next_bounded(end)).unwrap()]
  }

  /// Gets a value out of the slice given (by shared ref).
  ///
  /// * The default impl will not pick past index `u16::MAX`.
  #[inline(always)]
  fn pick_ref<'b, T>(&mut self, buf: &'b [T]) -> &'b T
  where
    Self: Sized,
  {
    let end: u16 = saturating_usize_as_u16(buf.len());
    &buf[usize::try_from(self.next_bounded(end)).unwrap()]
  }

  /// Gets a value out of the slice given (by unique ref).
  ///
  /// * The default impl will not pick past index `u16::MAX`.
  #[inline(always)]
  fn pick_mut<'b, T>(&mut self, buf: &'b mut [T]) -> &'b mut T
  where
    Self: Sized,
  {
    let end: u16 = saturating_usize_as_u16(buf.len());
    &mut buf[usize::try_from(self.next_bounded(end)).unwrap()]
  }

  /// Shuffles a slice in `O(len)` time.
  ///
  /// * The default impl shuffles only the first `u16::MAX` elements.
  #[inline]
  fn shuffle<T>(&mut self, buf: &mut [T])
  where
    Self: Sized,
  {
    // Note(Lokathor): The "standard" Fisher-Yates shuffle goes backward from
    // the end of the slice, but this version allows us to access memory forward
    // from the start to the end, so that we play more nicely with the
    // fetch-ahead of most modern CPUs.
    let mut possibility_count: u16 = buf.len().try_into().unwrap_or(u16::max_value());
    let mut this_index: usize = 0;
    let end = buf.len() - 1;
    while this_index < end {
      let offset = self.next_bounded(possibility_count) as usize;
      buf.swap(this_index, this_index + offset);
      possibility_count -= 1;
      this_index += 1;
    }
  }
}

// Asserts that `Gen32` is an object-safe trait.
const _: [&mut dyn Gen32; 0] = [];

/// Converts the `usize` into a `u16`, or gives `u16::MAX` if that wouldn't fit.
#[inline(always)]
const fn saturating_usize_as_u16(val: usize) -> u16 {
  if val <= u16::MAX as usize {
    val as u16
  } else {
    u16::MAX
  }
}
