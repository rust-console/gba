

//! Module for random number generation  
//! 
//! This module provides functions and utilites for randomly generated values
//! # Usage
//! ```rust
//! use gba::random::RNG;
//!
//! let mut rng = RNG::seed(123, 321);
//! let x = rng.next_u32();
//! ```
//! 
//! 
//! 

macro_rules! make_jump_lcgX {
  ($(#[$attr:meta])* $f:ident, $u:ty) => {
    $(#[$attr])*
    /// Gives the state `delta` steps from now in `log(delta)` time.
    #[must_use]
    #[inline(always)]
    const fn $f(mut delta: $u, state: $u, mult: $u, inc: $u) -> $u {
      let mut cur_mult: $u = mult;
      let mut cur_plus: $u = inc;
      let mut acc_mult: $u = 1;
      let mut acc_plus: $u = 0;
      while delta > 0 {
        if (delta & 1) > 0 {
          acc_mult = acc_mult.wrapping_mul(cur_mult);
          acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
        }
        cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
        cur_mult = cur_mult.wrapping_mul(cur_mult);
        delta /= 2;
      }
      acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
    }
  };
}

mod gen32;
pub use gen32::*;

mod pcg32;
pub use pcg32::*;

mod bounded_rand;
pub use bounded_rand::*;

mod algorithms;