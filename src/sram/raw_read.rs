//! A module containing low-level assembly functions that can be loaded into
//! WRAM. Both flash and battery-backed SRAM require reads to be performed
//! via code in WRAM and cannot be accessed by DMA.

// Loads the assembly into Rust
global_asm!(include_str!("asm_read_buf.s"));
global_asm!(include_str!("asm_read_byte.s"));
extern "C" {
    fn SramReadBuf(src: *mut u8, dst: *const u8, count_16: usize, skip: usize);
    fn SramReadByte(src: *const u8) -> u8;
}

/// Utility function to help the rest of this work well.
fn as_ptr<T>(ptr: &T) -> usize {
    ptr as *const _ as _
}

const BLOCK_SIZE: usize = 8;
unsafe fn sram_xfer_buf(src: usize, dst: usize, len: usize) {
    if len != 0 {
        let rem = len & (BLOCK_SIZE - 1);
        let blkct = (len + (BLOCK_SIZE - 1)) / BLOCK_SIZE;
        SramReadBuf((src - rem) as _, (dst - rem) as _, blkct, rem);
    }
}

/// Read data from SRAM into a buffer.
///
/// This should be used to access any data found in Flash or battery-backed
/// SRAM, as you must read those from code found in WRAM.
pub unsafe fn read_raw_buf(dst: &mut [u8], src: usize) {
    sram_xfer_buf(src, dst.as_mut_ptr() as _, dst.len())
}

/// Write data into SRAM from a buffer.
///
/// This is not strictly needed to write into SRAM, but reuses the optimized
/// loop used in `read_raw_buf`.
pub unsafe fn write_raw_buf(dst: usize, src: &[u8]) {
    sram_xfer_buf(dst, src.as_ptr() as _, src.len())
}

/// Reads a byte from SRAM at a given offset.
///
/// This should be used to access any data found in Flash or battery-backed
/// SRAM, as you must read those from code found in WRAM.
#[inline(always)]
pub unsafe fn read_raw_byte(src: usize) -> u8 {
    SramReadByte(src as _)
}
