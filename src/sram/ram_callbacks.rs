//! A module containing low-level assembly functions that can be loaded into
//! WRAM. Both flash and battery-backed SRAM require reads to be performed
//! via code in WRAM and cannot be accessed by DMA.

// Loads the assembly into Rust
global_asm!(include_str!("asm_read_buf.s"));
global_asm!(include_str!("asm_read_byte.s"));
extern "C" {
    fn SramReadBuf(dst: *mut u8, src: *const u8, count_16: usize, skip: usize);
    fn SramReadByte(src: *const u8) -> u8;
}

/// Utility function to help the rest of this work well.
fn as_ptr<T>(ptr: &T) -> usize {
    ptr as *const _ as _
}

/// Read data from SRAM into a buffer.
///
/// This should be used to access any data found in Flash or battery-backed
/// SRAM, as you must read those from code found in WRAM.
pub unsafe fn read_raw_buf(dst: &mut [u8], src: usize) {
    let len = dst.len();
    if len != 0 {
        let rem = len & 7;
        let dst_ptr = dst.as_mut_ptr().offset(-(rem as isize));
        let blkct = (len + 7) / 8;
        SramReadBuf(dst_ptr, (src - rem) as _, blkct, rem);
    }
}

/// Reads a byte from SRAM at a given offset.
///
/// This should be used to access any data found in Flash or battery-backed
/// SRAM, as you must read those from code found in WRAM.
#[inline(always)]
pub unsafe fn read_raw_byte(src: usize) -> u8 {
    SramReadByte(src as _)
}