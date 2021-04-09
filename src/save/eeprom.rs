//! A module containing support for EEPROM.
//!
//! EEPROM requires using DMA to issue commands for both reading and writing.

use super::{Error, MediaType, RawSaveAccess};
use crate::{
  prelude::*,
  save::{lock_media, MediaInfo, Timeout},
  sync::with_irqs_disabled,
};
use core::cmp;
use voladdress::*;

const PORT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x0DFFFF00) };
const SECTOR_SHIFT: usize = 3;
const SECTOR_LEN: usize = 1 << SECTOR_SHIFT;
const SECTOR_MASK: usize = SECTOR_LEN - 1;

/// Disable IRQs and DMAs during each read block.
fn disable_dmas(func: impl FnOnce()) {
  with_irqs_disabled(|| unsafe {
    // Disable other DMAs. This avoids our read/write from being interrupted
    // by a higher priority DMA channel.
    let dma0_ctl = DMA0CNT_H.read();
    let dma1_ctl = DMA1CNT_H.read();
    let dma2_ctl = DMA2CNT_H.read();
    DMA0CNT_H.write(dma0_ctl.with_enabled(false));
    DMA1CNT_H.write(dma1_ctl.with_enabled(false));
    DMA2CNT_H.write(dma2_ctl.with_enabled(false));

    // Executes the body of the function with DMAs and IRQs disabled.
    func();

    // Continues higher priority DMAs if they were enabled before.
    DMA0CNT_H.write(dma0_ctl);
    DMA1CNT_H.write(dma1_ctl);
    DMA2CNT_H.write(dma2_ctl);
  });
}

/// Sends a DMA command to EEPROM.
fn dma_send(source: &[u32], ct: u16) {
  disable_dmas(|| unsafe {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    DMA3SAD.write(source.as_ptr() as usize);
    DMA3DAD.write(0x0DFFFF00);
    DMA3CNT_L.write(ct);
    let dma3_ctl = DmaControl::new()
      .with_dest_addr(DestAddrControl::Increment)
      .with_src_addr(SrcAddrControl::Increment)
      .with_enabled(true);
    DMA3CNT_H.write(dma3_ctl);
  });
}

/// Receives a DMA packet from EEPROM.
fn dma_receive(source: &mut [u32], ct: u16) {
  disable_dmas(|| unsafe {
    DMA3SAD.write(0x0DFFFF00);
    DMA3DAD.write(source.as_mut_ptr() as usize);
    DMA3CNT_L.write(ct);
    let dma3_ctl = DmaControl::new()
      .with_dest_addr(DestAddrControl::Increment)
      .with_src_addr(SrcAddrControl::Increment)
      .with_enabled(true);
    DMA3CNT_H.write(dma3_ctl);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
  });
}

/// Union type to help build/receive commands.
struct BufferData {
  idx: usize,
  data: BufferContents,
}
#[repr(align(4))]
union BufferContents {
  uninit: (),
  bits: [u16; 82],
  words: [u32; 41],
}
impl BufferData {
  fn new() -> Self {
    BufferData { idx: 0, data: BufferContents { uninit: () } }
  }

  /// Writes a bit to the output buffer.
  fn write_bit(&mut self, val: u8) {
    unsafe {
      self.data.bits[self.idx] = val as u16;
      self.idx += 1;
    }
  }

  /// Writes a number to the output buffer
  fn write_num(&mut self, count: usize, num: u32) {
    for i in 0..count {
      self.write_bit(((num >> (count - 1 - i)) & 1) as u8);
    }
  }

  /// Reads a number from the input buffer.
  fn read_num(&mut self, off: usize, count: usize) -> u32 {
    let mut accum = 0;
    unsafe {
      for i in 0..count {
        accum <<= 1;
        accum |= self.data.bits[off + i] as u32;
      }
    }
    accum
  }

  /// Receives a number of words into the input buffer.
  fn receive(&mut self, count: usize) {
    unsafe {
      dma_receive(&mut self.data.words, count as u16);
    }
  }

  /// Submits the current buffer via DMA.
  fn submit(&self) {
    unsafe {
      dma_send(&self.data.words, self.idx as u16);
    }
  }
}

/// The properties of a given EEPROM type.
struct EepromProperties {
  addr_bits: usize,
  byte_len: usize,
}
impl EepromProperties {
  /// Reads a block from the save media.
  fn read_sector(&self, word: usize) -> [u8; 8] {
    // Set address command. The command is two one bits, followed by the
    // address, followed by a zero bit.
    //
    // 512B Command: [1 1|n n n n n n|0]
    // 8KiB Command: [1 1|n n n n n n n n n n n n n n|0]
    let mut buf = BufferData::new();
    buf.write_bit(1);
    buf.write_bit(1);
    buf.write_num(self.addr_bits, word as u32);
    buf.write_bit(0);
    buf.submit();

    // Receive the buffer data. The EEPROM sends 3 irrelevant bits followed
    // by 64 data bits.
    buf.receive(68);
    let mut out = [0; 8];
    for i in 0..8 {
      out[i] = buf.read_num(4 + i * 8, 8) as u8;
    }
    out
  }

  /// Writes a sector directly.
  fn write_sector_raw(&self, word: usize, block: &[u8]) -> Result<(), Error> {
    // Write sector command. The command is a one bit, followed by a
    // zero bit, followed by the address, followed by 64 bits of data.
    //
    // 512B Command: [1 0|n n n n n n|v v v v ...]
    // 8KiB Command: [1 0|n n n n n n n n n n n n n n|v v v v ...]
    let mut buf = BufferData::new();
    buf.write_bit(1);
    buf.write_bit(0);
    buf.write_num(self.addr_bits, word as u32);
    for i in 0..8 {
      buf.write_num(8, block[i] as u32);
    }
    buf.write_bit(0);
    buf.submit();

    // Wait for the sector to be written for 10 milliseconds.
    let timeout = Timeout::new()?;
    timeout.start();
    while PORT.read() & 1 != 1 {
      if timeout.is_timeout_met(10) {
        return Err(Error::OperationTimedOut);
      }
    }
    Ok(())
  }

  /// Writes a sector to the EEPROM, keeping any current contents outside the
  /// buffer's range.
  fn write_sector_safe(&self, word: usize, data: &[u8], start: usize) -> Result<(), Error> {
    let mut buf = self.read_sector(word);
    buf[start..start + data.len()].copy_from_slice(data);
    self.write_sector_raw(word, &buf)
  }

  /// Writes a sector to the EEPROM.
  fn write_sector(&self, word: usize, data: &[u8], start: usize) -> Result<(), Error> {
    if data.len() == 8 && start == 0 {
      self.write_sector_raw(word, data)
    } else {
      self.write_sector_safe(word, data, start)
    }
  }

  /// Checks whether an offset is in range.
  fn check_offset(&self, offset: usize, len: usize) -> Result<(), Error> {
    if offset.checked_add(len).is_none() && (offset + len) > self.byte_len {
      Err(Error::OutOfBounds)
    } else {
      Ok(())
    }
  }

  /// Implements EEPROM reads.
  fn read(&self, mut offset: usize, mut buf: &mut [u8]) -> Result<(), Error> {
    self.check_offset(offset, buf.len())?;
    let _guard = lock_media()?;
    while buf.len() != 0 {
      let start = offset & SECTOR_MASK;
      let end_len = cmp::min(SECTOR_LEN - start, buf.len());
      let sector = self.read_sector(offset >> SECTOR_SHIFT);
      buf[..end_len].copy_from_slice(&sector[start..start + end_len]);
      buf = &mut buf[end_len..];
      offset += end_len;
    }
    Ok(())
  }

  /// Implements EEPROM verifies.
  fn verify(&self, mut offset: usize, mut buf: &[u8]) -> Result<bool, Error> {
    self.check_offset(offset, buf.len())?;
    let _guard = lock_media()?;
    while buf.len() != 0 {
      let start = offset & SECTOR_MASK;
      let end_len = cmp::min(SECTOR_LEN - start, buf.len());
      if &buf[..end_len] != &self.read_sector(offset >> SECTOR_SHIFT) {
        return Ok(false);
      }
      buf = &buf[end_len..];
      offset += end_len;
    }
    Ok(true)
  }

  /// Implements EEPROM writes.
  fn write(&self, mut offset: usize, mut buf: &[u8]) -> Result<(), Error> {
    self.check_offset(offset, buf.len())?;
    let _guard = lock_media()?;
    while buf.len() != 0 {
      let start = offset & SECTOR_MASK;
      let end_len = cmp::min(SECTOR_LEN - start, buf.len());
      self.write_sector(offset >> SECTOR_SHIFT, &buf[..end_len], start)?;
      buf = &buf[end_len..];
      offset += end_len;
    }
    Ok(())
  }
}
const PROPS_512B: EepromProperties = EepromProperties { addr_bits: 6, byte_len: 512 };
const PROPS_8K: EepromProperties = EepromProperties { addr_bits: 14, byte_len: 8 * 1024 };

/// The [`RawSaveAccess`] used for 512 byte EEPROM.
pub struct Eeprom512B;
impl RawSaveAccess for Eeprom512B {
  fn info(&self) -> Result<&'static MediaInfo, Error> {
    Ok(&MediaInfo {
      media_type: MediaType::Eeprom512B,
      sector_shift: 3,
      sector_count: 64,
      requires_prepare_write: false,
    })
  }
  fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error> {
    PROPS_512B.read(offset, buffer)
  }
  fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error> {
    PROPS_512B.verify(offset, buffer)
  }
  fn prepare_write(&self, _: usize, _: usize) -> Result<(), Error> {
    Ok(())
  }
  fn write(&self, offset: usize, buffer: &[u8]) -> Result<(), Error> {
    PROPS_512B.write(offset, buffer)
  }
}

/// The [`RawSaveAccess`] used for 8 KiB EEPROM.
pub struct Eeprom8K;
impl RawSaveAccess for Eeprom8K {
  fn info(&self) -> Result<&'static MediaInfo, Error> {
    Ok(&MediaInfo {
      media_type: MediaType::Eeprom8K,
      sector_shift: 3,
      sector_count: 1024,
      requires_prepare_write: false,
    })
  }
  fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<(), Error> {
    PROPS_8K.read(offset, buffer)
  }
  fn verify(&self, offset: usize, buffer: &[u8]) -> Result<bool, Error> {
    PROPS_8K.verify(offset, buffer)
  }
  fn prepare_write(&self, _: usize, _: usize) -> Result<(), Error> {
    Ok(())
  }
  fn write(&self, offset: usize, buffer: &[u8]) -> Result<(), Error> {
    PROPS_8K.write(offset, buffer)
  }
}
