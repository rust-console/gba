//! A module containing support for battery backed SRAM.

use super::{Error, SramAccess, read_raw_buf, write_raw_buf};
use typenum::consts::U32768;
use voladdress::VolBlock;

const SRAM_SIZE: usize = 32 * 1024; // 32 KiB

fn check_bounds(offset: usize, len: usize) -> Result<(), Error> {
    if offset.checked_add(len).is_none() ||
        offset + len > SRAM_SIZE
    {
        return Err(Error::OutOfRange)
    }
    Ok(())
}

/// The [`SramAccess`] used for battery backed SRAM.
pub struct BatteryBackedAccess;
impl SramAccess for BatteryBackedAccess {
    fn len(&self) -> usize {
        SRAM_SIZE
    }

    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error> {
        check_bounds(offset, buffer.len())?;
        unsafe { read_raw_buf(buffer, 0x0E000000 + offset); }
        Ok(())
    }

    fn write(&self, offset: usize, buffer: &[u8], _exact: bool) -> Result<(), Error> {
        check_bounds(offset, buffer.len())?;
        unsafe { write_raw_buf(0x0E000000 + offset, buffer); }
        Ok(())
    }

    fn get_sram_write_ranges(&self, offset: usize, len: usize) -> Result<(usize, usize), Error> {
        Ok((offset, offset + len))
    }
}

/// A static instance of a [`SramAccess`] appropriate for battery backed SRAM.
pub static ACCESS: &'static dyn SramAccess = &BatteryBackedAccess;