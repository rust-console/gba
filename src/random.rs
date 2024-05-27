//! Randomization routines that will work well enough on the GBA.
//!
//! Randomization is basically about a trade off between time taken to produce
//! each next output and the quality of that output. Most modern randomization
//! libraries do 64-bit randomization and aim to be extremely unpredictable.
//! That's basically overkill on the GBA. The random generators here are 32-bit,
//! and they are relatively simple while maintaining reasonable output.

/// A [Linear Congruential Generator][wp-lcg] with 32-bits of output.
///
/// [wp-lcg]: https://en.wikipedia.org/wiki/Linear_congruential_generator
///
/// This holds a single `u32` as the state. Advancing the generator requires
/// only a multiply and an add, so it's fairly fast as a PRNG. The output
/// quality isn't particularly great as a result, and if you need less than 32
/// bits of randomness at a time you're advised to use the *upper* bits from
/// each output of this generator, which will be the better bits. The [Gen32]
/// impl of this type will handle that for you, if you use that trait.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
// This particular `MUL` value comes from "Tables of linear congruential
// generators of different sizes and good lattice structure" by Pierre L'Ecuyer.
// We select 1 as the `ADD` value just to have an odd increment, and because
// `ADD` values of 3 bits or less can be encoded with an immediate
// instruction in both a32 and t32 code.
pub struct Lcg32(GenericLcg32<32310901, 1>);
impl Lcg32 {
  /// Wraps the `u32` as an `Lcg32`.
  ///
  /// This doesn't do any manipulation of the input state to try and help seed
  /// the value, it just uses the value directly.
  #[inline]
  pub const fn new(state: u32) -> Self {
    Self(GenericLcg32::new(state))
  }

  /// Advances the generator one step, producing a `u32` of output.
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    self.0.next_u32()
  }

  /// Advances the generator by `delta` steps all at once.
  ///
  /// Because the generator output sequence loops, large `delta` values allow
  /// you to "reverse" the generator.
  #[inline]
  pub fn jump_state(&mut self, delta: u32) {
    self.0.jump_state(delta)
  }
}

/// A [Linear Congruential Generator][wp-lcg] with a const generic multiplier
/// and increment.
///
/// [wp-lcg]: https://en.wikipedia.org/wiki/Linear_congruential_generator
///
/// The `ADD` value can be any value at all. Different `ADD` values will reorder
/// the sequence that you get from a particular `MUL` value. For best results
/// `ADD` should be an odd value. An even `ADD` value gives a generator with a
/// significantly shorter period than an odd `ADD` value.
///
/// The `MUL` value must be carefully selected, because it has an overwhelming
/// impact on the generator's output quality. See the linked wikipedia article
/// (and its references) if you want information on what `MUL` values you might
/// want to use.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct GenericLcg32<const MUL: u32, const ADD: u32>(u32);
impl<const MUL: u32, const ADD: u32> GenericLcg32<MUL, ADD> {
  /// Wraps the `u32` as a generic LCG.
  ///
  /// This doesn't do any manipulation of the input state to try and help seed
  /// the value, it just uses the value directly.
  #[inline]
  pub const fn new(state: u32) -> Self {
    Self(state)
  }

  /// Advances the generator one step, producing a `u32` of output.
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    let next_state = self.0.wrapping_mul(MUL).wrapping_add(ADD);
    self.0 = next_state;
    next_state
  }

  /// Advances the generator by `delta` steps all at once.
  ///
  /// Because the generator output sequence loops, large `delta` values allow
  /// you to "reverse" the generator.
  #[inline]
  pub fn jump_state(&mut self, mut delta: u32) {
    let mut cur_mult: u32 = MUL;
    let mut cur_plus: u32 = ADD;
    let mut acc_mult: u32 = 1;
    let mut acc_plus: u32 = 0;
    while delta > 0 {
      if (delta & 1) > 0 {
        acc_mult = acc_mult.wrapping_mul(cur_mult);
        acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
      }
      cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
      cur_mult = cur_mult.wrapping_mul(cur_mult);
      delta /= 2;
    }
    self.0 = acc_mult.wrapping_mul(self.0).wrapping_add(acc_plus);
  }
}

/// A trait for pseudorandom number generators that have `u32`
/// output from each step of the generator.
pub trait Gen32 {
  /// Advance the generator to produce the next `u32`.
  fn next_u32(&mut self) -> u32;

  /// Produce a `u16`.
  #[inline]
  fn next_u16(&mut self) -> u16 {
    (self.next_u32() >> 16) as u16
  }

  /// Produce a `u8`.
  #[inline]
  fn next_u8(&mut self) -> u8 {
    (self.next_u32() >> 24) as u8
  }

  /// Produce a `bool`.
  #[inline]
  fn next_bool(&mut self) -> bool {
    (self.next_u16() as i32) < 0
  }

  /// Produce a value that's strictly less than `b`.
  ///
  /// ## Panics
  /// * If `b` is zero.
  #[inline]
  #[track_caller]
  fn next_bounded(&mut self, b: u16) -> u16 {
    assert!(b != 0, "Gen32::next_bounded> Bound must be non-zero.");
    let mut x: u32 = self.next_u16() as u32;
    let mut mul: u32 = (b as u32).wrapping_mul(x);
    let mut low: u16 = mul as u16;
    if low < b {
      let threshold = b.wrapping_neg() % b;
      while low < threshold {
        x = self.next_u32();
        mul = (b as u32).wrapping_mul(x);
        low = mul as u16;
      }
    }
    let high = (mul >> 16) as u16;
    high
  }

  /// Pick a random element of the slice, by value.
  ///
  /// ## Panics
  /// * If the length is 0.
  #[inline]
  fn pick<T: Copy>(&mut self, buf: &[T]) -> T {
    let len16: u16 = saturating_usize_as_u16(buf.len());
    buf[self.next_bounded(len16) as usize]
  }

  /// Pick a random element of the slice, by shared reference.
  ///
  /// ## Panics
  /// * If the length is 0.
  #[inline]
  fn pick_ref<'b, T>(&mut self, buf: &'b [T]) -> &'b T {
    let len16: u16 = saturating_usize_as_u16(buf.len());
    &buf[self.next_bounded(len16) as usize]
  }

  /// Pick a random element of the slice, by unique reference.
  ///
  /// ## Panics
  /// * If the length is 0.
  #[inline]
  fn pick_mut<'b, T>(&mut self, buf: &'b mut [T]) -> &'b mut T {
    let len16: u16 = saturating_usize_as_u16(buf.len());
    &mut buf[self.next_bounded(len16) as usize]
  }

  /// Shuffles the elements of the slice.
  ///
  /// On an empty slice this is a no-op.
  #[inline]
  fn shuffle<T>(&mut self, buf: &mut [T]) {
    if let Some(end_index) = buf.len().checked_sub(1) {
      let mut possibility_count: u16 = saturating_usize_as_u16(buf.len());
      let mut this_index: usize = 0;
      while this_index < end_index {
        let offset = self.next_bounded(possibility_count) as usize;
        buf.swap(this_index, this_index + offset);
        possibility_count -= 1;
        this_index += 1;
      }
    }
  }
}

impl Gen32 for Lcg32 {
  #[inline]
  fn next_u32(&mut self) -> u32 {
    Lcg32::next_u32(self)
  }
}

#[inline]
const fn saturating_usize_as_u16(val: usize) -> u16 {
  if val <= u16::MAX as usize {
    val as u16
  } else {
    u16::MAX
  }
}
