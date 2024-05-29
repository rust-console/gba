/// The core "memory setting" function.
///
/// * `dest` is the destination pointer.
/// * `byte_count` is the number of bytes to write.
/// * `x` holds the desired word to write across all of the bytes.
///
/// ## Safety
/// * `dest` must be aligned to 4 and writable for `byte_count` bytes.
#[inline(never)]
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(feature = "on_gba", link_section = ".iwram.bulk_memory_set")]
pub unsafe extern "C" fn bulk_memory_set(
  mut dest: *mut u32, byte_count: usize, x: u32,
) {
  on_gba_or_unimplemented!(
    // debug assert alignment, but if we call the method for this is generates a
    // stupid huge amount of code.
    debug_assert_eq!(dest as usize & 0b11, 0);

    let (mut blocks, mut spare) = (byte_count / 32, byte_count % 32);

    while blocks > 0 {
      unsafe {
        core::arch::asm!(
          "stm  r0!, {{r2,r3,r4,r5,r7,r8,r9,r10}}",

          inout("r0") dest,
          in("r2") x,
          in("r3") x,
          in("r4") x,
          in("r5") x,
          in("r7") x,
          in("r8") x,
          in("r9") x,
          in("r10") x,
          options(nostack),
        );
      };
      blocks -= 1;
    }

    debug_assert!(spare < 32);

    while spare > 4 {
      unsafe { dest.write_volatile(x) };
      dest = unsafe { dest.add(1) };
      spare -= 4;
    }

    let mut dest = dest.cast::<u8>();
    let x = x as u8;
    while spare > 0 {
      unsafe { dest.write_volatile(x) };
      dest = unsafe { dest.add(1) };
      spare -= 1;
    }
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
  unsafe { bulk_memory_set(dest.cast(), byte_count, byte32) };
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

    unsafe { bulk_memory_set(dest.cast(), byte_count, byte32) };
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
