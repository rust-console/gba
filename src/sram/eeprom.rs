//! A module containing support for EEPROM.

use core::cmp;
use core::ops::Range;
use crate::io::dma::*;
use crate::sram::{Timeout, lock_sram};
use crate::sync::{RawMutex, disable_irqs};
use super::{Error, SramAccess, SramType, read_raw_buf, write_raw_buf};
use voladdress::VolAddress;

enum EepromSize {
    Eeprom512B,
    Eeprom8K,
}

const PORT: VolAddress<u16> = unsafe { VolAddress::new(0x0DFFFF00) };
const SECTOR_SHIFT: usize = 3;
const SECTOR_LEN: usize = 1 << SECTOR_SHIFT;

/// Disable IRQs and DMAs during each read block.
fn disable_dmas(func: impl FnOnce()) {
    disable_irqs(|| unsafe {
        let dma0_ctl = DMA0::control();
        let dma1_ctl = DMA1::control();
        let dma2_ctl = DMA2::control();
        DMA0::set_control(dma0_ctl.with_enabled(false));
        DMA1::set_control(dma1_ctl.with_enabled(false));
        DMA2::set_control(dma2_ctl.with_enabled(false));

        func();

        DMA0::set_control(dma0_ctl);
        DMA1::set_control(dma1_ctl);
        DMA2::set_control(dma2_ctl);
    });
}

/// Sends a DMA command to EEPROM.
fn dma_send(source: &[u32], ct: u16) {
    disable_dmas(|| unsafe {
        DMA3::set_source(source.as_ptr());
        DMA3::set_dest(0x0DFFFF00 as *mut _);
        DMA3::set_count(ct);
        let dma3_ctl = DMAControlSetting::new()
            .with_dest_address_control(DMADestAddressControl::Increment)
            .with_source_address_control(DMASrcAddressControl::Increment)
            .with_enabled(true);
        DMA3::set_control(dma3_ctl);
    });
}

/// Receives a DMA packet from EEPROM.
fn dma_receive(source: &mut [u32], ct: u16) {
    disable_dmas(|| unsafe {
        DMA3::set_source(0x0DFFFF00 as *const _);
        DMA3::set_dest(source.as_ptr() as *mut _);
        DMA3::set_count(ct);
        let dma3_ctl = DMAControlSetting::new()
            .with_dest_address_control(DMADestAddressControl::Increment)
            .with_source_address_control(DMASrcAddressControl::Increment)
            .with_enabled(true);
        DMA3::set_control(dma3_ctl);
    });
}

/// A helper type for chunking data into segments.
struct ChunkArray {
    base_offset: usize,
    current: usize,
    end: usize,
}
impl ChunkArray {
    fn new(offset: usize, len: usize, limit: usize) -> Result<Self, Error> {
        if offset + len > limit {
            Err(Error::OutOfRange)
        } else {
            Ok(ChunkArray {
                base_offset: offset,
                current: offset,
                end: offset + len,
            })
        }
    }
}
#[derive(Debug)]
struct ChunkResult {
    word: usize,
    buf_offset: Range<usize>,
    src_offset: Range<usize>,
    start: usize,
}
impl Iterator for ChunkArray {
    type Item = ChunkResult;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let sector_end = (self.current + SECTOR_LEN) & !(SECTOR_LEN - 1);
            let end_offset = cmp::min(self.end, sector_end);
            let start = self.current & (SECTOR_LEN - 1);
            let res = ChunkResult {
                word: self.current >> SECTOR_SHIFT,
                buf_offset: self.current - self.base_offset..end_offset - self.base_offset,
                src_offset: start .. start + (end_offset - self.current),
                start,
            };
            self.current = end_offset;
            Some(res)
        }
    }
}

/// Union type to help build/receive commands.
#[repr(align(4))]
union BufferData {
    uninit: (),
    bits: [u16; 82],
    words: [u32; 41],
}
impl BufferData {
    fn new() -> Self {
        BufferData { uninit: () }
    }

    fn write_bit(&mut self, bit: usize, val: u8) {
        unsafe {
            self.bits[bit] = val as u16;
        }
    }
    fn write_num(&mut self, off: usize, count: usize, num: u32) {
        unsafe {
            for i in 0..count {
                self.bits[off + i] = (num >> (count - 1 - i)) as u16 & 1;
            }
        }
    }
    fn read_num(&mut self, off: usize, count: usize) -> u32 {
        let mut accum = 0;
        unsafe {
            for i in 0..count {
                accum <<= 1;
                accum |= self.bits[off + i] as u32;
            }
        }
        accum
    }

    fn receive(&mut self, count: usize) {
        unsafe {
            dma_receive(&mut self.words, count as u16);
        }
    }
    fn submit(&self, count: usize) {
        unsafe {
            dma_send(&self.words, count as u16);
        }
    }
}

/// The [`SramAccess`] used for battery backed SRAM.
pub struct EepromAccess(EepromSize);
impl EepromAccess {
    fn byte_count(&self) -> usize {
        match self.0 {
            EepromSize::Eeprom512B => 512,
            EepromSize::Eeprom8K => 8 * 1024,
        }
    }
    fn read_block(&self, word: usize) -> [u8; 8] {
        // Set address command.
        let mut buf = BufferData::new();
        buf.write_bit(0, 1);
        buf.write_bit(1, 1);
        match self.0 {
            EepromSize::Eeprom512B => {
                buf.write_num(2, 6, word as u32);
                buf.write_bit(8, 0);
                buf.submit(9);
            }
            EepromSize::Eeprom8K => {
                buf.write_num(2, 14, word as u32);
                buf.write_bit(16, 0);
                buf.submit(17);
            }
        }

        // Receive the buffer data
        buf.receive(68);
        let mut out = [0; 8];
        for i in 0..8 {
            out[i] = buf.read_num(4 + i * 8, 8) as u8;
        }
        out
    }
    fn write_raw(&self, word: usize, block: &[u8]) -> Result<(), Error> {
        unsafe {
            let mut buf = BufferData::new();
            buf.write_bit(0, 1);
            buf.write_bit(1, 0);
            match self.0 {
                EepromSize::Eeprom512B => {
                    buf.write_num(2, 6, word as u32);
                    for i in 0..8 {
                        buf.write_num(8 + i * 8, 8, block[i] as u32);
                    }
                    buf.write_bit(72, 0);
                    buf.submit(73);
                },
                EepromSize::Eeprom8K => {
                    buf.write_num(2, 14, word as u32);
                    for i in 0..8 {
                        buf.write_num(16 + i * 8, 8, block[i] as u32);
                    }
                    buf.write_bit(80, 0);
                    buf.submit(81);
                },
            }


            let timeout = Timeout::new()?;
            timeout.start();
            while PORT.read() & 1 != 1 {
                if timeout.is_timeout_met(10) {
                    return Err(Error::WriteError)
                }
            }
            Ok(())
        }
    }
    fn write_exact(&self, word: usize, data: &[u8], start: usize) -> Result<(), Error> {
        debug_assert!(start + data.len() <= 8, "Invalid starting offset.");
        let mut buf = self.read_block(word);
        buf[start..start+data.len()].copy_from_slice(data);
        self.write_raw(word, &buf)
    }
    fn write_block(&self, word: usize, data: &[u8], start: usize) -> Result<(), Error> {
        if data.len() == 8 && start == 0 {
            self.write_raw(word, data)
        } else {
            self.write_exact(word, data, start)
        }
    }
}
impl SramAccess for EepromAccess {
    fn sram_type(&self) -> Result<SramType, Error> {
        Ok(match self.0 {
            EepromSize::Eeprom512B => SramType::Eeprom512B,
            EepromSize::Eeprom8K => SramType::Eeprom8K,
        })
    }

    fn len(&self) -> Result<usize, Error> {
        Ok(self.byte_count())
    }

    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error> {
        let _guard = lock_sram()?;
        for chunk in ChunkArray::new(offset, buffer.len(), self.len()?)? {
            let block = self.read_block(chunk.word);
            buffer[chunk.buf_offset].copy_from_slice(&block[chunk.src_offset]);
        }
        Ok(())
    }

    fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error> {
        let _guard = lock_sram()?;
        for chunk in ChunkArray::new(offset, buffer.len(), self.len()?)? {
            let block = self.read_block(chunk.word);
            if &buffer[chunk.buf_offset] != &block[chunk.src_offset] {
                return Ok(false)
            }
        }
        Ok(true)
    }

    fn write_raw(&self, offset: usize, buffer: &[u8], _exact: bool) -> Result<(), Error> {
        let _guard = lock_sram()?;
        for chunk in ChunkArray::new(offset, buffer.len(), self.len()?)? {
            // We ignore the `exact` hint because the alignment and amount of
            // data read in excess is so small anyway.
            self.write_block(chunk.word, &buffer[chunk.buf_offset], chunk.start)?;
        }
        Ok(())
    }

    fn sector_shift(&self) -> Result<usize, Error> {
        Ok(SECTOR_SHIFT)
    }
}

/// A static instance of a [`SramAccess`] appropriate for 512B EEPROM.
pub static ACCESS_512B: &'static dyn SramAccess = &EepromAccess(EepromSize::Eeprom512B);

/// A static instance of a [`SramAccess`] appropriate for 8KiB EEPROM.
pub static ACCESS_8K: &'static dyn SramAccess = &EepromAccess(EepromSize::Eeprom8K);