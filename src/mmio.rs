//! Definitions for Memory-mapped IO (hardware control).

use core::ffi::c_void;

use bitfrob::u8x2;
#[allow(unused_imports)]
use voladdress::VolAddress;
use voladdress::{Unsafe, VolBlock, VolGrid2d, VolGrid2dStrided};

use crate::{
  dma::DmaControl,
  mgba::MgbaMessageLevel,
  video::{Color, DisplayControl, DisplayStatus, Mode3, Tile4bpp},
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

/// Display Control setting.
///
/// This sets what background mode is active, as well as various related
/// details.
///
/// Unlike most MMIO, this doesn't have an "all 0" state at boot. The
/// `forced_blank` bit it left set by the BIOS's startup routine.
pub const DISPCNT: PlainAddr<DisplayControl> =
  unsafe { VolAddress::new(0x0400_0000) };

/// Display Status setting.
///
/// Gives info on the display state, and controls display-based interrupts.
pub const DISPSTAT: PlainAddr<DisplayStatus> =
  unsafe { VolAddress::new(0x0400_0004) };

/// The current scanline that the display is working on.
///
/// Values of 160 to 227 indicate that a vertical blank line is happening.
pub const VCOUNT: RoAddr<u8> = unsafe { VolAddress::new(0x0400_0006) };

/// Source address for DMA3.
///
/// The correct pointer type depends on the transfer mode used.
pub const DMA0_SOURCE: VolAddress<*const c_void, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00B0) };

/// Destination address for DMA3.
///
/// The correct pointer type depends on the transfer mode used.
pub const DMA0_DESTINATION: VolAddress<*mut c_void, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00B4) };

/// The number of transfers desired.
///
/// A value of 0 indicates the maximum number of transfers: `0x4000`
pub const DMA0_TRANSFER_COUNT: VolAddress<u16, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00B8) };

/// DMA3 Control Bits.
pub const DMA0_CONTROL: VolAddress<DmaControl, SOGBA, Unsafe> =
  unsafe { VolAddress::new(0x0400_00BA) };

/// Source address for DMA3.
///
/// The correct pointer type depends on the transfer mode used.
pub const DMA1_SOURCE: VolAddress<*const c_void, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00BC) };

/// Destination address for DMA3.
///
/// The correct pointer type depends on the transfer mode used.
pub const DMA1_DESTINATION: VolAddress<*mut c_void, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00C0) };

/// The number of transfers desired.
///
/// A value of 0 indicates the maximum number of transfers: `0x4000`
pub const DMA1_TRANSFER_COUNT: VolAddress<u16, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00C4) };

/// DMA3 Control Bits.
pub const DMA1_CONTROL: VolAddress<DmaControl, SOGBA, Unsafe> =
  unsafe { VolAddress::new(0x0400_00C6) };

/// Source address for DMA3.
///
/// The correct pointer type depends on the transfer mode used.
pub const DMA2_SOURCE: VolAddress<*const c_void, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00C8) };

/// Destination address for DMA3.
///
/// The correct pointer type depends on the transfer mode used.
pub const DMA2_DESTINATION: VolAddress<*mut c_void, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00CC) };

/// The number of transfers desired.
///
/// A value of 0 indicates the maximum number of transfers: `0x4000`
pub const DMA2_TRANSFER_COUNT: VolAddress<u16, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00D0) };

/// DMA3 Control Bits.
pub const DMA2_CONTROL: VolAddress<DmaControl, SOGBA, Unsafe> =
  unsafe { VolAddress::new(0x0400_00D2) };

/// Source address for DMA3.
///
/// The correct pointer type depends on the transfer mode used.
pub const DMA3_SOURCE: VolAddress<*const c_void, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00D4) };

/// Destination address for DMA3.
///
/// The correct pointer type depends on the transfer mode used.
pub const DMA3_DESTINATION: VolAddress<*mut c_void, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00D8) };

/// The number of transfers desired.
///
/// A value of 0 indicates the maximum number of transfers: `0x1_0000`
pub const DMA3_TRANSFER_COUNT: VolAddress<u16, (), Unsafe> =
  unsafe { VolAddress::new(0x0400_00DC) };

/// DMA3 Control Bits.
pub const DMA3_CONTROL: VolAddress<DmaControl, SOGBA, Unsafe> =
  unsafe { VolAddress::new(0x0400_00DE) };

/// Key Input (read-only).
///
/// Gives the low-active button state of all system buttons.
pub const KEYINPUT: RoAddr<KeyInput> = unsafe { VolAddress::new(0x0400_0130) };

/// Interrupts Enabled.
///
/// When any sub-system is set to "send" interrupts, that interrupt type must
/// *also* be configured here or it won't actually be "received" by the CPU.
pub const IE: PlainAddr<IrqBits> = unsafe { VolAddress::new(0x0400_0200) };

/// Interrupts Flagged.
///
/// These are the interrupts that are pending, and haven't been handled. Clear a
/// pending interrupt by writing an [`IrqBits`] value with that bit enabled. The
/// assembly runtime handles this automatically, so you don't normally need to
/// interact with `IF` at all.
pub const IF: PlainAddr<IrqBits> = unsafe { VolAddress::new(0x0400_0202) };

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

// mGBA Logging

/// The buffer to put logging messages into.
///
/// The first 0 in the buffer is the end of each message.
pub const MGBA_LOG_BUFFER: VolBlock<u8, SOGBA, SOGBA, 256> =
  unsafe { VolBlock::new(0x04FF_F600) };

/// Write to this each time you want to reset a message (it also resets the
/// buffer).
pub const MGBA_LOG_SEND: WoAddr<MgbaMessageLevel> =
  unsafe { VolAddress::new(0x04FF_F700) };

/// Allows you to attempt to activate mGBA logging.
pub const MGBA_LOG_ENABLE: PlainAddr<u16> =
  unsafe { VolAddress::new(0x04FF_F780) };

/// The backdrop color is the color shown when no *other* element is displayed
/// in a given pixel.
pub const BACKDROP_COLOR: PlainAddr<Color> =
  unsafe { VolAddress::new(0x0500_0000) };

/// Palette data for the backgrounds
pub const BG_PALRAM: VolBlock<Color, SOGBA, SOGBA, 256> =
  unsafe { VolBlock::new(0x0500_0000) };

/// Palette data for the objects.
pub const OBJ_PALRAM: VolBlock<Color, SOGBA, SOGBA, 256> =
  unsafe { VolBlock::new(0x0500_0000) };

/// The VRAM's background tile view, using 4bpp tiles.
pub const VRAM_BG_TILE4: VolBlock<Tile4bpp, SOGBA, SOGBA, 2048> =
  unsafe { VolBlock::new(0x0600_0000) };

/// The VRAM's background tile view, using 8bpp tiles.
pub const VRAM_BG_TILE8: VolBlock<Tile4bpp, SOGBA, SOGBA, 1024> =
  unsafe { VolBlock::new(0x0600_0000) };

/// The VRAM's view in Video Mode 3 (240 x 160).
///
/// Each location is a direct color value.
pub const MODE3_VRAM: VolGrid2d<
  Color,
  SOGBA,
  SOGBA,
  { Mode3::WIDTH as usize },
  { Mode3::HEIGHT as usize },
> = unsafe { VolGrid2d::new(0x0600_0000) };

/// The VRAM's view in Video Mode 4.
///
/// Each location is a pair of palette indexes into the background palette.
/// Because the VRAM can't be written with a single byte, we have to work with
/// this in units of [`u8x2`]. It's annoying, I know.
pub const MODE4_VRAM: VolGrid2dStrided<
  u8x2,
  SOGBA,
  SOGBA,
  { 240 / 2 },
  160,
  2,
  0xA000,
> = unsafe { VolGrid2dStrided::new(0x0600_0000) };

/// The VRAM's view in Video Mode 5.
///
/// Each location is a direct color value, but there's a lower image size to
/// allow for two frames.
pub const MODE5_VRAM: VolGrid2dStrided<
  Color,
  SOGBA,
  SOGBA,
  160,
  128,
  2,
  0xA000,
> = unsafe { VolGrid2dStrided::new(0x0600_0000) };
