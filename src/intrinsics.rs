//! Intrinsics that LLVM looks for when compiling.

/// Signed division
#[no_mangle]
#[inline]
pub extern "aapcs" fn __aeabi_idiv(n: i32, d: i32) -> i32 {
  assert!(d != 0);
  let div_out: i32;
  let _mod_out: i32;
  unsafe {
    asm!(/* assembly template */ "swi 0x06"
        :/* output operands */ "={r0}"(div_out), "={r1}"(_mod_out)
        :/* input operands */ "{r0}"(n), "{r1}"(d)
        :/* clobbers */ "r3"
        :/* options */
    );
  }
  div_out
}

/// Signed division alias for glibc reasons
#[no_mangle]
#[inline]
pub extern "aapcs" fn __divsi3(n: i32, d: i32) -> i32 {
  // Note the different naming scheme.
  __aeabi_idiv(n, d)
}

/// Unsigned division gets cast into signed values, divided, and cast back
#[no_mangle]
#[inline]
pub extern "aapcs" fn __aeabi_uidiv(n: u32, d: u32) -> u32 {
  __aeabi_idiv(n as i32, d as i32) as u32
}

/// Unsigned division alias for glibc reasons
#[no_mangle]
#[inline]
pub extern "aapcs" fn __udivsi3(n: u32, d: u32) -> u32 {
  // Note the different naming scheme.
  __aeabi_uidiv(n, d)
}

/// Count leading zeroes, required in debug mode for unknown reasons
#[no_mangle]
#[inline]
pub extern "aapcs" fn __clzsi2(x: i32) -> i32 {
  let mut y = -(x >> 16); // If left half of x is 0,
  let mut m = (y >> 16) & 16; // set n = 16. If left half
  let mut n = 16 - m; // is nonzero, set n = 0 and
  let mut x = x >> m; // shift x right 16.
                      // Now x is of the form 0000xxxx.
  y = x - 0x100; // If positions 8-15 are 0,
  m = (y >> 16) & 8; // add 8 to n and shift x left 8.
  n = n + m;
  x = x << m;

  y = x - 0x1000; // If positions 12-15 are 0,
  m = (y >> 16) & 4; // add 4 to n and shift x left 4.
  n = n + m;
  x = x << m;

  y = x - 0x4000; // If positions 14-15 are 0,
  m = (y >> 16) & 2; // add 2 to n and shift x left 2.
  n = n + m;
  x = x << m;

  y = x >> 14; // Set y = 0, 1, 2, or 3.
  m = y & !(y >> 1); // Set m = 0, 1, 2, or 2 resp.
  n + 2 - m
}
