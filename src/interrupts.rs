use crate::mmio_types::InterruptFlags;

/// A function pointer for use as an interrupt handler.
pub type InterruptHandler = extern "C" fn(InterruptFlags);

/// Sets the function to run when an interrupt is executed. The function will
/// receive the interrupts that were acknowledged by the main interrupt handler
/// as an argument.
///
/// NOTE: This function *must* use the Thumb instruction set, by annotating it
/// with `#[instruction_set(arm::t32)]`.
pub fn set_interrupt_handler(handler: InterruptHandler) {
  unsafe {
    __IRQ_HANDLER = handler;
  }
}

/// The default interrupt handler (no-op).
#[instruction_set(arm::t32)]
pub extern "C" fn default_interrupt_handler(_flags: InterruptFlags) {}

// Inner definition of the interrupt handler. It is referenced in `crt0.s`.
#[doc(hidden)]
#[no_mangle]
static mut __IRQ_HANDLER: InterruptHandler = default_interrupt_handler;
