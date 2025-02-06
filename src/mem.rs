#![cfg_attr(not(feature = "on_gba"), allow(unused_variables))]

use crate::macros::on_gba_or_unimplemented;

/// Copies `u8` at a time between exclusive regions.
///
/// * This will *always* copy one byte at a time, and the code is always stored
///   in IWRAM, making it suitable for use with SRAM memory.
///
/// ## Safety
/// * As with all copying routines, the source must be readable for the size you
///   specify, and the destination must be writable for the size you specify.
/// * The regions must not overlap.
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(feature = "on_gba", link_section = ".iwram.copy_u8_unchecked")]
pub unsafe extern "C" fn copy_u8_unchecked(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  on_gba_or_unimplemented!(unsafe {
    // Note(Lokathor): This loop setup assumes that the `byte_count` is usually
    // greater than 0, and so subtracts first and then does a conditional
    // load/store pair if the value (after subtracting) is greater than or equal
    // to 0 (meaning that the value before the subtract *was* 1 or more).
    core::arch::asm! {
      "1:",
      "subs    {count}, {count}, #1",
      "ldrbge  {temp}, [{src}], #1",
      "strbge  {temp}, [{dest}], #1",
      "bgt     1b",
      temp = out(reg) _,
      count = inout(reg) byte_count => _,
      src = inout(reg) src => _,
      dest = inout(reg) dest => _,
      options(nostack)
    }
  });
}

/// Copies `[u32; 8]` sized chunks, to `dest` from `src`
///
/// This will, in general, be slightly faster than a generic `memcpy`, but
/// slightly slower than using DMA.
///
/// Particularly, this helps with:
/// * [`Tile4`][crate::video::Tile4] (one loop per tile).
/// * [`Tile8`][crate::video::Tile8] (two loops per tile).
/// * A palbank of [`Color`][crate::video::Color] values (one loop per palbank).
/// * A text mode screenblock (64 loops per screenblock).
/// * A Mode 3 bitmap (2400 loops).
/// * A Mode 4 bitmap (1200 loops).
///
/// ## Safety
/// * As with all copying routines, the source must be readable for the size you
///   specify, and the destination must be writable for the size you specify.
/// * Both pointers must be aligned to 4.
/// * The regions must not overlap.
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(feature = "on_gba", link_section = ".iwram.copy_u32x8_unchecked")]
pub unsafe extern "C" fn copy_u32x8_unchecked(
  dest: *mut [u32; 8], src: *const [u32; 8], count: usize,
) {
  on_gba_or_unimplemented!(unsafe {
    // Note(Lokathor): Same loop logic as `copy_u8_unchecked`, we're just
    // processing bigger chunks of data at a time.
    core::arch::asm!(
      "1:",
      "subs  {count}, {count}, #1",
      "ldmge {src}!, {{r3,r4,r5,r7, r8,r9,r12,lr}}",
      "stmge {dest}!, {{r3,r4,r5,r7, r8,r9,r12,lr}}",
      "bgt   1b",

      // Note(Lokathor): LLVM will always put `lr` on the stack as part of the
      // push/pop for the function, even if we don't use `lr`, so we might as
      // well use `lr`, because if we use a different register (such as `r10`)
      // that would only add to the amount of push/pop LLVM does.
      count = inout(reg) count => _,
      dest = inout(reg) dest => _,
      src = inout(reg) src => _,
      out("r3") _,
      out("r4") _,
      out("r5") _,
      out("r7") _,
      out("r8") _,
      out("r9") _,
      out("r12") _,
      out("lr") _,
      options(nostack)
    )
  });
}

/// Sets `word` in blocks of 80 per loop.
///
/// This is intended for clearing VRAM to a particular color when using
/// background modes 3, 4, and 5.
/// * To clear the Mode 3 bitmap, pass `240` as the count.
/// * To clear a Mode 4 frame pass `120`.
/// * To clear a Mode 5 frame pass `128`.
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(feature = "on_gba", link_section = ".iwram.set_u32x80_unchecked")]
pub unsafe extern "C" fn set_u32x80_unchecked(
  dest: *mut [u32; 80], word: u32, count: usize,
) {
  on_gba_or_unimplemented!(unsafe {
    core::arch::asm!(
      // Note(Lokathor): Same loop logic as `copy_u8_unchecked`, we're just
      // processing bigger chunks of data at a time, and also setting rather
      // than copying.
      "1:",
      "subs {count}, {count}, #1",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "stmge  {dest}!, {{r1,r3,r4,r5, r7,r8,r12,lr}}",
      "bgt   1b",

      // The assembler will give us a warning (that we can't easily disable)
      // if the reg_list for `stm` doesn't give the registers in order from
      // low to high, so we just manually pick registers. The count register
      // and the pointer register can be anything else.
      in("r1") word,
      in("r3") word,
      in("r4") word,
      in("r5") word,
      in("r7") word,
      in("r8") word,
      in("r12") word,
      in("lr") word,
      dest = inout(reg) dest => _,
      count = inout(reg) count => _,
      options(nostack),
    )
  });
}

#[cfg(feature = "aeabi_mem_fns")]
pub use aeabi_mem_fns::*;
#[cfg(feature = "aeabi_mem_fns")]
mod aeabi_mem_fns {
  //! Module for direct memory operations.
  //!
  //! Generally you don't need to call these yourself. Instead, the compiler
  //! will insert calls to the functions defined here as necessary.

  use core::ffi::c_void;

  /// Byte copy between exclusive regions.
  ///
  /// * This will *always* copy one byte at a time, making it suitable for use
  ///   with SRAM memory.
  ///
  /// ## Safety
  /// * If `byte_count` is zero then the pointers are not used and they can be
  ///   any value.
  /// * If `byte_count` is non-zero then:
  ///   * Both pointers must be valid for the number of bytes given.
  ///   * The two regions must either be *entirely* disjoint or *entirely*
  ///     overlapping. Partial overlap is not allowed.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memcpy1"]
  pub unsafe extern "C" fn __aeabi_memcpy1(
    dest: *mut u8, src: *const u8, byte_count: usize,
  ) {
    core::arch::asm! {
      "1:",
      "subs    {count}, {count}, #1",
      "ldrbge  {temp}, [{src}], #1",
      "strbge  {temp}, [{dest}], #1",
      "bgt     1b",
      temp = out(reg) _,
      count = inout(reg) byte_count => _,
      src = inout(reg) src => _,
      dest = inout(reg) dest => _,
      options(nostack)
    }
  }

  /// Halfword copy between exclusive regions.
  ///
  /// * If the `byte_count` is odd then a single byte copy will happen at the
  ///   end.
  ///
  /// ## Safety
  /// * If `byte_count` is zero then the pointers are not used and they can be
  ///   any value.
  /// * If `byte_count` is non-zero then:
  ///   * Both pointers must be valid for the span used and aligned to 2.
  ///   * The two regions must either be *entirely* disjoint or *entirely*
  ///     overlapping. Partial overlap is not allowed.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memcpy2"]
  pub unsafe extern "C" fn __aeabi_memcpy2(
    mut dest: *mut u16, mut src: *const u16, mut byte_count: usize,
  ) {
    core::arch::asm! {
      "1:",
      "subs    {count}, {count}, #2",
      "ldrhge  {temp}, [{src}], #2",
      "strhge  {temp}, [{dest}], #2",
      "bgt     1b",
      temp = out(reg) _,
      count = inout(reg) byte_count,
      src = inout(reg) src,
      dest = inout(reg) dest,
      options(nostack)
    }
    if byte_count != 0 {
      let dest = dest.cast::<u8>();
      let src = src.cast::<u8>();
      dest.write_volatile(src.read_volatile());
    }
  }

  /// Word copy between exclusive regions.
  ///
  /// * If `byte_count` is not a multiple of 4 then a halfword and/or byte copy
  ///   will happen at the end.
  ///
  /// ## Safety
  /// * If `byte_count` is zero then the pointers are not used and they can be
  ///   any value.
  /// * If `byte_count` is non-zero then:
  ///   * Both pointers must be valid for the span used and aligned to 4.
  ///   * The two regions must either be *entirely* disjoint or *entirely*
  ///     overlapping. Partial overlap is not allowed.
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memcpy4"]
  pub unsafe extern "C" fn __aeabi_memcpy4(
    dest: *mut u32, src: *const u32, byte_count: usize,
  ) {
    core::arch::naked_asm! {
      bracer::when!(("r2" >=u "#32") [2] {
        "push {{r4-r9}}",
        "1:",
        "subs   r2, r2, #32",
        "ldmge  r1!, {{r3-r9, r12}}",
        "stmge  r0!, {{r3-r9, r12}}",
        "bgt    1b",
        "pop {{r4-r9}}",
        "bxeq   lr",
      }),

      // copy 4 words, two at a time
      "tst    r2, #0b10000",
      "ldmne  r1!, {{r3, r12}}",
      "stmne  r0!, {{r3, r12}}",
      "ldmne  r1!, {{r3, r12}}",
      "stmne  r0!, {{r3, r12}}",
      "bics   r2, r2, #0b10000",
      "bxeq   lr",

      // copy 2 and/or 1 words
      "lsls   r3, r2, #29",
      "ldmcs  r1!, {{r3, r12}}",
      "stmcs  r0!, {{r3, r12}}",
      "ldrmi  r3, [r1], #4",
      "strmi  r3, [r0], #4",
      "bics   r2, r2, #0b1100",
      "bxeq   lr",

      // copy halfword and/or byte
      "lsls   r3, r2, #31",
      "ldrhcs r3, [r1], #2",
      "strhcs r3, [r0], #2",
      "ldrbmi r3, [r1], #1",
      "strbmi r3, [r0], #1",
      "bx     lr",
    }
  }

  /// Just call [`__aeabi_memcpy4`] instead.
  ///
  /// This function is provided only for API completeness, because in some cases
  /// the compiler might automatically generate a call to this function.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memcpy8"]
  pub unsafe extern "C" fn __aeabi_memcpy8(
    dest: *mut u32, src: *const u32, byte_count: usize,
  ) {
    __aeabi_memcpy4(dest, src, byte_count);
  }

  /// Arbitrary-width copy between exclusive regions.
  ///
  /// ## Safety
  /// * If `byte_count` is zero then the pointers are not used and they can be
  ///   any value.
  /// * If `byte_count` is non-zero then:
  ///   * Both pointers must be valid for the span used (no required alignment).
  ///   * The two regions must either be *entirely* disjoint or *entirely*
  ///     overlapping. Partial overlap is not allowed.
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memcpy"]
  pub unsafe extern "C" fn __aeabi_memcpy(
    dest: *mut u8, src: *const u8, byte_count: usize,
  ) {
    core::arch::naked_asm! {
      "cmp    r2, #7", // if count <= (fix+word): just byte copy
      "ble    {__aeabi_memcpy1}",

      // check max coalign
      "eor    r3, r0, r1",
      "lsls   r3, r3, #31",
      "bmi    {__aeabi_memcpy1}",
      "bcs    2f",

      // max coalign4, possible fixup and jump
      "lsls   r3, r0, #31",
      "submi  r2, r2, #1",
      "ldrbmi r3, [r1], #1",
      "strbmi r3, [r0], #1",
      "subcs  r2, r2, #2",
      "ldrhcs r3, [r1], #2",
      "strhcs r3, [r0], #2",
      "b      {__aeabi_memcpy4}",

      // max coalign2, possible fixup and jump
      "2:",
      "lsls   r3, r0, #31",
      "submi  r2, r2, #1",
      "ldrbmi r3, [r1], #1",
      "strbmi r3, [r0], #1",
      "b      {__aeabi_memcpy2}",

      //
      __aeabi_memcpy4 = sym __aeabi_memcpy4,
      __aeabi_memcpy2 = sym __aeabi_memcpy2,
      __aeabi_memcpy1 = sym __aeabi_memcpy1,
    }
  }

  /// Copy between exclusive regions, prefer [`__aeabi_memcpy`] if possible.
  ///
  /// This is the libc version of a memory copy. It's required to return the
  /// `dest` pointer at the end of the call, which makes it need an extra
  /// push/pop compared to a direct call to `__aeabi_memcpy`.
  ///
  /// * **Returns:** The `dest` pointer.
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.memcpy"]
  pub unsafe extern "C" fn memcpy(
    dest: *mut u8, src: *const u8, byte_count: usize,
  ) -> *mut u8 {
    // I've seen a standard call to `__aeabi_memcpy` give weird codegen,
    // so we (currently) do the call manually.
    core::arch::naked_asm! {
      "push {{r0, lr}}",
      "bl {__aeabi_memcpy}",
      "pop {{r0, lr}}",
      "bx lr",
      __aeabi_memcpy = sym __aeabi_memcpy,
    }
  }

  // MOVE

  // used by `__aeabi_memmove` in some cases
  #[inline]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.reverse_copy_u8"]
  unsafe extern "C" fn reverse_copy_u8(
    dest: *mut u8, src: *const u8, byte_count: usize,
  ) {
    core::arch::asm! {
      "1:",
      "subs    {count}, {count}, #1",
      "ldrbge  {temp}, [{src}, #-1]!",
      "strbge  {temp}, [{dest}, #-1]!",
      "bgt     1b",
      temp = out(reg) _,
      count = inout(reg) byte_count => _,
      src = inout(reg) src => _,
      dest = inout(reg) dest => _,
      options(nostack)
    }
  }

  // used by `__aeabi_memmove` in some cases
  #[inline]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.reverse_copy_u16"]
  unsafe extern "C" fn reverse_copy_u16(
    mut dest: *mut u16, mut src: *const u16, mut byte_count: usize,
  ) {
    core::arch::asm! {
      "1:",
      "subs    {count}, {count}, #2",
      "ldrhge  {temp}, [{src}, #-2]!",
      "strhge  {temp}, [{dest}, #-2]!",
      "bgt     1b",
      temp = out(reg) _,
      count = inout(reg) byte_count,
      src = inout(reg) src,
      dest = inout(reg) dest,
      options(nostack)
    }
    if byte_count != 0 {
      let dest = dest.cast::<u8>().sub(1);
      let src = src.cast::<u8>().sub(1);
      dest.write_volatile(src.read_volatile());
    }
  }

  // used by `__aeabi_memmove` in some cases
  #[naked]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.reverse_copy_u32"]
  unsafe extern "C" fn reverse_copy_u32(
    dest: *mut u32, src: *const u32, byte_count: usize,
  ) {
    core::arch::naked_asm! {
      bracer::when!(("r2" >=u "#32") [2] {
        "push {{r4-r9}}",
        "1:",
        "subs    r2, r2, #32",
        "ldmdbcs r1!, {{r3-r9, r12}}",
        "stmdbcs r0!, {{r3-r9, r12}}",
        "bgt     1b",
        "pop {{r4-r9}}",
        "bxeq   lr",
      }),

      // copy 4 words, two at a time
      "tst     r2, #0b10000",
      "ldmdbne r1!, {{r3, r12}}",
      "stmdbne r0!, {{r3, r12}}",
      "ldmdbne r1!, {{r3, r12}}",
      "stmdbne r0!, {{r3, r12}}",
      "bics    r2, r2, #0b10000",
      "bxeq    lr",

      // copy 2 and/or 1 words
      "lsls    r3, r2, #29",
      "ldmdbcs r1!, {{r3, r12}}",
      "stmdbcs r0!, {{r3, r12}}",
      "ldrmi   r3, [r1, #-4]!",
      "strmi   r3, [r0, #-4]!",
      "bxeq    lr",

      // copy halfword and/or byte
      "lsls    r2, r2, #31",
      "ldrhcs  r3, [r1, #-2]!",
      "strhcs  r3, [r0, #-2]!",
      "ldrbmi  r3, [r1, #-1]!",
      "strbmi  r3, [r0, #-1]!",
      "bx      lr",
    }
  }

  /// Copy between non-exclusive regions, prefer [`__aeabi_memmove`] if
  /// possible.
  ///
  /// This function is provided only for API completeness, because in some cases
  /// the compiler might automatically generate a call to this function.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memmove4"]
  pub unsafe extern "C" fn __aeabi_memmove4(
    dest: *mut u32, src: *const u32, byte_count: usize,
  ) {
    __aeabi_memmove(dest.cast(), src.cast(), byte_count)
  }

  /// Copy between non-exclusive regions, prefer [`__aeabi_memmove`] if
  /// possible.
  ///
  /// This function is provided only for API completeness, because in some cases
  /// the compiler might automatically generate a call to this function.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memmove8"]
  pub unsafe extern "C" fn __aeabi_memmove8(
    dest: *mut u32, src: *const u32, byte_count: usize,
  ) {
    __aeabi_memmove(dest.cast(), src.cast(), byte_count)
  }

  /// Copy between non-exclusive regions.
  ///
  /// * The pointers do not have a minimum alignment. The function will
  ///   automatically detect the best type of copy to perform.
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memmove"]
  pub unsafe extern "C" fn __aeabi_memmove(
    dest: *mut u8, src: *const u8, byte_count: usize,
  ) {
    core::arch::naked_asm! {
      // when d > s we need to copy back-to-front
      bracer::when!(("r0" >=u "r1") [1] {
        "add     r0, r0, r2",
        "add     r1, r1, r2",
        "eor     r3, r0, r1",
        "lsls    r3, r3, #31",
        "bmi     {reverse_copy_u8}",
        "bcs     2f",

        // max coalign4, possible fixup and jump
        "lsls    r3, r0, #31",
        "submi   r2, r2, #1",
        "ldrbmi  r3, [r1, #-1]!",
        "strbmi  r3, [r0, #-1]!",
        "subcs   r2, r2, #2",
        "ldrhcs  r3, [r1, #-2]!",
        "strhcs  r3, [r0, #-2]!",
        "b       {reverse_copy_u32}",

        // max coalign2, possible fixup and jump
        "2:",
        "tst     r0, #1",
        "sub     r2, r2, #1",
        "ldrb    r3, [r1, #-1]!",
        "strb    r3, [r0, #-1]!",
        "b       {reverse_copy_u16}",
      }),
      // forward copy is a normal memcpy
      "b      {__aeabi_memcpy}",
      __aeabi_memcpy = sym __aeabi_memcpy,
      reverse_copy_u8 = sym reverse_copy_u8,
      reverse_copy_u16 = sym reverse_copy_u16,
      reverse_copy_u32 = sym reverse_copy_u32,
    }
  }

  /// Copy between non-exclusive regions, prefer [`__aeabi_memmove`] if
  /// possible.
  ///
  /// This is the libc version of a memory move. It's required to return the
  /// `dest` pointer at the end of the call, which makes it need an extra
  /// push/pop compared to a direct call to `__aeabi_memmove`.
  ///
  /// * **Returns:** The `dest` pointer.
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.memmove"]
  pub unsafe extern "C" fn memmove(
    dest: *mut u8, src: *const u8, byte_count: usize,
  ) -> *mut u8 {
    core::arch::naked_asm! {
      "push {{r0, lr}}",
      "bl {__aeabi_memmove}",
      "pop {{r0, lr}}",
      "bx lr",
      __aeabi_memmove = sym __aeabi_memmove,
    }
  }

  // SET

  /// Copy between non-exclusive regions, prefer [`__aeabi_memset`] if possible.
  ///
  /// This function is provided only for API completeness, because in some cases
  /// the compiler might automatically generate a call to this function.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memset4"]
  pub unsafe extern "C" fn __aeabi_memset4(
    dest: *mut u32, byte_count: usize, byte: i32,
  ) {
    __aeabi_memset(dest.cast(), byte_count, byte)
  }

  /// Copy between non-exclusive regions, prefer [`__aeabi_memset`] if possible.
  ///
  /// This function is provided only for API completeness, because in some cases
  /// the compiler might automatically generate a call to this function.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memset8"]
  pub unsafe extern "C" fn __aeabi_memset8(
    dest: *mut u32, byte_count: usize, byte: i32,
  ) {
    __aeabi_memset(dest.cast(), byte_count, byte)
  }

  /// Sets all bytes in the region to the `byte` given.
  ///
  /// Because of historical reasons, the byte is passed in as an `i32`, but only
  /// the lowest 8 bits are used.
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memset"]
  pub unsafe extern "C" fn __aeabi_memset(
    dest: *mut u8, byte_count: usize, byte: i32,
  ) {
    core::arch::naked_asm! {
      bracer::when!(("r1" >=u "#8") [7] {
        // duplicate the byte across all of r2 and r3
        "and    r2, r2, #0xFF",
        "orr    r2, r2, r2, lsl #8",
        "orr    r2, r2, r2, lsl #16",
        "mov    r3, r2",

        // align the pointer for word ops
        "tst    r0, #0b1",
        "subne  r1, r1, #1",
        "strbne r2, [r0], #1",
        "tst    r0, #0b10",
        "subne  r1, r1, #2",
        "strhne r2, [r0], #2",

        bracer::when!(("r1" >=u "#32") [8] {
          "push {{r4-r9}}",
          "mov    r4, r2",
          "mov    r5, r2",
          "mov    r6, r2",
          "mov    r7, r2",
          "mov    r8, r2",
          "mov    r9, r2",
          "1:",
          "subs   r1, r1, #32",
          "stmge  r0!, {{r2-r9}}",
          "bgt    1b",
          "pop {{r4-r9}}",
          "bxeq   lr",
        }),

        // set 4 words
        "tst    r1, #0b10000",
        "stmne  r0!, {{r2, r3}}",
        "stmne  r0!, {{r2, r3}}",

        // set 2 and/or 1 words
        "lsls   r12, r1, #29",
        "stmcs  r0!, {{r2, r3}}",
        "strmi  r2, [r0], #4",

        // set halfword and/or byte
        "lsls   r12, r1, #31",
        "strhcs r2, [r0], #2",
        "strbmi r2, [r0], #1",
        "bx     lr",
      }),
      // byte loop
      "9:",
      "subs   r1, r1, #1",
      "strbcs r2, [r0], #1",
      "bgt    9b",
      "bx     lr",
    }
  }

  /// Write a value to all bytes in the region, prefer [`__aeabi_memset`] if
  /// possible.
  ///
  /// This is the libc version of a memory set. It's required to return the
  /// `dest` pointer at the end of the call, which makes it need an extra
  /// push/pop compared to a direct call to `__aeabi_memset`. Also, the
  /// argument ordering is swapped, so shuffling registers costs a few cycles.
  ///
  /// * **Returns:** The `dest` pointer.
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.memset"]
  pub unsafe extern "C" fn memset(
    dest: *mut u8, byte: i32, byte_count: usize,
  ) -> *mut u8 {
    core::arch::naked_asm! {
      "push {{r0, lr}}",
      "mov r3, r2",
      "mov r2, r1",
      "mov r1, r3",
      "bl {__aeabi_memset}",
      "pop {{r0, lr}}",
      "bx lr",
      __aeabi_memset = sym __aeabi_memset,
    }
  }

  // CLEAR

  /// Just call [`__aeabi_memset`] with 0 as the `byte` instead.
  ///
  /// This function is provided only for API completeness, because in some cases
  /// the compiler might automatically generate a call to this function.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memclr4"]
  pub unsafe extern "C" fn __aeabi_memclr4(dest: *mut u32, byte_count: usize) {
    __aeabi_memset(dest.cast(), byte_count, 0)
  }

  /// Just call [`__aeabi_memset`] with 0 as the `byte` instead.
  ///
  /// This function is provided only for API completeness, because in some cases
  /// the compiler might automatically generate a call to this function.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memclr8"]
  pub unsafe extern "C" fn __aeabi_memclr8(dest: *mut u32, byte_count: usize) {
    __aeabi_memset(dest.cast(), byte_count, 0)
  }

  /// Just call [`__aeabi_memset`] with 0 as the `byte` instead.
  ///
  /// This function is provided only for API completeness, because in some cases
  /// the compiler might automatically generate a call to this function.
  #[inline]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.__aeabi_memclr"]
  pub unsafe extern "C" fn __aeabi_memclr(dest: *mut u8, byte_count: usize) {
    __aeabi_memset(dest, byte_count, 0)
  }

  /// Reads 4 bytes, starting at the address given.
  ///
  /// See [__aeabi_uread4]
  ///
  /// [__aeabi_uread4]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.aeabi.uread4"]
  unsafe extern "C" fn __aeabi_uread4(address: *const c_void) -> u32 {
    core::arch::naked_asm!(
      "ldrb r2, [r0]",
      "ldrb r3, [r0, #1]",
      "orr  r2, r2, r3, lsl #8",
      "ldrb r3, [r0, #2]",
      "orr  r2, r2, r3, lsl #16",
      "ldrb r3, [r0, #3]",
      "orr  r2, r2, r3, lsl #24",
      "mov  r0, r2",
      "bx   lr",
    )
  }

  /// Writes 4 bytes, starting at the address given.
  ///
  /// See [__aeabi_uwrite4]
  ///
  /// [__aeabi_uwrite4]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.aeabi.uwrite4"]
  unsafe extern "C" fn __aeabi_uwrite4(value: u32, address: *mut c_void) {
    core::arch::naked_asm!(
      "strb r0, [r1]",
      "lsr  r2, r0, #8",
      "strb r2, [r1, #1]",
      "lsr  r2, r2, #8",
      "strb r2, [r1, #2]",
      "lsr  r2, r2, #8",
      "strb r2, [r1, #3]",
      "bx   lr",
    )
  }

  /// Reads 8 bytes, starting at the address given.
  ///
  /// See [__aeabi_uread8]
  ///
  /// [__aeabi_uread8]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.aeabi.uread8"]
  unsafe extern "C" fn __aeabi_uread8(address: *const c_void) -> u64 {
    core::arch::naked_asm!(
      "ldrb r1, [r0, #4]",
      "ldrb r2, [r0, #5]",
      "orr  r1, r1, r2, lsl #8",
      "ldrb r2, [r0, #6]",
      "orr  r1, r1, r2, lsl #16",
      "ldrb r2, [r0, #7]",
      "orr  r1, r1, r2, lsl #24",
      "b    {__aeabi_uread4}",
      __aeabi_uread4 = sym __aeabi_uread4,
    )
  }

  /// Writes 8 bytes, starting at the address given.
  ///
  /// See [__aeabi_uwrite8]
  ///
  /// [__aeabi_uwrite8]: https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#unaligned-memory-access
  #[naked]
  #[no_mangle]
  #[instruction_set(arm::a32)]
  #[link_section = ".iwram.aeabi.uwrite8"]
  unsafe extern "C" fn __aeabi_uwrite8(value: u64, address: *mut c_void) {
    core::arch::naked_asm!(
      "strb r0, [r2]",
      "lsr  r3, r0, #8",
      "strb r3, [r2, #1]",
      "lsr  r3, r3, #8",
      "strb r3, [r2, #2]",
      "lsr  r3, r3, #8",
      "strb r3, [r2, #3]",
      "strb r1, [r2, #4]",
      "lsr  r3, r1, #8",
      "strb r3, [r2, #5]",
      "lsr  r3, r3, #8",
      "strb r3, [r2, #6]",
      "lsr  r3, r3, #8",
      "strb r3, [r2, #7]",
      "bx   lr",
    )
  }
}
