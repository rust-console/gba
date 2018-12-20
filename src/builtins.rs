#![allow(missing_docs)]

//! The module to provide "builtin" functions that LLVM expects.
//!
//! You shouldn't need to call anything in here yourself, it just has to be in
//! the translation unit and LLVM will find it.

//TODO: make 64 bit too
#[no_mangle]
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"))]
pub extern "C" fn __clzsi2(mut x: usize) -> usize {
  let mut y: usize;
  let mut n: usize = {
    #[cfg(target_pointer_width = "64")]
    {
      64
    }
    #[cfg(target_pointer_width = "32")]
    {
      32
    }
    #[cfg(target_pointer_width = "16")]
    {
      16
    }
  };
  #[cfg(target_pointer_width = "64")]
  {
    y = x >> 32;
    if y != 0 {
      n = n - 32;
      x = y;
    }
  }
  #[cfg(target_pointer_width = "32")]
  {
    y = x >> 16;
    if y != 0 {
      n = n - 16;
      x = y;
    }
  }
  y = x >> 8;
  if y != 0 {
    n = n - 8;
    x = y;
  }
  y = x >> 4;
  if y != 0 {
    n = n - 4;
    x = y;
  }
  y = x >> 2;
  if y != 0 {
    n = n - 2;
    x = y;
  }
  y = x >> 1;
  if y != 0 {
    n - 2
  } else {
    n - x
  }
}

#[test]
fn __clzsi2_test() {
  let mut i = 1 << 63;
  while i > 0 {
    assert_eq!(__clzsi2(i), i.leading_zeros() as usize);
    i >>= 1;
  }
}

// TODO: add some shims
// #[no_mangle] extern "aapcs" fn __aeabi_uidiv(num: u32: denom: u32) -> u32
// #[no_mangle] extern "aapcs" fn __aeabi_idiv(num: i32: denom: i32) -> u32
