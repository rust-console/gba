#![allow(missing_docs)]

//! The module to provide "builtin" functions that LLVM expects.
//!
//! You shouldn't need to call anything in here yourself, it just has to be in
//! the translation unit and LLVM will find it.

#[no_mangle]
pub unsafe extern "C" fn __clzsi2(mut x: usize) -> usize {
  let mut y: usize;
  let mut n: usize = 32;
  y = x >> 16;
  if y != 0 {
    n = n - 16;
    x = y;
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
