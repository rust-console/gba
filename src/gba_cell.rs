//! Provides the [`GbaCell`] type.

/// A "cell" type suitable to hold a global on the GBA.
#[repr(transparent)]
pub struct GbaCell<T>(core::cell::UnsafeCell<T>);

#[cfg(feature = "on_gba")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "on_gba")))]
unsafe impl<T> Sync for GbaCell<T> {}

impl<T> GbaCell<T> {
  /// Constructs a new cell with the value given
  pub const fn new(t: T) -> Self {
    Self(core::cell::UnsafeCell::new(t))
  }
}
