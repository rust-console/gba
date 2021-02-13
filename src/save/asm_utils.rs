//! A module containing low-level assembly functions that can be loaded into
//! WRAM. Both flash media and battery-backed SRAM require reads to be
//! performed via code in WRAM and cannot be accessed by DMA.

#![cfg_attr(
  not(all(target_vendor = "nintendo", target_env = "agb")),
  allow(unused_variables, non_snake_case)
)]

#[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
global_asm!(include_str!("asm_routines.s"));

#[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
extern "C" {
  fn WramXferBuf(src: *const u8, dst: *mut u8, count: usize);
  fn WramReadByte(src: *const u8) -> u8;
  fn WramVerifyBuf(buf1: *const u8, buf2: *const u8, count: usize) -> bool;
}

#[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
fn WramXferBuf(src: *const u8, dst: *mut u8, count: usize) {
  unimplemented!()
}

#[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
fn WramReadByte(src: *const u8) -> u8 {
  unimplemented!()
}

#[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
fn WramVerifyBuf(buf1: *const u8, buf2: *const u8, count: usize) -> bool {
  unimplemented!()
}

/// Copies data from a given memory address into a buffer.
///
/// This should be used to access any data found in Flash or battery-backed
/// SRAM, as you must read those one byte at a time and from code stored
/// in WRAM.
///
/// This uses raw addresses into the memory space. Use with care.
#[inline(always)]
pub unsafe fn read_raw_buf(dst: &mut [u8], src: usize) {
  if dst.len() != 0 {
    WramXferBuf(src as _, dst.as_mut_ptr(), dst.len());
  }
}

/// Copies data from a buffer into a given memory address.
///
/// This is not strictly needed to write into save media, but reuses the
/// optimized loop used in `read_raw_buf`, and will often be faster.
///
/// This uses raw addresses into the memory space. Use with care.
#[inline(always)]
pub unsafe fn write_raw_buf(dst: usize, src: &[u8]) {
  if src.len() != 0 {
    WramXferBuf(src.as_ptr(), dst as _, src.len());
  }
}

/// Verifies that the data in a buffer matches that in a given memory address.
///
/// This should be used to access any data found in Flash or battery-backed
/// SRAM, as you must read those one byte at a time and from code stored
/// in WRAM.
///
/// This uses raw addresses into the memory space. Use with care.
#[inline(always)]
pub unsafe fn verify_raw_buf(buf1: &[u8], buf2: usize) -> bool {
  if buf1.len() != 0 {
    WramVerifyBuf(buf1.as_ptr(), buf2 as _, buf1.len() - 1)
  } else {
    true
  }
}

/// Reads a byte from a given memory address.
///
/// This should be used to access any data found in Flash or battery-backed
/// SRAM, as you must read those from code found in WRAM.
///
/// This uses raw addresses into the memory space. Use with care.
#[inline(always)]
pub unsafe fn read_raw_byte(src: usize) -> u8 {
  WramReadByte(src as _)
}
