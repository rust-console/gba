#![allow(non_snake_case)]

//! The BIOS includes several System Call Functions which can be accessed by SWI
//! instructions.
//!
//! * All BIOS functions clobber `r0`, `r1`, and `r3`.
//! * Some functions also use `r2` as an input register.
//! * All other registers are unaffected.

// Note(Lokathor): This makes intra-doc links work.
#[allow(unused)]
use crate::prelude::*;

use core::arch::asm;

/// (`swi 0x00`) Performs a "soft reset" of the device.
///
/// Loads `r14` based on the `u8` value at `0x0300_7FFA`:
/// * zero: `0x0800_0000` (ROM)
/// * non-zero: `0x0200_0000` (EWRAM)
///
/// Then resets the following memory and registers:
/// * `0x300_7E00` ..= `0x300_7FFF`: zeroed
/// * `r0` ..= `r12`: zeroed
/// * `sp_usr`: `0x300_7F00`
/// * `sp_irq`: `0x300_7FA0`
/// * `sp_svc`: `0x300_7FE0`
/// * `lr_svc`, `lr_irq` : zeroed
/// * `spsr_svc`, `spsr_irq`: zeroed
///
/// Then jumps to the `r14` address. This never returns.
pub unsafe fn SoftReset() -> ! {
  asm!("swi 0x00", options(noreturn))
}

/// (`swi 0x01`) Resets RAM and/or IO registers
///
/// * Note that if the IWRAM flag is used it doesn't reset the final `0x200`
///   bytes of IWRAM. Instead, those bytes are reset during a call to the
///   [`SoftReset`] function.
/// * BIOS Bug: Data in `SIODATA32` is always destroyed, even if the `sio` flag
///   is not set.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn RegisterRamReset(flags: crate::mmio_types::ResetFlags) {
  asm!("swi 0x01",
    inlateout("r0") flags.0 => _,
    out("r1") _,
    out("r3") _,
    options(nomem, nostack, preserves_flags)
  )
}

/// (`swi 0x02`) Halts the CPU until an interrupt request occurs.
///
/// The CPU is placed into low-power mode, while other parts (video, sound,
/// timers, serial, keypad) continue to operate. This mode only terminates when
/// one of the enabled interrupts is requested.
///
/// This halt state uses [`IE`] to determine what interrupts to allow, but the
/// [`IME`] value is ignored (interrupts can occur even if `IME` is `false`).
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn Halt() {
  asm!("swi 0x02",
    out("r0") _,
    out("r1") _,
    out("r3") _,
    options(nomem, nostack, preserves_flags)
  )
}

/// (`swi 0x03`) Puts the CPU in a *very* low power state.
///
/// While stopped, the CPU, Sound, Video, SIO-shift-clock, DMA, and Timers are
/// all disabled.
///
/// The system can return from this state only if there is an interrupt from the
/// Keypad, Game Pak, or General-Purpose-SIO.
///
/// Before calling Stop you are advised to disable the Video to reduce battery
/// usage, otherwise it just freezes.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn Stop() {
  asm!("swi 0x03",
    out("r0") _,
    out("r1") _,
    out("r3") _,
    options(nomem, nostack, preserves_flags)
  )
}

/// (`swi 0x04`) "Interrupt Wait".
///
/// This is similar to [`Halt`], but when an interrupt does occur this function
/// will automatically return the CPU to halt state unless the interrupt is one
/// of the interrupt types specified by `flags`.
///
/// If you set `discard_current_flags` then any pending interrupts are cleared
/// and this function will wait until a new flag is set. Otherwise the function
/// will return immediately if you request a wait for an interrupt that's
/// already pending.
///
/// When handling an interrupt through this function you must perform the normal
/// acknowledgement using [`IRQ_ACKNOWLEDGE`] and **also** acknowledge using
/// [`INTR_WAIT_ACKNOWLEDGE`].
///
/// **Caution:** This function automatically also sets [`IME`] to `true`.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn IntrWait(discard_current_flags: bool, flags: crate::mmio_types::InterruptFlags) {
  // Note(Lokathor): we don't mark this preserves_flags because the user's IRQ
  // handler gets called which might end up trashing the flags.
  asm!("swi 0x03",
    inlateout("r0") discard_current_flags as u8 => _,
    inlateout("r1") flags.0 => _,
    out("r3") _,
    options(nomem, nostack)
  )
}

/// (`swi 0x05`) "VBlank Interrupt Wait"
///
/// Waits for the next VBlank interrupt.
///
/// This function is just shorthand for the following:
/// ```no_run
/// # use crate::prelude::*;
/// const VBLANK_IRQ: InterruptFlags = InterruptFlags::new().with_vblank(true);
/// IntrWait(true, VBLANK_IRQ)
/// ```
/// See [`IntrWait`]
///
/// **Note:** Because this uses `IntrWait`, [`IME`] will be set to `true`
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn VBlankIntrWait() {
  // Note(Lokathor): we don't mark this preserves_flags because the user's IRQ
  // handler gets called which might end up trashing the flags.
  asm!(
    "swi 0x05",
    out("r0") _,
    out("r1") _,
    out("r3") _,
    options(nomem, nostack)
  )
}

/// (`swi 0x06`) Performs `i32` division.
///
/// **Outputs:** `(n/d, n%d, (n/d).unsigned_abs())`
#[inline]
#[must_use]
#[instruction_set(arm::t32)]
pub fn Div(number: i32, denominator: core::num::NonZeroI32) -> (i32, i32, u32) {
  let d: i32;
  let m: i32;
  let abs_d: u32;
  unsafe {
    asm!("swi 0x06",
      inlateout("r0") number => d,
      inlateout("r1") denominator.get() => m,
      lateout("r3") abs_d,
      options(pure, nomem, nostack, preserves_flags),
    )
  }
  (d, m, abs_d)
}

/// (`swi 0x08`) Square root of an integer value.
///
/// To obtain as much fraction as possible, shift the input left by 2N bits to
/// get an output that is left shifted by N bits.
/// * sqrt(2) => 0
/// * sqrt(2 << 30) => 1.41421 << 15
#[inline]
#[instruction_set(arm::t32)]
pub fn Sqrt(number: u32) -> u16 {
  let output: u32;
  unsafe {
    asm!("swi 0x08",
      inlateout("r0") number => output,
      out("r1") _,
      out("r3") _,
      options(pure, nomem, nostack, preserves_flags),
    )
  }
  output as u16
}

/// (`swi 0x09`) Arc tangent
///
/// The input and output have 14 fractional bits.
#[inline]
#[instruction_set(arm::t32)]
pub fn ArcTan(tan: i16) -> i16 {
  let output;
  unsafe {
    asm!("swi 0x09",
      inlateout("r0") tan => output,
      out("r1") _,
      out("r3") _,
      options(pure, nomem, nostack, preserves_flags),
    )
  }
  output
}

/// (`swi 0x0A`) Arc tangent 2
///
/// * The inputs have 14 fractional bits.
/// * The output range is `0 ..= u16::MAX`, reprisenting a portion of 2 PI.
#[inline]
#[instruction_set(arm::t32)]
pub fn ArcTan2(x: i16, y: i16) -> u16 {
  let output;
  unsafe {
    asm!("swi 0x0A",
      inlateout("r0") x => output,
      in("r1") y,
      out("r3") _,
      options(pure, nomem, nostack, preserves_flags),
    )
  }
  output
}

/// (`swi 0x0B`) Quickly copy/fill some memory.
///
/// * `src`: points to either `u16` or `u32` data.
/// * `dst`: points to the same type of data.
/// * `len_mode`: bitfield value:
///   * bits 0 ..= 20: the number of elements to copy/fill.
///   * bit 24: enable for fill, otherwise this is a copy.
///   * bit 26: enable for `u32` at a time, otherwise this uses `u16` at a time.
///
/// All pointers must be aligned to the appropriate type, and also valid for the
/// appropriate element count.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn CpuSet(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len_mode: u32) {
  asm!("swi 0x0B",
    in("r0") src,
    in("r1") dst,
    in("r2") len_mode,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x0C`) Quickly copy/fill some memory (most faster!)
///
/// * `src` points to the data source.
/// * `dst` points to the data destination.
/// * `len_mode`: bitfield value:
///   * bits 0 ..= 20: the number of `u32` to copy/fill.
///   * bit 24: enable for fill, otherwise this is a copy.
///
/// All pointers must be aligned. The length must be a multiple of 8.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn CpuFastSet(src: *const u32, dst: *mut u32, len_mode: u32) {
  asm!("swi 0x0C",
    in("r0") src,
    in("r1") dst,
    in("r2") len_mode,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

#[repr(C)]
pub struct BgAffineSetSrc {
  /// 8-bit fraction
  pub origin_center_x: i32,
  /// 8-bit fraction
  pub origin_center_y: i32,
  pub display_center_x: i16,
  pub display_center_y: i16,
  /// 8-bit fraction
  pub scale_ratio_x: i16,
  /// 8-bit fraction
  pub scale_ratio_y: i16,
  /// 8-bit fraction, range 0 to u16::MAX
  pub angle_of_rotation: u16,
}
#[repr(C)]
pub struct BgAffineSetDst {
  pub pa: i16,
  pub pb: i16,
  pub pc: i16,
  pub pd: i16,
  pub start_x_coordinate: i32,
  pub start_y_coordinate: i32,
}

/// (`swi 0x0E`) Calculates BG affine data.
///
/// * `src`: Points to the start of a slice of [`BgAffineSetSrc`]
/// * `dst`: Points to the start of a slice of [`BgAffineSetDst`]
/// * `count`: The number of elements to process from `src` to `dst`.
///
/// Both pointers must be aligned and valid for the length given.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn BgAffineSet(src: *const BgAffineSetSrc, dst: *mut BgAffineSetDst, count: usize) {
  asm!("swi 0x0E",
    in("r0") src,
    in("r1") dst,
    in("r2") count,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

#[repr(C)]
pub struct ObjAffineSetSrc {
  /// 8-bit fraction
  pub scale_ratio_x: i16,
  /// 8-bit fraction
  pub scale_ratio_y: i16,
  /// 8-bit fraction, range 0 to u16::MAX
  pub angle: u16,
}

/// (`swi 0x0F`) Calculates OBJ affine data.
///
/// Unlike with [`BgAffineSet`], this can optionally write the output data
/// directly into OAM (see below).
///
/// * `src`: points to the start of a slice of [`ObjAffineSetSrc`] values.
/// * `dst`: points to the start of the output location (`pa`).
/// * `count`: The number of `src` values to process to `dst`.
/// * `out_param_offset`: the number of bytes between *each field* of the output
///   data.
///   * Specify 2 if you want to output to an `[i16; 4]` or similar.
///   * Specify 8 if you want to output directly to OAM.
///
/// The pointers must be valid for the count given, and aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn ObjAffineSet(
  src: *const ObjAffineSetSrc,
  dst: *mut i16,
  count: usize,
  out_param_offset: usize,
) {
  asm!("swi 0x0F",
    in("r0") src,
    in("r1") dst,
    in("r2") count,
    in("r3") out_param_offset,
    options(nostack, preserves_flags),
  )
}

#[repr(C)]
pub struct UnpackInfo {
  pub source_data_len_bytes: u16,
  /// Supports 1, 2, 4, or 8 bit source elements.
  pub source_unit_bit_width: u8,
  /// Supports 1, 2, 4, 8, 16, or 32 destination elements.
  pub destination_unit_bit_width: u8,
  /// This field combines two purposes:
  /// * bits 0 ..= 30: this value is added to all non-zero source units.
  /// * bit 31: if this is set, add the above to all zero source units.
  pub data_offset: u32,
}

/// (`swi 0x10`) Used to undo bit packing.
///
/// * `src`: The start of the source bytes.
/// * `dst`: The start of the destination.
/// * `info`: Describes the unpacking to perform.
///
/// All pointers must be valid for the correct memory spans and aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn BitUnPack(src: *const u8, dst: *mut u32, info: &UnpackInfo) {
  asm!("swi 0x10",
    in("r0") src,
    in("r1") dst,
    in("r2") info,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x11`) LZ77 Decompression with 8-bit output.
///
/// Arguments
/// * `src`: pointer to the source region. The source region is prefixed with a
///   `u32` bitfield value that describes the decompression to perform. It's
///   then followed by the byte sequence to decompress.
///   * Prefix value: `output_data_size << 8 | (1 << 4) | (0)`
///   * Flags: 1 byte that specifies the types of the next 8 blocks (MSB to
///     LSB).
///   * Blocks:
///     * (0) Literal: Copy 1 byte from the source to the output.
///     * (1) Back Reference: Repeat `N+3` bytes from `BACK+1` bytes earlier in
///       the output. This uses the next two bytes from the source to describe
///       the back reference:
///       * first byte bits 0 ..= 3: most significant bits of `BACK`
///       * first byte bits 4 ..= 7: `N`
///       * second byte: least significant bits of `BACK`
///       * (So each `N` is 3 bits, and each `BACK` is 12 bits.)
///   * After 8 blocks there's another flag and then another 8 blocks.
///   * The overall size of the source data should be a multiple of 4 (pad with
///     0 as necessary).
/// * `dst`: pointer to the destination region.
///
/// All pointers must be valid for the correct memory spans and aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn LZ77UnCompReadNormalWrite8bit(src: *const u32, dst: *mut u8) {
  asm!("swi 0x11",
    in("r0") src,
    in("r1") dst,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x12`) LZ77 Decompression with 16-bit output.
///
/// This is largely as per [`LZ77UnCompReadNormalWrite8bit`], but each output is
/// 16-bits, which means that `BACK` values of 0 will corrupt the process. This
/// puts a small constraint on the data compressor, but doesn't really affect
/// you when you're using this function to decompress some already-compressed data.
///
/// All pointers must be valid for the correct memory spans and aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn LZ77UnCompReadNormalWrite16bit(src: *const u32, dst: *mut u16) {
  asm!("swi 0x12",
    in("r0") src,
    in("r1") dst,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x13`) Decompresses Huffman-encoded data.
///
/// * `src`: The source buffer. There's a `u32` header, a huffman tree, and then
///   a compressed bitstream.
///   * header (4 bytes): `(output_byte_count << 8) | (2 << 4) |
///     data_unit_bit_size`, the output bit size per data unit can be 4 or 8.
///   * tree size (1 byte): the number of bytes in the tree table.
///   * tree table (up to 255 bytes): a list of 8-bit nodes, starting with the
///     root node.
///     * root node and non-data child nodes (1 byte):
///       * bits 0 ..= 5: offset to next child node.
///         * next_child0: (CurrentAddr AND NOT 1)+Offset*2+2
///         * next_child1: as above +1
///       * bit 6: node1 end flag (1 = next node is data)
///       * bit 7: node0 end flag (1 = next node is data)
///     * data nodes (1 byte):
///       * the literal value to output. If the output unit size is less than 8
///         bits at a time the upper bits of the literal should be 0.
///   * compressed bitstream (stored as a series of `u32` values). The node bits
///     are stored in each `u32` starting from the high bit.
/// * `dst`: The output buffer.
///
/// All pointers must be valid for the correct memory spans and aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn HuffUnCompReadNormal(src: *const u32, dst: *mut u32) {
  asm!("swi 0x13",
    in("r0") src,
    in("r1") dst,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x14`) Expands run-length compressed data, outputting as 8-bit units.
///
/// * `src`: The source buffer. There's a `u32` header, and then a loop of
///   "flag" and then "data" bytes until the end of the stream.
///   * header (4 bytes): `(output_byte_count << 8) | (3 << 4) | 0`
///   * flag byte:
///     * bits 0 ..= 6: expanded data length, uncompressed N-1, compressed N-3.
///     * bit 7: 0=uncompressed, 1=compressed
///   * data byte: N uncompressed bytes or 1 compressed byte repeated N times.
/// * `dst`: The output buffer.
///
/// All pointers must be valid for the correct memory spans and aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn RLUnCompReadNormalWrite8bit(src: *const u32, dst: *mut u8) {
  asm!("swi 0x14",
    in("r0") src,
    in("r1") dst,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x15`) Expands run-length compressed data, outputting as 16-bit units.
///
/// This is like [`RLUnCompReadNormalWrite8bit`] but outputs in 16-bit units, so
/// it's suitable for use with VRAM.
///
/// All pointers must be valid for the correct memory spans and aligned.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn RLUnCompReadNormalWrite16bit(src: *const u32, dst: *mut u16) {
  asm!("swi 0x15",
    in("r0") src,
    in("r1") dst,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x16`) Performs an "unfilter" on 8-bit data units.
///
/// An unfiltering converts a starting value and a series of delta values into
/// the appropriate totals.
/// * Filtered: 10, +1, +1, +1, +1, +5, +5, ...
/// * Unfiltered: 10, 11, 12, 13, 14, 19, 24, ...
///
/// This is not itself a compression technique, but it's far easier to compress
/// the filtered form of data in some cases, so this is often used in
/// *combination* with other compression techniques.
///
/// Arguments
/// * `src`: pointer to the source region. The source region is prefixed with a
///   `u32` bitfield value that describes the unfiltering to perform. It's then
///   followed by the bytes to unfilter.
///     * Prefix value: `element_count << 8 | (8 << 4) | (1)`
/// * `dst`: pointer to the destination region.
///
/// Note that, because this uses 8-bit writes, it cannot output correctly to
/// VRAM.
///
/// The source pointer must be aligned to 4 (the header is read as a `u32`), and
/// both pointers must be valid for the correct span:
/// * `src`: `element_count` + 4 bytes
/// * `dst`: `element_count` bytes
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn Diff8bitUnFilterWrite8bit(src: *const u8, dst: *mut u32) {
  asm!("swi 0x16",
    in("r0") src,
    in("r1") dst,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x17`) Performs an "unfilter" on 8-bit data units, using 16-bit
/// output.
///
/// This is *very close* to [`Diff8bitUnFilterWrite8bit`] except that the output
/// is 16-bits per element.
///
/// Arguments
/// * `src`: pointer to the source region. The source region is prefixed with a
///   `u32` bitfield value that describes the unfiltering to perform. It's then
///   followed by the bytes to unfilter.
///     * Prefix value: `element_count << 8 | (8 << 4) | (1)`
/// * `dst`: pointer to the destination region.
///
/// Because this outputs with 16-bit writes, it is suitable for use with VRAM.
///
/// The source pointer must be aligned to 4 (the header is read as a `u32`), and
/// both pointers must be valid for the correct span:
/// * `src`: `element_count` + 4 bytes
/// * `dst`: `element_count` * 2 bytes
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn Diff8bitUnFilterWrite16bit(src: *const u8, dst: *mut u16) {
  asm!("swi 0x17",
    in("r0") src,
    in("r1") dst,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

/// (`swi 0x18`) Performs an "unfilter" on 16-bit data units.
///
/// This is *very close* to [`Diff8bitUnFilterWrite8bit`] except that the output
/// is 16-bits per element and the prefix is different.
///
/// Arguments
/// * `src`: pointer to the source region. The source region is prefixed with a
///   `u32` bitfield value that describes the unfiltering to perform. It's then
///   followed by the bytes to unfilter.
///     * Prefix value: `element_count << 8 | (8 << 4) | (2)`
/// * `dst`: pointer to the destination region.
///
/// Because this outputs with 16-bit writes, it is suitable for use with VRAM.
///
/// The source pointer must be aligned to 4 (the header is read as a `u32`), and
/// both pointers must be valid for the correct span:
/// * `src`: (`element_count` * 2) + 4 bytes
/// * `dst`: `element_count` * 2 bytes
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn Diff16bitUnFilter(src: *const u16, dst: *mut u16) {
  asm!("swi 0x18",
    in("r0") src,
    in("r1") dst,
    out("r3") _,
    options(nostack, preserves_flags),
  )
}

// TODO: MidiKey2Freq (1F)

// TODO: SoundBias (19)

// TODO: SoundChannelClear (1E)

// TODO: SoundDriverInit (1A)

// TODO: SoundDriverMain (1C)

// TODO: SoundDriverMode (1B)

// TODO: SoundDriverVSync (1D)

// TODO: MultiBoot (25)

// TODO: SoundDriverVSyncOff (28)

// TODO: SoundDriverVSyncOn (29)
