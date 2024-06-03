//! Definitions for Memory-mapped IO (hardware control).

use core::ffi::c_void;

use bitfrob::u8x2;
#[allow(unused_imports)]
use voladdress::VolAddress;
use voladdress::{Unsafe, VolBlock, VolGrid2d, VolGrid2dStrided, VolSeries};

use crate::{
  dma::DmaControl,
  mgba::MgbaLogLevel,
  obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2},
  video::{
    BackgroundControl, Color, DisplayControl, DisplayStatus, TextEntry, Tile4,
  },
  IrqBits, KeyInput,
};

/// "safe on GBA", which is either Safe or Unsafe according to the `on_gba`
/// cargo feature.
#[cfg(feature = "on_gba")]
type SOGBA = voladdress::Safe;
#[cfg(not(feature = "on_gba"))]
type SOGBA = voladdress::Unsafe;

/// Responds "normally" to read/write, just holds a setting
type PlainAddr<T> = VolAddress<T, SOGBA, SOGBA>;
/// Read-only addr
type RoAddr<T> = VolAddress<T, SOGBA, ()>;
/// Write-only addr
type WoAddr<T> = VolAddress<T, (), SOGBA>;

mod peripheral_controls;
mod video_memory;

pub use peripheral_controls::*;
pub use video_memory::*;
