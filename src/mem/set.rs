/// Sets all bytes in the region to the `byte` given.
///
/// Because of historical reasons, the byte is passed in as an `i32`, but only
/// the lowest 8 bits are used.
///
/// ## Safety
/// * `dest` must be valid to write to for `byte_count` bytes.
#[instruction_set(arm::a32)]
#[link_section = ".iwram.__aeabi_memset"]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memset(
  mut dest: *mut u8, mut byte_count: usize, byte: i32,
) {
  if byte_count >= 8 {
    let byte: u8 = byte as u8;
    let byte: u16 = (byte as u16) << 8 | (byte as u16);
    let byte: u32 = (byte as u32) << 16 | (byte as u32);

    if (dest as usize & 1) != 0 {
      unsafe { dest.write_volatile(byte as u8) };
      dest = unsafe { dest.add(1) };
      byte_count -= 1;
    }

    let mut dest = dest.cast::<u16>();
    debug_assert!(dest.is_aligned());

    if (dest as usize & 0b10) != 0 {
      unsafe { dest.write_volatile(byte as u16) };
      dest = unsafe { dest.add(1) };
      byte_count -= 2;
    }

    let dest = dest.cast::<u32>();
    debug_assert!(dest.is_aligned());

    let byte_r2 = byte;
    let byte_r3 = byte;
    unsafe { _bulk_set_util(dest, byte_count, byte_r2, byte_r3) };
  } else {
    let byte = byte as u8;
    for _ in 0..byte_count {
      unsafe { dest.write_volatile(byte) };
      dest = unsafe { dest.add(1) };
    }
  }
}

/// Like [`__aeabi_memset`], but for a known-aligned pointer.
///
/// ## Safety
/// * `dest` must be valid to write to for `byte_count` bytes.
/// * The `dest` must be aligned to 4.
#[inline]
#[instruction_set(arm::a32)]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memset4(
  dest: *mut u32, byte_count: usize, byte: i32,
) {
  debug_assert!(dest.is_aligned());
  if byte_count >= 8 {
    let byte: u8 = byte as u8;
    let byte: u16 = (byte as u16) << 8 | (byte as u16);
    let byte: u32 = (byte as u32) << 16 | (byte as u32);
    unsafe { _bulk_set_util(dest, byte_count, byte, byte) };
  } else {
    let mut dest = dest.cast::<u8>();
    let byte = byte as u8;
    for _ in 0..byte_count {
      unsafe { dest.write_volatile(byte) };
      dest = unsafe { dest.add(1) };
    }
  }
}

/// Sets `reg2` and `reg3` all across the destination.
/// ## Safety
/// * `dest` must be aligned and valid to write for `byte_count` bytes.
#[instruction_set(arm::a32)]
#[link_section = ".iwram._bulk_set_util"]
pub(crate) unsafe extern "C" fn _bulk_set_util(
  mut dest: *mut u32, mut byte_count: usize, reg2: u32, reg3: u32,
) {
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
        inlateout("r0") dest,
        inlateout("r1") byte_count,
        in("r2") reg2,
        in("r3") reg3,
      );
    }
  }

  #[allow(unused_assignments)]
  unsafe {
    core::arch::asm!(
      // check for 4 remaining words.
      "tst    r1, #0b10000",
      "stmne  r0!, {{r2, r3}}",
      "stmne  r0!, {{r2, r3}}",
      // check for 2 and/or 1 remaining words
      "lsls   {trash}, r1, #29",
      "stmcs  r0!, {{r2, r3}}",
      "strmi  r2, [r0], #4",
      // set halfword and/or byte
      "lsls   {trash}, r1, #29",
      "stmcs  r0!, {{r2, r3}}",
      "strmi  r2, [r0], #4",
      trash = out(reg) _,
      inlateout("r0") dest,
      in("r1") byte_count,
      in("r2") reg2,
      in("r3") reg3,
      options(nostack)
    );
  }
}

/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
#[inline]
#[instruction_set(arm::a32)]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memset8(
  dest: *mut u32, byte_count: usize, byte: i32,
) {
  unsafe { __aeabi_memset4(dest.cast(), byte_count, byte) }
}

/// Write a value to all bytes in the region.
///
/// This is the `libc` version of a memory set. This implementation just calls
/// [`__aeabi_memset`] with the argument order swapped around. Prefer a direct
/// call to that function if possible.
///
/// This function is provided only for API completeness, because in some cases
/// the compiler might automatically generate a call to this function.
///
/// * **Returns:** The `dest` pointer.
#[inline]
#[instruction_set(arm::a32)]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn memset(
  dest: *mut u8, byte: i32, byte_count: usize,
) -> *mut u8 {
  unsafe { __aeabi_memset(dest, byte_count, byte) };
  dest
}
