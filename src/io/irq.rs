//! Module containing a wrapper for interrupt request (IRQ) handling.
//!
//! When an interrupt is executed, the CPU will be set to IRQ mode and code
//! execution will jump to the physical interrupt vector, located in BIOS. The
//! BIOS interrupt handler will then save several registers to the IRQ stack
//! pointer and execution will jump to the user interrupt handler starting at
//! `0x0300_7FFC`, in ARM mode.
//!
//! Currently, the user interrupt handler is defined in `crt0.s`. It is set up
//! to execute a user-specified interrupt handler after saving some registers.
//! This handler is declared as a static function pointer on the Rust side, and
//! can be set by using [`set_irq_handler`](irq::set_irq_handler).
//!
//! ## Notes
//! * The interrupt will only be triggered if [`IME`](irq::IME) is enabled, the
//!   flag corresponding to the interrupt is enabled on the [`IE`](irq::IE)
//!   register, and the "IRQ Enable" flag is set on the register related to the
//!   interrupt, which varies. For example, to enable interrupts on VBlank you
//!   would set the
//!   [`vblank_irq_enable`](io::display::DisplayStatusSetting::vblank_irq_enable)
//!   flag on the [`DISPSTAT`](io::display::DISPCNT) register.
//! * If you intend to use [`interrupt_wait`](bios::interrupt_wait) or
//!   [`vblank_interrupt_wait`](bios::vblank_interrupt_wait) to wait for an
//!   interrupt, your interrupt handler MUST update the BIOS Interrupt Flags at
//!   [`BIOS_IF`](irq::BIOS_IF) in addition to the usual interrupt
//!   acknowledgement (which is handled for you by the user interrupt handler).
//!   This is done by setting the corresponding IRQ flag on
//!   [`BIOS_IF`](irq::BIOS_IF) at the end of the interrupt handler.
//! * You can change the low-level details of the interrupt handler by editing
//!   the `MainIrqHandler` routine in `crt0.s`. For example, you could declare
//!   an external static variable in Rust holding a table of interrupt function
//!   pointers and jump directly into one of them in assembly, without the need
//!   to write the branching logic in Rust. However, note that the main
//!   interrupt handler MUST acknowledge all interrupts received by setting
//!   their corresponding bits to `1` in the [`IF`](irq::IF) register.
//! * If you wait on one or more interrupts, be sure at least one of them is
//!   able to be triggered or the call to wait will never return.
//! * If you wait on multiple interrupts and those interrupts fire too quickly,
//!   it is possible that the call to wait will never return as interrupts will
//!   be constantly received before control is returned to the caller. This
//!   usually only happens when waiting on multiple timer interrupts with very
//!   fast overflow rates.
//!
//! ## Example
//!
//! ```rust
//! extern "C" fn irq_handler(flags: IrqFlags) {
//!   if flags.vblank() {
//!     // Run drawing logic here.
//!
//!     // Acknowledge the IRQ on the BIOS Interrupt Flags register.
//!     BIOS_IF.write(BIOS_IF.read().with_vblank(true));
//!   }
//! }
//!
//! fn main_loop() {
//!   // Set the IRQ handler to use.
//!   irq::set_irq_handler(irq_handler);
//!
//!   // Handle only the VBlank interrupt.
//!   const FLAGS: IrqFlags = IrqFlags::new().with_vblank(true);
//!   IE.write(flags);
//!
//!   // Enable all interrupts that are set in the IE register.
//!   IME.write(IrqEnableSetting::UseIE);
//!
//!   // Enable IRQ generation during VBlank.
//!   const DISPLAY_SETTINGS: DisplayStatusSetting = DisplayStatusSetting::new()
//!     .with_vblank_irq_enable(true);
//!   DISPSTAT.write(DISPLAY_SETTINGS);
//!
//!   loop {
//!     // Sleep the CPU until a VBlank IRQ is generated.
//!     bios::vblank_interrupt_wait();
//!   }
//! }
//! ```
//!
//! ## Implementation Details
//!
//! This is the setup the provided user interrupt handler in `crt0.s` will do
//! when an interrupt is received, in order. It is based on the _Recommended
//! User Interrupt Handling_ portion of the GBATEK reference.
//!
//! 1. Save the status of [`IME`](irq::IME).
//! 2. Save the IRQ stack pointer and change to system mode to use the user
//!    stack instead of the IRQ stack (to prevent stack overflow).
//! 3. Disable interrupts by setting [`IME`](irq::IME) to 0, so other interrupts
//!    will not preempt the main interrupt handler.
//! 4. Acknowledge all IRQs that occurred and were enabled in the
//!    [`IE`](irq::IE) register by writing the bits to the [`IF`](irq::IF)
//!    register.
//! 5. Save the user stack pointer, switch to Thumb mode and jump to the
//!    user-specified interrupt handler. The IRQ flags that were set are passed
//!    as an argument in `r0`.
//! 6. When the handler returns, restore the user stack pointer and switch back
//!    to IRQ mode.
//! 7. Restore the IRQ stack pointer and the status of [`IME`](irq::IME).
//! 8. Return to the BIOS interrupt handler.

use super::*;

newtype!(
  /// A newtype over all interrupt flags.
  IrqFlags,
  u16
);

impl IrqFlags {
  phantom_fields! {
    self.0: u16,
    vblank: 0,
    hblank: 1,
    vcounter: 2,
    timer0: 3,
    timer1: 4,
    timer2: 5,
    timer3: 6,
    serial: 7,
    dma0: 8,
    dma1: 9,
    dma2: 10,
    dma3: 11,
    keypad: 12,
    game_pak: 13,
  }
}

/// Interrupt Enable Register. Read/Write.
///
/// After setting up interrupt handlers, set the flags on this register type corresponding to the
/// IRQs you want to handle.
pub const IE: VolAddress<IrqFlags> = unsafe { VolAddress::new(0x400_0200) };

/// Interrupt Request Flags / IRQ Acknowledge. Read/Write.
///
/// The main user interrupt handler will acknowledge the interrupt that was set
/// by writing to this register, so there is usually no need to modify it.
/// However, if the main interrupt handler in `crt0.s` is changed, then the
/// handler must write a `1` bit to all bits that are enabled on this register
/// when it is called.
pub const IF: VolAddress<IrqFlags> = unsafe { VolAddress::new(0x400_0202) };

newtype! {
    /// Setting to control whether interrupts are enabled.
    IrqEnableSetting, u16
}

impl IrqEnableSetting {
  phantom_fields! {
    self.0: u16,
    /// System-wide control for if interrupts of all kinds are enabled or not.
    interrupts_enabled: 0,
  }

  /// Yes, you want to have interrupts.
  pub const IRQ_YES: Self = Self::new().with_interrupts_enabled(true);

  /// No, you do not want to have interrupts.
  pub const IRQ_NO: Self = Self::new();
}

/// Interrupt Master Enable Register. Read/Write.
pub const IME: VolAddress<IrqEnableSetting> = unsafe { VolAddress::new(0x400_0208) };

/// BIOS Interrupt Flags. Read/Write.
///
/// When using either [`interrupt_wait`](bios::interrupt_wait) or
/// [`vblank_interrupt_wait`](bios::vblank_interrupt_wait), the corresponding
/// interrupt handler MUST set the flag of the interrupt it has handled on this
/// register in addition to the usual interrupt acknowledgement.
pub const BIOS_IF: VolAddress<IrqFlags> = unsafe { VolAddress::new(0x0300_7FF8) };

/// A function pointer for use as an interrupt handler.
pub type IrqHandler = extern "C" fn(IrqFlags);

/// Sets the function to run when an interrupt is executed. The function will
/// receive the interrupts that were acknowledged by the main interrupt handler
/// as an argument.
pub fn set_irq_handler(handler: IrqHandler) {
  unsafe {
    __IRQ_HANDLER = handler;
  }
}

extern "C" fn default_handler(_flags: IrqFlags) {}

// Inner definition of the interrupt handler. It is referenced in `crt0.s`.
#[doc(hidden)]
#[no_mangle]
static mut __IRQ_HANDLER: IrqHandler = default_handler;
