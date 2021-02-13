//! A module containing support for battery backed SRAM.
//!
//! ## API
//!
//! Battery-backed SRAM does not have an API beyond [`BatteryBackedAccess`], due
//! to the simplicity of the memory type.
//!
//! ## Technical details
//!
//! Battery-backed SRAM is the simplest kind of SRAM to read from. The only real
//! complication is the requirement to read from it with code located in WRAM,
//! which we use [`read_raw_buf`] for. Otherwise, it is simply a big block of
//! memory mapped into the address space.

use super::{Error, SramAccess, SramType, read_raw_buf, write_raw_buf};
use crate::sram::verify_raw_buf;

const SRAM_SIZE: usize = 32 * 1024; // 32 KiB

/// Checks whether an offset is contained within the bounds of the SRAM.
fn check_bounds(offset: usize, len: usize) -> Result<(), Error> {
    if offset.checked_add(len).is_none() || offset + len > SRAM_SIZE {
        return Err(Error::OutOfRange)
    }
    Ok(())
}

/// The [`SramAccess`] used for battery backed SRAM.
pub struct BatteryBackedAccess;
impl SramAccess for BatteryBackedAccess {
    fn sram_type(&self) -> Result<SramType, Error> {
        Ok(SramType::Sram32K)
    }

    fn len(&self) -> Result<usize, Error> {
        Ok(SRAM_SIZE)
    }

    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error> {
        check_bounds(offset, buffer.len())?;
        unsafe { read_raw_buf(buffer, 0x0E000000 + offset); }
        Ok(())
    }

    fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error> {
        check_bounds(offset, buffer.len())?;
        let val = unsafe { verify_raw_buf(buffer, 0x0E000000 + offset) };
        Ok(val)
    }

    fn write_raw(&self, offset: usize, buffer: &[u8], _exact: bool) -> Result<(), Error> {
        check_bounds(offset, buffer.len())?;
        unsafe { write_raw_buf(0x0E000000 + offset, buffer); }
        Ok(())
    }

    fn sector_shift(&self) -> Result<usize, Error> {
        Ok(15)
    }
}
