use super::*;

/// `swi #0x06`: Performs `i32 / i32` in software.
///
/// Returns `(n/d, n%d, (n/d).abs_unsigned())`
///
/// ```txt
/// Div(-1234, 10) => (-123, -4, 123)
/// ```
///
/// ## Panics
/// * if `d` is 0.
#[inline]
#[must_use]
pub fn Div(n: i32, d: i32) -> (i32, i32, u32) {
  assert_ne!(d, 0);
  let n_div_d: i32;
  let n_mod_d: i32;
  let abs_n_div_d: u32;
  unsafe {
    asm! {
      "swi #0x06",
      inout("r0") n => n_div_d,
      inout("r1") d => n_mod_d,
      out("r3") abs_n_div_d,
      options(pure, nomem, preserves_flags)
    }
  };
  (n_div_d, n_mod_d, abs_n_div_d)
}

/// `swi #0x08`
#[inline]
#[must_use]
pub fn Sqrt(x: u32) -> u32 {
  let output: u32;
  unsafe {
    asm! {
      "swi #0x08",
      inout("r0") x => output,
      out("r1") _,
      out("r3") _,
      options(pure, nomem, preserves_flags)
    }
  };
  output
}

/// `swi #0x09`
#[inline]
#[must_use]
pub fn ArcTan(x: i16) -> i16 {
  let output: i16;
  unsafe {
    asm! {
      "swi #0x09",
      inout("r0") x => output,
      out("r1") _,
      out("r3") _,
      options(pure, nomem, preserves_flags)
    }
  };
  output
}

/// `swi #0x0A`
#[inline]
#[must_use]
pub fn ArcTan2(x: i16, y: i16) -> u32 {
  let output: u32;
  unsafe {
    asm! {
      "swi #0x0A",
      inout("r0") x as i32 as u32 => output,
      inout("r1") y => _,
      out("r3") _,
      options(pure, nomem, preserves_flags)
    }
  };
  output
}
