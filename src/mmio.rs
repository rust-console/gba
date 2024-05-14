//! Definitions for Memory-mapped IO (hardware control).

#[allow(unused_imports)]
use voladdress::{Safe, Unsafe, VolAddress};

use crate::{
  video::{Color, DisplayControl},
  KeyInput,
};

/// "safe on GBA", which is either Safe or Unsafe according to the `on_gba`
/// cargo feature.
#[cfg(feature = "on_gba")]
type SOGBA = Safe;
#[cfg(not(feature = "on_gba"))]
type SOGBA = Unsafe;

/// Responds "normally" to read/write, just holds a setting
type PlainAddr<T> = VolAddress<T, SOGBA, SOGBA>;
/// Read-only addr
type RoAddr<T> = VolAddress<T, SOGBA, ()>;

/// Display Control setting.
///
/// This sets what background mode is active, as well as various related
/// details.
///
/// Unlike most MMIO, this doesn't have an "all 0" state at boot. The
/// `forced_blank` bit it left set by the BIOS's startup routine.
pub const DISPCNT: PlainAddr<DisplayControl> =
  unsafe { VolAddress::new(0x0400_0000) };

/// Key Input (read-only).
///
/// Gives the low-active button state of all system buttons.
pub const KEYINPUT: RoAddr<KeyInput> = unsafe { VolAddress::new(0x0400_0130) };

/// Interrupt Master Enable
///
/// * When this is set to `true`, hardware interrupts that are flagged will
///   immediately run the interrupt handler.
/// * When this is `false`, any interrupt events that are flagged will be left
///   pending until this is again set to `true`.
///
/// This defaults to `false`.
///
/// Technically there's a two CPU cycle delay between this being written and
/// interrupts actually being enabled/disabled. In practice, it doesn't matter.
pub const IME: PlainAddr<bool> = unsafe { VolAddress::new(0x0400_0208) };

/// The backdrop color is the color shown when no *other* element is displayed
/// in a given pixel.
pub const BACKDROP_COLOR: PlainAddr<Color> =
  unsafe { VolAddress::new(0x0500_0000) };
