//! Definitions for Memory-mapped IO (hardware control).

use voladdress::VolAddress;

/// "safe on GBA", which is either Safe or Unsafe according to the `on_gba`
/// cargo feature.
#[cfg(feature = "on_gba")]
type SOGBA = voladdress::Safe;
#[cfg(not(feature = "on_gba"))]
type SOGBA = voladdress::Unsafe;

type PlainAddr<T> = VolAddress<T, SOGBA, SOGBA>;

/// Interrupt Master Enable
///
/// * When this is set to `true`, hardware interrupts that are flagged will
///   immediately run the interrupt handler.
/// * When this is `false`, any interrupt events that are flagged will be left
///   pending until this is again set to `true`.
///
/// This defaults to `false`.
pub const IME: PlainAddr<bool> = unsafe { VolAddress::new(0x0400_0208) };
