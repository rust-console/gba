#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

//! The GBA's BIOS provides limited built-in utility functions.
//!
//! BIOS functions are accessed with an `swi` instruction to perform a software
//! interrupt. This means that there's a *significant* overhead for a BIOS call
//! (tens of cycles) compared to a normal function call (3 cycles, or even none
//! of the function ends up inlined). Despite this higher cost, some bios
//! functions are useful enough to justify the overhead.

use crate::{fixed::i16fx14, interrupts::IrqBits};

// Note(Lokathor): All `swi` calls will preserve the flags. You should generally
// not use any other inline-asm options with `swi` calls.

/// `0x00`: Software Reset.
///
/// This clears the BIOS portion of IWRAM (the top `0x200` bytes), resets the
/// SVC, IRQ, and SYS stack pointers to their defaults, then performs a `bx r14`
/// to go to an address based on what's written to the byte at `0x0300_7FFA`:
/// * zero: `0x0800_0000` (ROM)
/// * non-zero: `0x0200_0000` (IWRAM).
///
/// (Note: the target address is determined *before* clearing the top of IWRAM.)
#[inline]
#[instruction_set(arm::t32)]
pub fn SoftReset() -> ! {
  unsafe {
    core::arch::asm! {
      "swi #0x00",
      options(noreturn),
    }
  };
}

/// `0x04`: Waits for a specific interrupt type(s) to happen.
///
/// Pauses the CPU until any of the interrupt types set in `target_irqs` to
/// occur. This can create a significant savings of the battery while you're
/// waiting, so use this function when possible.
///
/// **Important:** This function forces [`IME`](crate::mmio::IME) on.
///
/// Your interrupt handler (if any) will be run before this function returns.
///
/// If none of the interrupts specified in `target_irqs` are properly configured
/// to fire then this function will loop forever without returning.
///
/// This function uses a special BIOS variable to track what interrupts have
/// occured recently.
/// * If `ignore_existing` is set, then any previous interrupts (since
///   `IntrWait` was last called) that match `target_irqs` are *ignored* and
///   this function will wait for a new target interrupt to occur.
/// * Otherwise, any previous interrupts that match `target_irqs` will cause the
///   function to return immediately without waiting for a new interrupt.
#[inline]
#[instruction_set(arm::t32)]
pub fn IntrWait(ignore_existing: bool, target_irqs: IrqBits) {
  unsafe {
    core::arch::asm! {
      "swi #0x04",
      inout("r0") ignore_existing as u32 => _,
      inout("r1") target_irqs.to_u16() => _,
      out("r3") _,
      options(preserves_flags),
    }
  };
}

/// `0x05`: Builtin shorthand for [`IntrWait(true, IrqBits::VBLANK)`](IntrWait)
#[inline]
#[instruction_set(arm::t32)]
pub fn VBlankIntrWait() {
  unsafe {
    core::arch::asm! {
      "swi #0x05",
      out("r0") _,
      out("r1") _,
      out("r3") _,
      options(preserves_flags),
    }
  };
}

/// `0x09`: Arc tangent.
///
/// * **Returns:** The output is in the range +/- `pi/2`, but accuracy is worse
///   outside of +/- `pi/4`.
#[inline]
#[instruction_set(arm::t32)]
pub fn ArcTan(theta: i16fx14) -> i16fx14 {
  let mut i = theta.into_raw();
  unsafe {
    core::arch::asm! {
      "swi #0x09",
      inout("r0") i,
      out("r1") _,
      out("r3") _,
      options(pure, nomem, preserves_flags),
    }
  };
  i16fx14::from_raw(i)
}

/// `0x0A`: The "2-argument arctangent" ([atan2][wp-atan2]).
///
/// [wp-atan2]: https://en.wikipedia.org/wiki/Atan2
///
/// * **Returns:** The angle of the input vector, with `u16::MAX` being
///   equivalent to `2pi`.
#[inline]
#[instruction_set(arm::t32)]
pub fn ArcTan2(x: i16fx14, y: i16fx14) -> u16 {
  let x = x.into_raw();
  let y = y.into_raw();
  let output: u16;
  unsafe {
    core::arch::asm! {
      "swi #0x0A",
      inout("r0") x => output,
      inout("r1") y => _,
      out("r3") _,
      options(pure, nomem, preserves_flags),
    }
  };
  output
}

/// Used to provide info to a call of the [`BitUnPack`] function.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BitUnpackInfo {
  /// Number of bytes in the source buffer
  pub src_byte_len: u16,
  /// Bits per source element: 1, 2, 4, or 8.
  pub src_elem_width: u8,
  /// Bits per destination element: 1, 2, 4, 8, 16, or 32.
  pub dest_elem_width: u8,
  /// Bits `0..=30` are the offset value added to all non-zero elements.
  ///
  /// If bit `31` is set then offset value is *also* added to zero elements.
  pub offset_and_touch_zero: u32,
}

/// `0x10`: Copy data from `src` to `dest` while increasing the bit depth of the
/// elements copied.
///
/// * This reads one byte at a time from `src`. Each source byte holds 1 or more
///   source elements, depending on the source bit depth you specify. Elements
///   within a byte are packed from low bit to high bit.
/// * Each non-zero source element has the offset added to it. If the source
///   element is zero and the "touch zero" flag is set, then that source element
///   will also have the offset added to it. This creates a destination element.
/// * Destination elements are collected into the output `u32` buffer one at a
///   time, from low bit to high bit. If a source element plus the offset
///   produces a value larger than the destination element bit size this will
///   corrupt any following destination elements within the buffer. When the
///   buffer has 32 bits held then it's written to the destination pointer.
/// * When the source byte read has no more source elements remaining the source
///   pointer will advance and `src_byte_len` will go down by 1. When
///   `src_byte_len` goes to 0 the function's main loop will break and return.
///   If there was partial output in the `u32` buffer when the function's
///   primary loop ends this data will be lost.
///
/// ## Safety
/// * The `info` provided must correctly describe the data.
/// * `src` must be readable for the number of **bytes** specified
/// * `dest` must be writable for the number of **words** that the source
///   buffer, source depth, and destination depth will total up to.
/// * `dest` must be 4 byte aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn BitUnPack(src: *const u8, dest: *mut u32, info: &BitUnpackInfo) {
  core::arch::asm! {
    "swi #0x10",
    inout("r0") src => _,
    inout("r1") dest => _,
    inout("r2") info => _,
    out("r3") _,
    options(preserves_flags),
  }
}

/// `0x11`: Decompress LZ77 data from `src` to `dest` using 8-bit writes.
///
/// * The `src` is the LZ77 header and data, and must start aligned to 4.
/// * The `dest` pointer is written 8 bits at a time, meaning that this function
///   is **not** VRAM compatible.
///
/// ## The GBA's LZ77 Format
/// * Data header (32bit)
///   * Bit 0-7: Magic number `0b0001_0000`
///   * Bit 8-31: Byte count of *decompressed* data
///
/// Repeat below. Each Flag Byte followed by eight Blocks.
/// * Flag data (8bit)
///   * Bit 0-7: block type bits for the next 8 blocks, MSB first.
/// * Block Type 0 (Uncompressed): Literal byte.
///   * Bit 0-7: one byte to copy directly to the output
/// * Block Type 1 (Compressed): Repeated sequence. Copies `N+3` bytes from
///   `dest-delta-1` back in the output sequence to `dest`. The GBATEK docs call
///   the delta value "disp", presumably for "displacement".
///   * Bit 0-3: high 4 bits of `delta`
///   * Bit 4-7: `N`
///   * Bit 8-15: low 8 bits of `delta`
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn LZ77UnCompReadNormalWrite8bit(src: *const u8, dest: *mut u8) {
  core::arch::asm! {
    "swi #0x11",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  }
}

/// `0x12`: Decompress LZ77 data from `src` to `dest` using 16-bit writes.
///
/// * The `src` is the LZ77 header and data, and must start aligned to 4.
/// * The `dest` pointer is written 16 bits at a time, so it must have align 2.
///
/// See [`LZ77UnCompReadNormalWrite16bit`] for a description of the LZ77 format
/// used.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn LZ77UnCompReadNormalWrite16bit(src: *const u8, dest: *mut u16) {
  core::arch::asm! {
    "swi #0x12",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  }
}

/// `0x13`: Decompress huffman encoded data.
///
/// * `src` points to the header and data (must be aligned to 4).
/// * `dest` points to the output buffer (must be aligned to 4).
///
/// ## Data Format
///
/// * `header` (32bit)
///   * Bits 0-3: bits per data element (normally 4 or 8).
///   * Bits 4-7: must be 2
///   * Bits 8-31: size of decompressed data in *bytes*
/// * `tree_size` (8bit)
///   * Bits 0-7: `tree_table/2 - 1` (aka the offset to `compressed_bitstream`)
/// * `tree_table` (list of 8bit nodes, starting with the root node)
///   * Root Node and Non-Data-Child Nodes are:
///     * Bits 0-5: Offset to next child node.
///       * Next child node0 is at `(CurrentAddr AND NOT 1)+Offset*2+2`
///       * Next child node1 is at `(CurrentAddr AND NOT 1)+Offset*2+2+1`
///     * Bit 6: Node1 End Flag (1=Next child node is data)
///     * Bit 7: Node0 End Flag (1=Next child node is data)
///   * Data nodes are (when End Flag was set in parent node):
///     * Bits 0-7: Data element (upper bits should be zero when elements are
///       less than 8 bits)
/// * `compressed_bitstream` (stored in units of 32bits)
///   * Bit 0-31: Node Bits (high bit to low bit)  (0=Node0, 1=Node1)
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn HuffUnCompReadNormal(src: *const u8, dest: *mut u32) {
  core::arch::asm! {
    "swi #0x13",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  }
}

/// `0x14`: Decompress run-length encoded data (8-bit writes).
///
/// * `src` points to the header and data (must be aligned to 4).
/// * `dest` points to the output buffer.
///
/// ## Data Format
/// * `header` (32bit)
///   * Bits 0-7: magic number `0b0011_0000`
///   * Bit: 8-31:  Size of decompressed data in *bytes*
/// * Repeat below. Each Flag Byte followed by one or more Data Bytes.
///   * Flag data (8bit)
///     * Bits 0-6: Expanded Data Length (uncompressed N-1, compressed N-3)
///     * Bit 7: Flag (0=uncompressed, 1=compressed)
///   * Data Byte(s): N uncompressed bytes, or 1 byte repeated N times
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn RLUnCompReadNormalWrite8bit(src: *const u8, dest: *mut u8) {
  core::arch::asm! {
    "swi #0x14",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  }
}

/// `0x15`: Decompress run-length encoded data (16-bit writes).
///
/// * `src` points to the header and data (must be aligned to 4).
/// * `dest` points to the output buffer.
///
/// ## Data Format
/// * See [`RLUnCompReadNormalWrite8bit`]
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn RLUnCompReadNormalWrite16bit(src: *const u8, dest: *mut u16) {
  core::arch::asm! {
    "swi #0x15",
    inout("r0") src => _,
    inout("r1") dest => _,
    out("r3") _,
    options(preserves_flags),
  }
}
