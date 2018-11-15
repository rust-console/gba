//! Things that I wish were in core, but aren't.

/// A simple wrapper for any `*mut T` to adjust the basic operations.
///
/// Read and Write are made to be volatile. Offset is made to be
/// wrapping_offset. This makes it much easier to correctly work with IO
/// Registers and all display related memory on the GBA.
///
/// As a bonus, use of this type is mostly `cargo test` safe. Reads will return
/// a `zeroed()` value instead, and writes will do nothing.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<T>(pub *mut T);

impl<T> core::fmt::Pointer for VolatilePtr<T> {
  /// Formats exactly like the inner `*mut T`.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:p}", self.0)
  }
}

impl<T> VolatilePtr<T> {
  /// Performs a volatile read.
  ///
  /// # Safety
  ///
  /// This method adds absolutely no additional safety, so all safety concerns
  /// for a normal raw pointer volatile read apply.
  pub unsafe fn read(&self) -> T {
    #[cfg(not(test))]
    {
      core::ptr::read_volatile(self.0)
    }
    #[cfg(test)]
    {
      core::mem::zeroed::<T>()
    }
  }

  /// Performs a volatile write.
  ///
  /// # Safety
  ///
  /// This method adds absolutely no additional safety, so all safety concerns
  /// for a normal raw pointer volatile write apply.
  pub unsafe fn write(&self, data: T) {
    #[cfg(not(test))]
    {
      core::ptr::write_volatile(self.0, data);
    }
    #[cfg(test)]
    {
      drop(data)
    }
  }

  /// Performs a wrapping_offset by the number of slots given to a new position.
  ///
  /// # Safety
  ///
  /// This is a wrapping_offset, so all safety concerns of a normal raw pointer
  /// wrapping_offset apply.
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.wrapping_offset(count))
  }
}
