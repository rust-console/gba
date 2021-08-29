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
struct IrqOffsets {
    base: usize,
    mask: u16
}

static OFFSETS: [IrqOffsets; 14] = [
    IrqOffsets { base: 0x0004, mask: 0x0008 }, // VBlank  DISPSTAT
    IrqOffsets { base: 0x0004, mask: 0x0010 }, // HBlank  DISPSTAT
    IrqOffsets { base: 0x0004, mask: 0x0020 }, // VCount  DISPSTAT
    IrqOffsets { base: 0x0102, mask: 0x0040 }, // Timer0  TM0CNT
    IrqOffsets { base: 0x0106, mask: 0x0040 }, // Timer1  TM1CNT
    IrqOffsets { base: 0x010A, mask: 0x0040 }, // Timer2  TM2CNT
    IrqOffsets { base: 0x010E, mask: 0x0040 }, // Timer3  TM3CNT
    IrqOffsets { base: 0x0128, mask: 0x4000 }, // Serial  SIOCNT
    IrqOffsets { base: 0x00BA, mask: 0x4000 }, // DMA0    DMA0CNT
    IrqOffsets { base: 0x00C6, mask: 0x4000 }, // DMA1    DMA1CNT
    IrqOffsets { base: 0x00D2, mask: 0x4000 }, // DMA2    DMA2CNT
    IrqOffsets { base: 0x00DE, mask: 0x4000 }, // DMA3    DMA3CNT
    IrqOffsets { base: 0x0132, mask: 0x4000 }, // Keypad  KEYCNT
    IrqOffsets { base: 0x0000, mask: 0x0000 }, // Gamepak (none)
];

#[inline]
unsafe fn set_irq_flag(kind: IrqKind) {
    let offset = &OFFSETS[kind as usize];
    let addr: *mut u16 = core::mem::transmute(REG_BASE + offset.base);
    *addr |= offset.mask;
    IE.write(InterruptFlags(IE.read().0 | (1 << (kind as u8))))
}

#[inline]
unsafe fn unset_irq_flag(kind: IrqKind) {
    let offset = &OFFSETS[kind as usize];
    let addr: *mut u16 = core::mem::transmute(REG_BASE + offset.base);
    *addr &= !offset.mask;
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

const REG_BASE: usize = 0x0400_0000;

/// Sets the Interrupt Master Enable flag. This must be set to `true` in order
/// for any interrupts to be triggered.
pub fn set_master_enabled(enabled: bool) {
    unsafe { IME.write(enabled) };
}

/// Clears all registered interrupt handlers.
pub fn clear_all_handlers() {
    with_no_irqs! {
        unsafe {
            __IRQ_HANDLERS = [None; 14];
        }
    }
}

/// Registers a function to handle the specified interrupt kind. To unset the
/// handler for the interrupt, pass `None` instead.
///
/// **Note:** The function *must* be compiled as Thumb code by annotating it with
pub fn set_handler(kind: IrqKind, handler: Option<IrqHandler>) {
    with_no_irqs! {
        unsafe {
            __IRQ_HANDLERS[kind as usize] = handler;
        }
    }
}

pub fn enable(kind: IrqKind) {
    with_no_irqs! {
        unsafe {
            set_irq_flag(kind);
        }
    }
}

pub fn disable(kind: IrqKind) {
    with_no_irqs! {
        unsafe {
            unset_irq_flag(kind);
        }
    }
}
