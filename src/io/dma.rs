//! Module for using the four Direct Memory Access (DMA) units.
//!
//! The GBA has four DMA units, numbered 0 through 3. If you ever try to have
//! more than one active at once the lowest numbered DMA will take priority and
//! complete first. Any use of DMA halts the CPU's operation. DMA can also be
//! configured to activate automatically at certain times, and when configured
//! like that the CPU runs in between the automatic DMA activations. (This is
//! actually the intended method for doing sound.) Each DMA unit has an intended
//! use:
//!
//! * DMA0: highest priority, but can only read from internal memory.
//! * DMA1/DMA2: Intended for sound transfers.
//! * DMA3: Can be used to write into Game Pak ROM / FlashROM (not SRAM).
//!
//! ## DMA Anatomy
//!
//! Each DMA is utilized via a combination four IO registers:
//!
//! * **Source Address:** (`*const u32`) Where to read from. DMA0 can only read
//!   from internal memory, the other units can read from any non-SRAM memory.
//! * **Destination Address:** (`*mut u32`) Where to write to. DMA0/1/2 can only
//!   write to internal memory, DMA3 can write to any non-SRAM memory.
//! * **Word Count:** (`u16`) How many units to transfer. Despite being called
//!   "word count" you can also use DMA to transfer half-words. DMA0/1/2 are
//!   limited to a 14-bit counter value, DMA3 allowed the full 16-bit range to
//!   be used for the counter. Note that even when transferring half-words you
//!   MUST have both Source and Destination be 32-bit aligned.
//! * **Control:** (`DMAControlSetting`) This is one of those fiddly bit-flag
//!   registers with all sorts of settings. See the type for more info.
//!
//! Note that Source, Destination, and Count are all read-only, while the
//! Control is read/write. When a DMA unit is _Enabled_ it copies the relevent
//! Source, Destination, and Count values into its own internal registers (so a
//! second Enable will reuse the old values). If the DMA _Repeats_ it re-copies
//! the Count, and also the Destination if
//! `DMADestAddressControl::IncrementReload` is configured in the Control, but
//! not the Source.
//!
//! When the DMA completes the Enable bit will be cleared from the Control,
//! unless the Repeat bit is set in the Control, in which case the Enable bit is
//! left active and the DMA will automatically activate again at the right time
//! (depending on the Start Timing setting). You have to manually turn off the
//! correct bit to stop the DMA unit.
//!
//! ## Safety
//!
//! As you might have noticed by now, utilizing DMA can be very fiddly. It moves
//! around bytes with no concern for the type system, including the `Clone` and
//! `Copy` traits that Rust relies on. Use of DMA can be made _somewhat safe_
//! via wrapper methods (such as those we've provided), but it's fundamentally
//! an unsafe thing to use.
//!
//! ## DMA Can Cause Subtle Bugs
//!
//! Since the CPU is halted while DMA is active you can miss out on interrupts
//! that should have fired. This can cause any number of unintended effects. DMA
//! is primarily intended for loading large amounts of graphical data from ROM,
//! or loading sound data at regulated intervals to avoid pops and crackles. It
//! _can_ be used for general purpose bulk transfers but you are advised to use
//! restraint.

use super::*;

newtype! {
  /// Allows you to configure a DMA unit.
  DMAControlSetting, u16
}
#[allow(missing_docs)]
impl DMAControlSetting {
  phantom_fields! {
    self.0: u16,
    dest_address_control: 5-6=DMADestAddressControl<Increment, Decrement, Fixed, IncrementReload>,
    source_address_control: 7-8=DMASrcAddressControl<Increment, Decrement, Fixed>,
    dma_repeat: 9,
    use_32bit: 10,
    start_time: 12-13=DMAStartTiming<Immediate, VBlank, HBlank, Special>,
    irq_when_done: 14,
    enabled: 15,
  }
}

/// Sets how the destination address should be adjusted per data transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DMADestAddressControl {
  /// Offset +1
  Increment = 0,
  /// Offset -1
  Decrement = 1,
  /// No change
  Fixed = 2,
  /// Offset +1 per transfer and auto-reset to base when the DMA repeats.
  IncrementReload = 3,
}

/// Sets how the source address should be adjusted per data transfer.
///
/// Note that only 0,1,2 are allowed, 3 is prohibited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DMASrcAddressControl {
  /// Offset +1
  Increment = 0,
  /// Offset -1
  Decrement = 1,
  /// No change
  Fixed = 2,
}

/// Sets when the DMA should activate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DMAStartTiming {
  /// Causes the DMA to start as soon as possible (2 wait cycles after enabled)
  Immediate = 0,
  /// Start at VBlank
  VBlank = 1,
  /// Start at HBlank
  HBlank = 2,
  /// The special timing depends on the DMA it's used with:
  /// * 0: Prohibited
  /// * 1/2: Sound FIFO,
  /// * 3: Video Capture, for transferring from memory/camera into VRAM
  Special = 3,
}

pub struct DMA0;
impl DMA0 {
  /// DMA 0 Source Address, read only.
  const DMA0SAD: VolAddress<*const u32> = unsafe { VolAddress::new(0x400_00B0) };
  /// DMA 0 Destination Address, read only.
  const DMA0DAD: VolAddress<*mut u32> = unsafe { VolAddress::new(0x400_00B4) };
  /// DMA 0 Word Count, read only.
  const DMA0CNT_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_00B8) };
  /// DMA 0 Control, read/write.
  const DMA0CNT_H: VolAddress<DMAControlSetting> = unsafe { VolAddress::new(0x400_00BA) };

  /// Assigns the source register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The source pointer must be aligned and valid to read from.
  pub unsafe fn set_source(src: *const u32) {
    Self::DMA0SAD.write(src)
  }

  /// Assigns the destination register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The source pointer must be aligned and valid to write to.
  pub unsafe fn set_dest(dest: *mut u32) {
    Self::DMA0DAD.write(dest)
  }

  /// Assigns the count register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The count given must specify a valid number of units to write, starting at
  /// the assigned destination address.
  pub unsafe fn set_count(count: u16) {
    Self::DMA0CNT_L.write(count)
  }

  /// Reads the current control setting.
  pub fn control() -> DMAControlSetting {
    Self::DMA0CNT_H.read()
  }

  /// Writes the control setting given.
  ///
  /// # Safety
  ///
  /// You must ensure that the Source, Destination, and Count values are set
  /// correctly **before** you activate the Enable bit.
  pub unsafe fn set_control(setting: DMAControlSetting) {
    Self::DMA0CNT_H.write(setting)
  }
}

pub struct DMA1;
impl DMA1 {
  /// DMA 1 Source Address, read only.
  const DMA1SAD: VolAddress<*const u32> = unsafe { VolAddress::new(0x400_00BC) };
  /// DMA 1 Destination Address, read only.
  const DMA1DAD: VolAddress<*mut u32> = unsafe { VolAddress::new(0x400_00C0) };
  /// DMA 1 Word Count, read only.
  const DMA1CNT_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_00C4) };
  /// DMA 1 Control, read/write.
  const DMA1CNT_H: VolAddress<DMAControlSetting> = unsafe { VolAddress::new(0x400_00C6) };

  /// Assigns the source register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The source pointer must be aligned and valid to read from.
  pub unsafe fn set_source(src: *const u32) {
    Self::DMA1SAD.write(src)
  }

  /// Assigns the destination register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The source pointer must be aligned and valid to write to.
  pub unsafe fn set_dest(dest: *mut u32) {
    Self::DMA1DAD.write(dest)
  }

  /// Assigns the count register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The count given must specify a valid number of units to write, starting at
  /// the assigned destination address.
  pub unsafe fn set_count(count: u16) {
    Self::DMA1CNT_L.write(count)
  }

  /// Reads the current control setting.
  pub fn control() -> DMAControlSetting {
    Self::DMA1CNT_H.read()
  }

  /// Writes the control setting given.
  ///
  /// # Safety
  ///
  /// You must ensure that the Source, Destination, and Count values are set
  /// correctly **before** you activate the Enable bit.
  pub unsafe fn set_control(setting: DMAControlSetting) {
    Self::DMA1CNT_H.write(setting)
  }
}

pub struct DMA2;
impl DMA2 {
  /// DMA 2 Source Address, read only.
  const DMA2SAD: VolAddress<*const u32> = unsafe { VolAddress::new(0x400_00C8) };
  /// DMA 2 Destination Address, read only.
  const DMA2DAD: VolAddress<*mut u32> = unsafe { VolAddress::new(0x400_00CC) };
  /// DMA 2 Word Count, read only.
  const DMA2CNT_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_00D0) };
  /// DMA 2 Control, read/write.
  const DMA2CNT_H: VolAddress<DMAControlSetting> = unsafe { VolAddress::new(0x400_00D2) };

  /// Assigns the source register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The source pointer must be aligned and valid to read from.
  pub unsafe fn set_source(src: *const u32) {
    Self::DMA2SAD.write(src)
  }

  /// Assigns the destination register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The source pointer must be aligned and valid to write to.
  pub unsafe fn set_dest(dest: *mut u32) {
    Self::DMA2DAD.write(dest)
  }

  /// Assigns the count register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The count given must specify a valid number of units to write, starting at
  /// the assigned destination address.
  pub unsafe fn set_count(count: u16) {
    Self::DMA2CNT_L.write(count)
  }

  /// Reads the current control setting.
  pub fn control() -> DMAControlSetting {
    Self::DMA2CNT_H.read()
  }

  /// Writes the control setting given.
  ///
  /// # Safety
  ///
  /// You must ensure that the Source, Destination, and Count values are set
  /// correctly **before** you activate the Enable bit.
  pub unsafe fn set_control(setting: DMAControlSetting) {
    Self::DMA2CNT_H.write(setting)
  }
}

/// This is the "general purpose" DMA unit, with the fewest limits.
pub struct DMA3;
impl DMA3 {
  /// DMA 3 Source Address, read only.
  const DMA3SAD: VolAddress<*const u32> = unsafe { VolAddress::new(0x400_00D4) };
  /// DMA 3 Destination Address, read only.
  const DMA3DAD: VolAddress<*mut u32> = unsafe { VolAddress::new(0x400_00D8) };
  /// DMA 3 Word Count, read only.
  const DMA3CNT_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_00DC) };
  /// DMA 3 Control, read/write.
  const DMA3CNT_H: VolAddress<DMAControlSetting> = unsafe { VolAddress::new(0x400_00DE) };

  /// Assigns the source register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The source pointer must be aligned and valid to read from.
  pub unsafe fn set_source(src: *const u32) {
    Self::DMA3SAD.write(src)
  }

  /// Assigns the destination register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The source pointer must be aligned and valid to write to.
  pub unsafe fn set_dest(dest: *mut u32) {
    Self::DMA3DAD.write(dest)
  }

  /// Assigns the count register.
  ///
  /// This register is read only, so it is not exposed directly.
  ///
  /// # Safety
  ///
  /// The count given must specify a valid number of units to write, starting at
  /// the assigned destination address.
  pub unsafe fn set_count(count: u16) {
    Self::DMA3CNT_L.write(count)
  }

  /// Reads the current control setting.
  pub fn control() -> DMAControlSetting {
    Self::DMA3CNT_H.read()
  }

  /// Writes the control setting given.
  ///
  /// # Safety
  ///
  /// You must ensure that the Source, Destination, and Count values are set
  /// correctly **before** you activate the Enable bit.
  pub unsafe fn set_control(setting: DMAControlSetting) {
    Self::DMA3CNT_H.write(setting)
  }

  /// Fills `count` slots (starting at `dest`) with the value at `src`.
  ///
  /// # Safety
  ///
  /// Both pointers must be aligned, and all positions specified for writing
  /// must be valid for writing.
  pub unsafe fn fill32(src: *const u32, dest: *mut u32, count: u16) {
    const FILL_CONTROL: DMAControlSetting = DMAControlSetting::new()
      .with_source_address_control(DMASrcAddressControl::Fixed)
      .with_use_32bit(true)
      .with_enabled(true);
    // TODO: destination checking against SRAM
    Self::DMA3SAD.write(src);
    Self::DMA3DAD.write(dest);
    Self::DMA3CNT_L.write(count);
    Self::DMA3CNT_H.write(FILL_CONTROL);
    // Note(Lokathor): Once DMA is set to activate it takes 2 cycles for it to
    // kick in. You can do any non-DMA thing you like before that, but since
    // it's only two cycles we just insert two NOP instructions to ensure that
    // successive calls to `fill32` or other DMA methods don't interfere with
    // each other.
    asm!(
      "
        NOP
        NOP
    ",
      options(nomem, nostack)
    );
  }
}
