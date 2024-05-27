/// The core "memory setting" function.
///
/// * `dest` is the destination pointer.
/// * `byte_count` is the number of bytes to write.
/// * `r2` and `r3` should hold the desired byte, repeated into all four bytes
///   of each `u32` value.
///
/// ## Safety
/// * `dest` must be aligned to 4 and writable for `byte_count` bytes.
#[inline(never)]
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(feature = "on_gba", link_section = ".iwram._bulk_set_util")]
pub unsafe extern "C" fn bulk_memory_set(
  mut dest: *mut u32, mut byte_count: usize, r2: u32, r3: u32,
) {
  on_gba_or_unimplemented!(
    // debug assert alignment, but if we call the method for this is generates a
    // stupid huge amount of code.
    debug_assert_eq!(dest as usize & 0b11, 0);

    // Note(Lokathor): We need a threshold for how many bytes is the minimum for
    // doing the `stm` loop. It requires a big push/pop to get enough usable
    // registers, and then setting up those registers as well. That's all
    // somewhat costly, so we don't want the threshold too low. The current
    // threshold of 32 bytes is essentially an arbitrary one, it's the size of
    // one 4bpp tile.
    if byte_count >= 32 {
      unsafe {
        core::arch::asm!(
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

          inout("r0") dest,
          inout("r1") byte_count,
          in("r2") r2,
          in("r3") r3,
        );
      }
    }

    unsafe {
      core::arch::asm!(
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

        inout("r0") dest => _,
        inout("r1") byte_count => _,
        in("r2") r2,
        in("r3") r3,
        options(nostack)
      )
    };
  );
}

/// AEABI-styled memory set.
///
/// ## Safety
/// * `dest` must be aligned to 4 and writable for `byte_count` bytes.
#[inline]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memset4(
  dest: *mut u32, byte_count: usize, byte: i32,
) {
  let byte8 = byte as u8;
  let byte16 = byte8 as u16 | (byte8 as u16) << 8;
  let byte32 = byte16 as u32 | (byte16 as u32) << 16;

  debug_assert_eq!(dest as usize & 0b11, 0);
  unsafe { bulk_memory_set(dest.cast(), byte_count, byte32, byte32) };
}

/// AEABI-styled memory set.
///
/// ## Safety
/// * `dest` must be aligned to 8 and writable for `byte_count` bytes.
#[inline]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memset8(
  dest: *mut u32, byte_count: usize, byte: i32,
) {
  debug_assert_eq!(dest as usize & 0b111, 0);
  unsafe { __aeabi_memset4(dest.cast(), byte_count, byte) }
}

/// AEABI-styled memory set.
///
/// ## Safety
/// * `dest` must be writable for `byte_count` bytes.
#[inline]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memset(
  mut dest: *mut u8, mut byte_count: usize, byte: i32,
) {
  if byte_count == 0 {
    return;
  }
  let byte8 = byte as u8;
  let byte16 = byte8 as u16 | (byte8 as u16) << 8;
  let byte32 = byte16 as u32 | (byte16 as u32) << 16;

  // We only get fancy if the requested span is sufficiently large.
  if byte_count >= 8 {
    if (dest as usize) & 0b1 != 0 {
      debug_assert!(byte_count >= 1);
      unsafe { dest.write_volatile(byte8) };
      dest = unsafe { dest.add(1) };
      byte_count -= 1;
      if byte_count == 0 {
        return;
      }
    }
    let mut dest: *mut u16 = dest.cast();
    debug_assert_eq!(dest as usize & 0b1, 0);

    if (dest as usize) & 0b10 != 0 {
      debug_assert!(byte_count >= 2);
      unsafe { dest.write_volatile(byte16) };
      dest = unsafe { dest.add(1) };
      byte_count -= 2;
      if byte_count == 0 {
        return;
      }
    }
    let dest: *mut u32 = dest.cast();
    debug_assert_eq!(dest as usize & 0b11, 0);

    unsafe { bulk_memory_set(dest.cast(), byte_count, byte32, byte32) };
  } else {
    for _ in 0..byte_count {
      unsafe { dest.write_volatile(byte8) };
      dest = unsafe { dest.add(1) };
    }
  }
}

/// `libc`-style memory set.
///
/// Don't ever call this function. Literally you don't ever want to directly
/// call this function. It's **always** slightly more costly than just calling
/// [`__aeabi_memset`] directly. This function is provided only because the
/// compiler will occasionally directly insert calls to `memset`, and so this is
/// needed for compatibility.
#[inline]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn memset(
  dest: *mut u8, byte: i32, byte_count: usize,
) -> *mut u8 {
  unsafe { __aeabi_memset(dest, byte_count, byte) };
  dest
}
