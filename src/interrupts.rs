use crate::mmio_types::InterruptFlags;
use crate::mmio_addresses::{IME, IE};

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

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum InterruptKind {
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

struct InterruptOffsets {
    base: usize,
    mask: u16
}

static OFFSETS: [InterruptOffsets; 14] = [
    InterruptOffsets { base: 0x0004, mask: 0x0008 }, // DISPSTAT
    InterruptOffsets { base: 0x0004, mask: 0x0010 }, // DISPSTAT
    InterruptOffsets { base: 0x0004, mask: 0x0020 }, // DISPSTAT
    InterruptOffsets { base: 0x0102, mask: 0x0040 }, // TM0CNT
    InterruptOffsets { base: 0x0106, mask: 0x0040 }, // TM1CNT
    InterruptOffsets { base: 0x010A, mask: 0x0040 }, // TM2CNT
    InterruptOffsets { base: 0x010E, mask: 0x0040 }, // TM3CNT
    InterruptOffsets { base: 0x0128, mask: 0x4000 }, // SIOCNT
    InterruptOffsets { base: 0x00BA, mask: 0x4000 }, // DMA0CNT
    InterruptOffsets { base: 0x00C6, mask: 0x4000 }, // DMA1CNT
    InterruptOffsets { base: 0x00D2, mask: 0x4000 }, // DMA2CNT
    InterruptOffsets { base: 0x00DE, mask: 0x4000 }, // DMA3CNT
    InterruptOffsets { base: 0x0132, mask: 0x4000 }, // KEYCNT
    InterruptOffsets { base: 0x0000, mask: 0x0000 }, // Gamepak (none)
];

const REG_BASE: usize = 0x0400_0000;

pub fn enable(kind: InterruptKind) {
    // Disable interrupts
    let ime = IME.read();
    unsafe { IME.write(false); }

    unsafe {
        let offset = &OFFSETS[kind as usize];
        let addr: *mut u16 = core::mem::transmute(REG_BASE + offset.base);
        *addr |= offset.mask;
        IE.write(InterruptFlags(IE.read().0 | (1 << (kind as u8))))
    }

    // Restore IME
    unsafe { IME.write(ime); }
}

pub fn disable(kind: InterruptKind) {
    // Disable interrupts
    let ime = IME.read();
    unsafe { IME.write(false); }

    unsafe {
        let offset = &OFFSETS[kind as usize];
        let addr: *mut u16 = core::mem::transmute(REG_BASE + offset.base);
        *addr &= !offset.mask;
        IE.write(InterruptFlags(IE.read().0 & !(1 << (kind as u8))))
    }

    // Restore IME
    unsafe { IME.write(ime); }
}

#[doc(hidden)]
#[no_mangle]
static mut __IRQ_HANDLERS: InterruptHandler = default_interrupt_handler;
