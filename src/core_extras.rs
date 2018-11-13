//! Things that I wish were in core, but aren't.

/// A simple wrapper to any `*mut T` so that the basic "read" and "write"
/// operations are volatile.
///
/// Accessing the GBA's IO registers and video ram and specific other places on
/// **must** be done with volatile operations. Having this wrapper makes that
/// more clear for all the global const values into IO registers.
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
    core::ptr::read_volatile(self.0)
  }

  /// Performs a volatile write.
  ///
  /// # Safety
  ///
  /// This method adds absolutely no additional safety, so all safety concerns
  /// for a normal raw pointer volatile write apply.
  pub unsafe fn write(&self, data: T) {
    core::ptr::write_volatile(self.0, data);
  }

  /// Offsets this address by the amount given.
  ///
  /// # Safety
  ///
  /// This is a standard offset, so all safety concerns of a normal raw pointer
  /// offset apply.
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.offset(count))
  }
}
