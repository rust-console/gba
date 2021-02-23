//! Special utilities for debugging ROMs on various emulators.
//!
//! This is the underlying implementation behind the various print macros in
//! the gba crate. It currently supports the latest versions of mGBA and NO$GBA.

use crate::{
  io::{
    dma::{DMAControlSetting, DMA0, DMA1, DMA2, DMA3},
    irq::{IrqEnableSetting, IME},
  },
  sync::{InitOnce, RawMutex, Static},
};
use core::fmt::{Arguments, Error};
use voladdress::VolAddress;

pub mod mgba;
pub mod nocash;

/// A cross-emulator debug level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DebugLevel {
  /// This causes the emulator (or debug interface) to halt!
  Fatal,
  Error,
  Warning,
  Info,
  Debug,
}

/// An interface for debugging features.
pub trait DebugInterface {
  /// Whether debugging is enabled.
  fn device_attached(&self) -> bool;

  /// Prints a debug message to the emulator.
  fn debug_print(&self, debug: DebugLevel, args: &Arguments<'_>) -> Result<(), Error>;
}

/// An lock to ensure interface changes go smoothly.
static LOCK: RawMutex = RawMutex::new();
/// An optimization to allow us to short circuit debugging early when there is no interface.
static NO_DEBUG: Static<bool> = Static::new(false);
/// The debugging interface in use.
static INTERFACE: Static<Option<&'static dyn DebugInterface>> = Static::new(None);
/// Debug interface detection only happens once.
static DETECT_ONCE: InitOnce<()> = InitOnce::new();

/// Sets the debug interface in use manually.
pub fn set_debug_interface(interface: &'static dyn DebugInterface) {
  let _lock = LOCK.lock();
  INTERFACE.write(Some(interface));
  NO_DEBUG.write(false);
}

/// Disables debugging.
pub fn set_debug_disabled() {
  let _lock = LOCK.lock();
  INTERFACE.write(None);
  NO_DEBUG.write(true);
}

/// Prints a line to the debug interface, if there is any.
#[inline(never)]
pub fn debug_print(debug: DebugLevel, args: &Arguments<'_>) -> Result<(), Error> {
  if let Some(interface) = get_debug_interface() {
    interface.debug_print(debug, args)?;
  }
  Ok(())
}

/// Returns the current active debugging interface if there is one, or `None`
/// if one isn't attached.
#[inline(never)]
pub fn get_debug_interface() -> Option<&'static dyn DebugInterface> {
  let mut interface = INTERFACE.read();
  if interface.is_none() {
    DETECT_ONCE.get(|| {
      let mut new_value: Option<&'static dyn DebugInterface> = None;
      if mgba::detect() {
        new_value = Some(&mgba::MGBADebugInterface);
      } else if nocash::detect() {
        new_value = Some(&nocash::NoCashDebugInterface);
      }
      if new_value.is_some() {
        INTERFACE.write(new_value);
        interface = new_value;
      }
    });
  }
  interface
}

/// Whether debugging is disabled.
///
/// This should only be relied on for correctness. If this is false, there is no
/// possible way any debugging calls will succeed, and it is better to simply
/// skip the entire routine.
#[inline(always)]
pub fn is_debugging_disabled() -> bool {
  NO_DEBUG.read()
}

/// Crashes the program by disabling interrupts and entering an infinite loop.
///
/// This is used to implement fatal errors outside of mGBA.
#[inline(never)]
pub fn crash() -> ! {
  #[cfg(all(target_vendor = "nintendo", target_env = "agb"))]
  {
    IME.write(IrqEnableSetting::IRQ_NO);
    unsafe {
      // Stop all ongoing DMAs just in case.
      DMA0::set_control(DMAControlSetting::new());
      DMA1::set_control(DMAControlSetting::new());
      DMA2::set_control(DMAControlSetting::new());
      DMA3::set_control(DMAControlSetting::new());

      // Writes the halt call back to memory
      //
      // we use an infinite loop in RAM just to make sure removing the
      // Game Pak doesn't break this crash loop.
      let target = VolAddress::<u16>::new(0x03000000);
      target.write(0xe7fe); // assembly instruction: `loop: b loop`
      core::mem::transmute::<_, extern "C" fn() -> !>(0x03000001)()
    }
  }

  #[cfg(not(all(target_vendor = "nintendo", target_env = "agb")))]
  loop { }
}
