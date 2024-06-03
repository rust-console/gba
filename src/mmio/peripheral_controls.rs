use crate::timers::TimerControl;

use super::*;

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

/// Background 0 controls
pub const BG0CNT: PlainAddr<BackgroundControl> =
  unsafe { VolAddress::new(0x0400_0008) };

/// Background 1 controls
pub const BG1CNT: PlainAddr<BackgroundControl> =
  unsafe { VolAddress::new(0x0400_000A) };

/// Background 2 controls
pub const BG2CNT: PlainAddr<BackgroundControl> =
  unsafe { VolAddress::new(0x0400_000C) };

/// Background 3 controls
pub const BG3CNT: PlainAddr<BackgroundControl> =
  unsafe { VolAddress::new(0x0400_000E) };

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

/// Timer0's current counter value.
pub const TIMER0_COUNTER: RoAddr<u16> = unsafe { VolAddress::new(0x0400_0100) };
/// Timer1's current counter value.
pub const TIMER1_COUNTER: RoAddr<u16> = unsafe { VolAddress::new(0x0400_0104) };
/// Timer2's current counter value.
pub const TIMER2_COUNTER: RoAddr<u16> = unsafe { VolAddress::new(0x0400_0108) };
/// Timer3's current counter value.
pub const TIMER3_COUNTER: RoAddr<u16> = unsafe { VolAddress::new(0x0400_010C) };

/// The value for Timer0 to reload on overflow on when the `start` bit is newly
/// set.
pub const TIMER0_RELOAD: WoAddr<u16> = unsafe { VolAddress::new(0x0400_0100) };
/// The value for Timer1 to reload on overflow on when the `start` bit is newly
/// set.
pub const TIMER1_RELOAD: WoAddr<u16> = unsafe { VolAddress::new(0x0400_0104) };
/// The value for Timer2 to reload on overflow on when the `start` bit is newly
/// set.
pub const TIMER2_RELOAD: WoAddr<u16> = unsafe { VolAddress::new(0x0400_0108) };
/// The value for Timer3 to reload on overflow on when the `start` bit is newly
/// set.
pub const TIMER3_RELOAD: WoAddr<u16> = unsafe { VolAddress::new(0x0400_010C) };

/// Control bits for Timer 0.
pub const TIMER0_CONTROL: PlainAddr<TimerControl> =
  unsafe { VolAddress::new(0x0400_0102) };

/// Control bits for Timer 1.
pub const TIMER1_CONTROL: PlainAddr<TimerControl> =
  unsafe { VolAddress::new(0x0400_0106) };

/// Control bits for Timer 2.
pub const TIMER2_CONTROL: PlainAddr<TimerControl> =
  unsafe { VolAddress::new(0x0400_010A) };

/// Control bits for Timer 3.
pub const TIMER3_CONTROL: PlainAddr<TimerControl> =
  unsafe { VolAddress::new(0x0400_010E) };

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

/// The buffer to put logging messages into.
///
/// The first `\0` in the buffer is the end of each message.
pub const MGBA_LOG_BUFFER: VolBlock<u8, SOGBA, SOGBA, 256> =
  unsafe { VolBlock::new(0x04FF_F600) };

/// Write to this each time you want to send out the current buffer content.
///
/// It also resets the buffer content.
pub const MGBA_LOG_SEND: WoAddr<MgbaLogLevel> =
  unsafe { VolAddress::new(0x04FFF700) };

/// Allows you to enable/disable mGBA logging.
///
/// This is enabled by default by the assembly runtime, so you don't normally
/// need to touch this.
pub const MGBA_LOG_ENABLE: PlainAddr<u16> =
  unsafe { VolAddress::new(0x04FF_F780) };
