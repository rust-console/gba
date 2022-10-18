// Note(Lokathor): We have a generic LCG type below, but for now we can hide the
// process of having to pick what multiplier and increment to use behind a
// newtype that selects some default constants.

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Lcg32(GenericLcg32<32310901, 1>);
impl Lcg32 {
  #[inline]
  pub const fn new(state: u32) -> Self {
    Self(GenericLcg32::new(state))
  }
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    self.0.next_u32()
  }
  #[inline]
  pub fn jump_state(&mut self, delta: u32) {
    self.0.jump_state(delta)
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct GenericLcg32<const MUL: u32, const ADD: u32>(u32);
impl<const MUL: u32, const ADD: u32> GenericLcg32<MUL, ADD> {
  #[inline]
  pub const fn new(state: u32) -> Self {
    Self(state)
  }

  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    let next_state = self.0.wrapping_mul(MUL).wrapping_add(ADD);
    next_state
  }

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

pub trait Gen32 {
  fn next_u32(&mut self) -> u32;

  #[inline]
  fn next_u16(&mut self) -> u16 {
    (self.next_u32() >> 16) as u16
  }

  #[inline]
  fn next_u8(&mut self) -> u8 {
    (self.next_u32() >> 24) as u8
  }

  #[inline]
  fn next_bool(&mut self) -> bool {
    (self.next_u16() as i32) < 0
  }

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

  #[inline]
  fn pick<T: Copy>(&mut self, buf: &[T]) -> T {
    let len16: u16 = saturating_usize_as_u16(buf.len());
    buf[self.next_bounded(len16) as usize]
  }

  #[inline]
  fn pick_ref<'b, T>(&mut self, buf: &'b [T]) -> &'b T {
    let len16: u16 = saturating_usize_as_u16(buf.len());
    &buf[self.next_bounded(len16) as usize]
  }

  #[inline]
  fn pick_mut<'b, T>(&mut self, buf: &'b mut [T]) -> &'b mut T {
    let len16: u16 = saturating_usize_as_u16(buf.len());
    &mut buf[self.next_bounded(len16) as usize]
  }

  #[inline]
  fn shuffle<T>(&mut self, buf: &mut [T]) {
    let mut possibility_count: u16 = saturating_usize_as_u16(buf.len());
    let mut this_index: usize = 0;
    let end_index = buf.len() - 1;
    while this_index < end_index {
      let offset = self.next_bounded(possibility_count) as usize;
      buf.swap(this_index, this_index + offset);
      possibility_count -= 1;
      this_index += 1;
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
