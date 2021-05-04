use crate::*;
use crate::random::*;

/// A default seed for any PCG.
///
/// Truncate to fit, as necessary.
pub const DEFAULT_PCG_SEED: u128 = 201526561274146932589719779721328219291;

/// A default `inc` for any PCG.
///
/// Truncate to fit, as necessary.
pub const DEFAULT_PCG_INC: u128 = 34172814569070222299;


// Other multipliers: 0xffffffff0e703b65 0xf2fc5985
const PCG_MULTIPLIER_32: u32 = 0xf13283ad;

make_jump_lcgX!(jump_lcg32, u32);

/// A [permuted congruential
/// generator](https://en.wikipedia.org/wiki/Permuted_congruential_generator)
/// with 32 bits of output per step.
///
/// * Generally you should create new generator values with the
///   [`seed`](Self::seed) constructor. This will shuffle around the inputs
///   somewhat, so it will work alright even with "boring" input values like
///   `seed(0,0)` or whatever.
/// * If you want to exactly save/restore a generator use the `Into` and `From`
///   impls to convert the generator into and from a `[u32; 2]`.
/// * The methods on this type are quite minimal. You're expected to use the
///   [`Gen32`] trait to provide most of the useful operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RNG {
  state: u32,
  inc: u32,
}

impl RNG {
  /// Seed a new generator.
  pub const fn seed(seed: u32, inc: u32) -> Self {
    let inc = (inc << 1) | 1;
    let mut state = pcg_core_state32!(0_u32, inc);
    state = state.wrapping_add(seed);
    state = pcg_core_state32!(state, inc);
    Self { state, inc }
  }

  /// Gets the next 32-bits of output.
  #[inline]
  pub fn next_u32(&mut self) -> u32 {
    // LLVM do the instruction-level parallelism plz ;_;
    let out = rxs_m_xs_u32_to_u32!(self.state);
    self.state = pcg_core_state32!(self.state, self.inc);
    out
  }
  /// Gets the next 16-bits of output
  #[inline]
  pub fn next_u16(&mut self) -> u16 {
    let out = pcg_xsh_rs_u32_to_u16!(self.state);
    self.state = pcg_core_state32!(self.state, self.inc);
    out
  }
  /// Jumps the generator by `delta` steps forward.
  ///
  /// The generator sequence loops, so if you want to go "backwards" you can
  /// just subtract the number of steps you want to go back from `u32::MAX` and
  /// jump by that amount.
  #[inline]
  pub fn jump(&mut self, delta: u32) {
    self.state = jump_lcg32(delta, self.state, PCG_MULTIPLIER_32, self.inc);
  }
}

impl Default for RNG {
  fn default() -> Self {
    const THE_DEFAULT: RNG = RNG::seed(DEFAULT_PCG_SEED as _, DEFAULT_PCG_INC as _);
    THE_DEFAULT
  }
}

impl From<[u32; 2]> for RNG {
  fn from([state, inc]: [u32; 2]) -> Self {
    Self { state, inc }
  }
}

impl From<RNG> for [u32; 2] {
  fn from(pcg: RNG) -> Self {
    [pcg.state, pcg.inc]
  }
}

impl Gen32 for RNG {
  fn next_u32(&mut self) -> u32 {
    RNG::next_u32(self)
  }
  fn next_u16(&mut self) -> u16 {
    RNG::next_u16(self)
  }
}
