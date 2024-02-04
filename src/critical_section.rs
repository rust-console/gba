use critical_section::{set_impl, Impl, RawRestoreState};

use crate::mmio::IME;

struct GbaCriticalSection;
set_impl!(GbaCriticalSection);

unsafe impl Impl for GbaCriticalSection {
  unsafe fn acquire() -> RawRestoreState {
    let restore = IME.read();
    IME.write(false);
    restore
  }

  unsafe fn release(restore: RawRestoreState) {
    IME.write(restore);
  }
}
