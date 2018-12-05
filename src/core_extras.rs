//! Things that I wish were in core, but aren't.

/// A simple wrapper for any `*mut T` to adjust the basic operations.
///
/// Read and Write are made to be volatile. Offset is made to be
/// wrapping_offset. This makes it much easier to correctly work with IO
/// Registers and all display related memory on the GBA.
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
  /// Performs a `read_volatile`.
  pub unsafe fn read(&self) -> T {
    self.0.read_volatile()
  }

  /// Performs a `write_volatile`.
  pub unsafe fn write(&self, data: T) {
    self.0.write_volatile(data);
  }

  /// Performs a `wrapping_offset`.
  pub fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.wrapping_offset(count))
  }

  /// Performs a cast into some new pointer type.
  pub fn cast<Z>(self) -> VolatilePtr<Z> {
    VolatilePtr(self.0 as *mut Z)
  }
}
