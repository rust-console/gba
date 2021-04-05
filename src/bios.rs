#![allow(non_snake_case)]

//! The BIOS includes several System Call Functions which can be accessed by SWI
//! instructions.
//!
//! * Incoming parameters are usually passed through registers R0,R1,R2,R3.
//! * Outgoing registers R0,R1,R3 are typically containing either garbage, or
//!   return value(s).
//! * All other registers (R2,R4-R14) are kept unchanged.

// TODO: SoftReset

// TODO: RegisterRamReset

#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn Halt() {
  asm!("swi 0x02", options(nomem, nostack))
}

#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn Stop() {
  asm!("swi 0x03", options(nomem, nostack))
}

// TODO: IntrWait (requires interrupt flags mmio type)

#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn VBlankIntrWait() {
  asm!(
    "swi 0x05",
    out("r0") _,
    out("r1") _,
    options(nomem, nostack)
  )
}

/// Performs `i32` division.
///
/// **Outputs:** (n / d, n % d, (n / d).unsigned_abs())
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
      options(pure, nomem, nostack),
    )
  }
  (d, m, abs_d)
}

/// Square root of an integer value.
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
      // out("r1") _,
      // out("r3") _,
      options(pure, nomem, nostack),
    )
  }
  output as u16
}

/// arc tangent
///
/// * 14 fractional bits
///
/// TODO: doc and check
#[inline]
#[instruction_set(arm::t32)]
pub fn ArcTan(tan: i16) -> i16 {
  let output;
  unsafe {
    asm!("swi 0x09",
      inlateout("r0") tan => output,
      // out("r1") _,
      // out("r3") _,
      options(pure, nomem, nostack),
    )
  }
  output
}

/// arc tangent 2
///
/// * input: 14 fractional bits
/// * output: 0 to 2pi
///
/// TODO: doc and check
#[inline]
#[instruction_set(arm::t32)]
pub fn ArcTan2(x: i16, y: i16) -> u16 {
  let output;
  unsafe {
    asm!("swi 0x0A",
      inlateout("r0") x => output,
      in("r1") y,
      // out("r3") _,
      options(pure, nomem, nostack),
    )
  }
  output
}

/// Quickly copy/fill.
///
/// The pointers can point to `u16` or `u32`, but must have matching types and
/// be aligned appropriately.
///
/// * `len_mode`: serves a dual purpose:
///   * bits 0 ..= 20: The number of *elements* to set. This must be a multiple
///     of 8.
///   * bit 24: "fixed source address" flag. If this is set all destination
///     elements become the single value at the `src` address. Otherwise the
///     source pointer is treated as the start pointer to a memory slice to
///     copy.
///   * bit 26: use `u32` elements (otherwise this uses `u16`)
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn CpuSet(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len_mode: u32) {
  asm!("swi 0x0B",
    in("r0") src,
    in("r1") dst,
    in("r2") len_mode,
    // out("r3") _,
    options(nostack),
  )
}

/// Quickly copy/fill in large chunks.
///
/// * `len_mode`: serves a dual purpose:
///   * bits 0 ..= 20: The number of words to set. This must be a multiple of 8.
///   * bit 24: "fixed source address" flag. If this is set all destination
///     words become the single value at the `src` address. Otherwise the source
///     pointer is treated as the start pointer to a memory slice to copy.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn CpuFastSet(src: *const u32, dst: &mut [u32], len_mode: u32) {
  asm!("swi 0x0C",
    in("r0") src,
    in("r1") dst.as_mut_ptr(),
    in("r2") len_mode,
    // out("r3") _,
    options(nostack),
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
  pub diff_x_same_line: i16,
  pub diff_x_next_line: i16,
  pub diff_y_same_line: i16,
  pub diff_y_next_line: i16,
  pub start_x_coordinate: i32,
  pub start_y_coordinate: i32,
}

/// Used to calculate background affine parameters.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn BgAffineSet(src: &[BgAffineSetSrc], dst: &mut [BgAffineSetDst], count: usize) {
  asm!("swi 0x0E",
    in("r0") src.as_ptr(),
    in("r1") dst.as_mut_ptr(),
    in("r2") count,
    // out("r3") _,
    options(nostack),
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

/// Used to calculate OBJ affine parameters.
///
/// * outputs are affine parameter sets (four `i16` each).
/// * `out_param_offset`: the number of bytes between each output field. Use 2
///   for affine data that's continuous (`[i16; 4]`), or with 8 you can output
///   the data directly to OAM.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn ObjAffineSet(
  src: &[ObjAffineSetSrc],
  dst: *mut i16,
  count: usize,
  out_param_offset: usize,
) {
  asm!("swi 0x0F",
    in("r0") src.as_ptr(),
    in("r1") dst,
    in("r2") count,
    in("r3") out_param_offset,
    options(nostack),
  )
}

#[repr(C)]
pub struct UnpackInfo {
  pub source_data_len_bytes: u16,
  /// Supports 1, 2, 4, or 8
  pub source_unit_bit_width: u8,
  /// Supports 1, 2, 4, 8, 16, or 32
  pub destination_unit_bit_width: u8,
  /// This field combines two purposes:
  /// * bits 0 ..= 30: this value is added to all non-zero source units.
  /// * bit 31: if this is set, add the above to all zero source units.
  pub data_offset: u32,
}

/// Used to increase the color depth of bitmap data.
///
/// * The final size of the unpacked data must be a multiple of 4 bytes, and
///   must not overflow `dst`.
/// * The bit width of the source units, plus the offset, should not exceed the
///   bit width of the destination.
/// * Destination can be WRAM or VRAM.
#[inline]
#[instruction_set(arm::t32)]
pub unsafe fn BitUnPack(src: &[u8], dst: &mut [u32], info: &UnpackInfo) {
  asm!("swi 0x10",
    in("r0") src.as_ptr(),
    in("r1") dst.as_mut_ptr(),
    in("r2") info as *const UnpackInfo,
    // out("r3") _,
    options(nostack),
  )
}

// TODO: Diff8bitUnFilterWrite8bit

// TODO: Diff8bitUnFilterWrite16bit

// TODO: Diff16bitUnFilter

// TODO: HuffUnCompReadNormal

// TODO: LZ77UnCompReadNormalWrite8bit

// TODO: LZ77UnCompReadNormalWrite16bit

// TODO: RLUnCompReadNormalWrite8bit

// TODO: RLUnCompReadNormalWrite16bit

// TODO: MultiBoot

// TODO: MidiKey2Freq

// TODO: SoundBias

// TODO: SoundChannelClear

// TODO: SoundDriverInit

// TODO: SoundDriverMain

// TODO: SoundDriverMode

// TODO: SoundDriverVSync

// TODO: SoundDriverVSyncOff

// TODO: SoundDriverVSyncOn

// TODO: SoundWhatever0

// TODO: SoundWhatever1

// TODO: SoundWhatever2

// TODO: SoundWhatever3

// TODO: SoundWhatever4
