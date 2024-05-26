use super::{__aeabi_memset, _bulk_set_util};

#[inline]
#[instruction_set(arm::a32)]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memclr8(dest: *mut u32, byte_count: usize) {
  unsafe { _bulk_set_util(dest, byte_count, 0, 0) }
}

#[inline]
#[instruction_set(arm::a32)]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memclr4(dest: *mut u32, byte_count: usize) {
  unsafe { _bulk_set_util(dest, byte_count, 0, 0) }
}

#[inline]
#[instruction_set(arm::a32)]
#[cfg_attr(feature = "no_mangle_memset", no_mangle)]
pub unsafe extern "C" fn __aeabi_memclr(dest: *mut u8, byte_count: usize) {
  unsafe { __aeabi_memset(dest.cast(), byte_count, 0) }
}
