//! A module that produces the marker strings used by emulators to determine
//! which SRAM type a ROM uses.
//!
//! This takes advantage of the LLVM's usual dead code elimination. The
//! functions that generate the markers use `volatile_mark_read` to force the
//! LLVM to assume the statics used. Therefore, as long as one of these
//! functions is called, the corresponding static is emitted with no actual
//! code generated.

#[repr(align(4))]
struct Align<T>(T);

static EEPROM: Align<[u8; 12]> = Align(*b"EEPROM_Vnnn\0");
static SRAM: Align<[u8; 12]> = Align(*b"SRAM_Vnnn\0\0\0");
static FLASH512K: Align<[u8; 16]> = Align(*b"FLASH512_Vnnn\0\0\0");
static FLASH1M: Align<[u8; 16]> = Align(*b"FLASH1M_Vnnn\0\0\0\0");

#[inline(always)]
pub fn emit_eeprom_marker() {
    crate::sync::volatile_mark_ro(&EEPROM);
}
#[inline(always)]
pub fn emit_sram_marker() {
    crate::sync::volatile_mark_ro(&SRAM);
}
#[inline(always)]
pub fn emit_flash_512k_marker() {
    crate::sync::volatile_mark_ro(&FLASH512K);
}
#[inline(always)]
pub fn emit_flash_1m_marker() {
    crate::sync::volatile_mark_ro(&FLASH1M);
}