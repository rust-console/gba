//! Interrupt handling.
//!
//! Set an interrupt handler by calling
//! [`irq::set_handler()`](crate::irq::set_handler()) for each interrupt you
//! want to respond to, then call
//! [`irq::set_master_enabled()`](crate::irq::set_master_enabled()) to allow
//! interrupts to be raised. All handler functions *must* be compiled as Thumb
//! code by annotating them with `#[instruction_set(arm::t32)]`.
//!
//! Beneath the hood, a top-level "switchboard" interrupt handler written in
//! assembly takes care of dispatching to each user-defined handler function.
//! This top-level handler also allows for nested interrupts. When a
//! user-defined interrupt handler is about to be called, the main handler will
//! first clear the [`IE`] register and save the state of the [`IE`],
//! [`IF`](crate::mmio_addresses::IRQ_PENDING) and [`IME`] registers. To handle
//! nested interrupts, write the [`InterruptFlags`] for the interrupt kinds you
//! want to the [`IE`] register from within the appropriate handler function.
//!
//! # Example
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//! #![feature(isa_attribute)]
//!
//! use gba::{prelude::*, info};
//!
//! #[panic_handler]
//! fn panic(_info: &core::panic::PanicInfo) -> ! {
//!   loop {}
//! }
//!
//! #[no_mangle]
//! fn main() -> ! {
//!     irq::set_handler(IrqKind::VBlank, Some(vblank_handler));
//!     irq::set_handler(IrqKind::HBlank, Some(hblank_handler));
//!     irq::enable(IrqKind::VBlank);
//!     irq::enable(IrqKind::HBlank);
//!     irq::set_master_enabled(true);
//!
//!     loop {
//!         unsafe { VBlankIntrWait() };
//!     }
//! }
//!
//! #[instruction_set(arm::t32)]
//! extern "C" fn vblank_handler() {
//!     info!("VBlank received.");
//!
//!     // Allow nested HBlank IRQs.
//!     unsafe { IE.write(IE.read().with_hblank(true)); }
//! }
//!
//! #[instruction_set(arm::t32)]
//! extern "C" fn hblank_handler() {
//!     info!("HBlank received.");
//! }
//! ```

use crate::{__IRQ_HANDLERS, IrqHandler};
use crate::mmio_types::InterruptFlags;
use crate::mmio_addresses::{IME, IE};

/// The types of interrupts that can be handled.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum IrqKind {
    VBlank = 0,
    HBlank,
    VCount,
    Timer0,
    Timer1,
    Timer2,
    Timer3,
    Serial,
    DMA0,
    DMA1,
    DMA2,
    DMA3,
    Keypad,
    Gamepak,
}

/// Offset from I/O address space to relevant MMIO address and the bitmask to
/// set/unset to enable/disable an interrupt.
struct IrqSender {
    offset: usize,
    mask: u16
}

static SENDERS: [IrqSender; 14] = [
    IrqSender { offset: 0x0004, mask: 0x0008 }, // VBlank  DISPSTAT
    IrqSender { offset: 0x0004, mask: 0x0010 }, // HBlank  DISPSTAT
    IrqSender { offset: 0x0004, mask: 0x0020 }, // VCount  DISPSTAT
    IrqSender { offset: 0x0102, mask: 0x0040 }, // Timer0  TM0CNT
    IrqSender { offset: 0x0106, mask: 0x0040 }, // Timer1  TM1CNT
    IrqSender { offset: 0x010A, mask: 0x0040 }, // Timer2  TM2CNT
    IrqSender { offset: 0x010E, mask: 0x0040 }, // Timer3  TM3CNT
    IrqSender { offset: 0x0128, mask: 0x4000 }, // Serial  SIOCNT
    IrqSender { offset: 0x00BA, mask: 0x4000 }, // DMA0    DMA0CNT
    IrqSender { offset: 0x00C6, mask: 0x4000 }, // DMA1    DMA1CNT
    IrqSender { offset: 0x00D2, mask: 0x4000 }, // DMA2    DMA2CNT
    IrqSender { offset: 0x00DE, mask: 0x4000 }, // DMA3    DMA3CNT
    IrqSender { offset: 0x0132, mask: 0x4000 }, // Keypad  KEYCNT
    IrqSender { offset: 0x0000, mask: 0x0000 }, // Gamepak (none)
];

const MMIO_BASE_ADDRESS: usize = 0x0400_0000;

#[inline]
unsafe fn set_irq_flag(kind: IrqKind) {
    let sender = &SENDERS[kind as usize];
    let addr: *mut u16 = core::mem::transmute(MMIO_BASE_ADDRESS + sender.offset);
    *addr |= sender.mask;
    IE.write(InterruptFlags(IE.read().0 | (1 << (kind as u8))))
}

#[inline]
unsafe fn unset_irq_flag(kind: IrqKind) {
    let sender = &SENDERS[kind as usize];
    let addr: *mut u16 = core::mem::transmute(MMIO_BASE_ADDRESS + sender.offset);
    *addr &= !sender.mask;
    IE.write(InterruptFlags(IE.read().0 & !(1 << (kind as u8))))
}

macro_rules! with_no_irqs {
  ($body:expr) => {{
      // Save IME and disable interrupts
      let ime = IME.read();
      unsafe { IME.write(false); }

      $body

      // Restore IME
      unsafe { IME.write(ime); }
  }};
}

/// Sets the Interrupt Master Enable flag. This must be set to `true` in order
/// for any interrupts to be triggered.
pub fn set_master_enabled(enabled: bool) {
    unsafe { IME.write(enabled) };
}

/// Clears all registered interrupt handlers.
pub fn clear_all_handlers() {
    with_no_irqs! {
        unsafe { __IRQ_HANDLERS = [None; 14]; }
    }
}

/// Registers a function to handle the specified interrupt kind. To unset the
/// handler for the interrupt, pass `None` instead.
///
/// **Note:** The function *must* be compiled as Thumb code by annotating it
/// with `#[instruction_set(arm::t32)]`.
pub fn set_handler(kind: IrqKind, handler: Option<IrqHandler>) {
    with_no_irqs! {
        unsafe { __IRQ_HANDLERS[kind as usize] = handler; }
    }
}

pub fn enable(kind: IrqKind) {
    with_no_irqs! {
        unsafe { set_irq_flag(kind); }
    }
}

pub fn disable(kind: IrqKind) {
    with_no_irqs! {
        unsafe { unset_irq_flag(kind); }
    }
}
