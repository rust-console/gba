//! A module that produces the marker strings used by emulators to determine
//! which save media type a ROM uses.
//!
//! This takes advantage of the LLVM's usual dead code elimination. The
//! functions that generate the markers use `volatile_mark_read` to force the
//! LLVM to assume the statics used. Therefore, as long as one of these
//! functions is called, the corresponding static is emitted with very little
//! code actually generated.

use super::*;

#[repr(align(4))]
struct Align<T>(T);

static EEPROM: Align<[u8; 12]> = Align(*b"EEPROM_Vnnn\0");
static SRAM: Align<[u8; 12]> = Align(*b"SRAM_Vnnn\0\0\0");
static FLASH512K: Align<[u8; 16]> = Align(*b"FLASH512_Vnnn\0\0\0");
static FLASH1M: Align<[u8; 16]> = Align(*b"FLASH1M_Vnnn\0\0\0\0");

#[inline(always)]
fn emit_eeprom_marker() {
  crate::sync::memory_read_hint(&EEPROM);
}
#[inline(always)]
fn emit_sram_marker() {
  crate::sync::memory_read_hint(&SRAM);
}
#[inline(always)]
fn emit_flash_512k_marker() {
  crate::sync::memory_read_hint(&FLASH512K);
}
#[inline(always)]
fn emit_flash_1m_marker() {
  crate::sync::memory_read_hint(&FLASH1M);
}

/// Declares that the ROM uses battery backed SRAM/FRAM.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// Battery Backed SRAM is generally very fast, but limited in size compared
/// to flash chips.
pub fn use_sram() {
  emit_sram_marker();
  set_save_implementation(Some(&sram::BatteryBackedAccess));
}

/// Declares that the ROM uses 64KiB flash memory.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// Flash save media is generally very slow to write to and relatively fast
/// to read from. It is the only real option if you need larger save data.
pub fn use_flash_64k() {
  emit_flash_512k_marker();
  set_save_implementation(Some(&flash::FlashAccess));
}

/// Declares that the ROM uses 128KiB flash memory.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// Flash save media is generally very slow to write to and relatively fast
/// to read from. It is the only real option if you need larger save data.
pub fn use_flash_128k() {
  emit_flash_1m_marker();
  set_save_implementation(Some(&flash::FlashAccess));
}

/// Declares that the ROM uses 512 bytes EEPROM memory.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// EEPROM is generally pretty slow and also very small. It's mainly used in
/// Game Paks because it's cheap.
pub fn use_eeprom_512b() {
  emit_eeprom_marker();
  set_save_implementation(Some(&eeprom::Eeprom512B));
}

/// Declares that the ROM uses 8 KiB EEPROM memory.
///
/// This creates a marker in the ROM that allows emulators to understand what
/// save type the Game Pak uses, and sets the accessor to one appropriate for
/// memory type.
///
/// EEPROM is generally pretty slow and also very small. It's mainly used in
/// Game Paks because it's cheap.
pub fn use_eeprom_8k() {
  emit_eeprom_marker();
  set_save_implementation(Some(&eeprom::Eeprom8K));
}
