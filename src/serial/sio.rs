//! Contains types and definitions for Serial IO registers.

use crate::macros::{const_new, u16_bool_field, u16_enum_field};
use voladdress::{Safe, VolAddress};

/// Serial IO Control. Read/Write.
pub const SIOCNT: VolAddress<SioControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0128) };

/// Serial IO Data. Read/Write.
pub const SIODATA8: VolAddress<u16, Safe, Safe> =
  unsafe { VolAddress::new(0x400_012A) };

/// General IO Control. Read/Write.
pub const RCNT: VolAddress<IoControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0134) };

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum BaudRate {
  Bps9600 = 0,
  Bps38400 = 1,
  Bps57600 = 2,
  Bps115200 = 3,
}
impl Default for BaudRate {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::Bps9600
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SioMode {
  Normal8Bit = (0 << 12),
  MultiPlayer = (1 << 12),
  Normal32Bit = (2 << 12),
  Uart = (3 << 12),
}
impl Default for SioMode {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::Normal8Bit
  }
}

/// Setting for the serial IO control register.
///
/// * 0-1: `BaudRate`
/// * 2: Use hardware flow control
/// * 3: Use odd parity instead of even
/// * 4: TX buffer is full
/// * 5: RX buffer is empty
/// * 6: Error occurred
/// * 7: Use 8-bit data length instead of 7-bit
/// * 8: Use hardware FIFO
/// * 9: Enable parity check
/// * 10: Enable data receive
/// * 11: Enable data transmit
/// * 12-13: `SioMode`
/// * 14: Trigger interrupt on RX
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SioControlSetting(u16);
impl SioControlSetting {
  const_new!();
  u16_enum_field!(0 - 1: BaudRate, baud_rate, with_baud_rate);
  u16_bool_field!(2, flow_control, with_flow_control);
  u16_bool_field!(3, parity_odd, with_parity_odd);
  u16_bool_field!(4, tx_full, with_tx_full);
  u16_bool_field!(5, rx_empty, with_rx_empty);
  u16_bool_field!(6, error, with_error);
  u16_bool_field!(7, data_length_8bit, with_data_length_8bit);
  u16_bool_field!(8, fifo_enable, with_fifo_enable);
  u16_bool_field!(9, parity_enable, with_parity_enable);
  u16_bool_field!(10, tx_enable, with_tx_enable);
  u16_bool_field!(11, rx_enable, with_rx_enable);
  u16_enum_field!(12 - 13: SioMode, mode, with_mode);
  u16_bool_field!(14, irq_enable, with_irq_enable);
}

/// Setting for the general IO control register.
///
/// * 0: SC state
/// * 1: SD state
/// * 2: SI state
/// * 3: SO state
/// * 4: Set SC as output, instead of input
/// * 5: Set SD as output, instead of input
/// * 6: Set SI as output, instead of input
/// * 7: Set SO as output, instead of input
/// * 8: Trigger interrupt on SI change
/// * 14-15: `IoMode`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IoControlSetting(u16);
impl IoControlSetting {
  const_new!();
  u16_bool_field!(0, sc, with_sc);
  u16_bool_field!(1, sd, with_sd);
  u16_bool_field!(2, si, with_si);
  u16_bool_field!(3, so, with_so);
  u16_bool_field!(4, sc_output_enable, with_sc_output_enable);
  u16_bool_field!(5, sd_output_enable, with_sd_output_enable);
  u16_bool_field!(6, si_output_enable, with_si_output_enable);
  u16_bool_field!(7, so_output_enable, with_so_output_enable);
  u16_bool_field!(8, si_irq_enable, with_si_irq_enable);
  u16_enum_field!(14 - 15: IoMode, mode, with_mode);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum IoMode {
  Disabled = (0 << 14),
  GPIO = (2 << 14),
  JoyBus = (3 << 14),
}
impl Default for IoMode {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::Disabled
  }
}

/// Empty struct that implements embedded_hal traits.
#[cfg(feature = "serial")]
#[derive(Clone)]
pub struct SioSerial;

#[cfg(feature = "serial")]
impl SioSerial {
  /// Initialize SioSerial with provided baud rate and default 8N1 settings.
  pub fn init(baud: BaudRate) -> Self {
    RCNT.write(IoControlSetting::new());
    SIOCNT.write(
      // default settings: 8N1
      SioControlSetting::new()
        .with_baud_rate(baud)
        .with_data_length_8bit(true)
        .with_mode(SioMode::Uart)
        .with_fifo_enable(true)
        .with_rx_enable(true)
        .with_tx_enable(true),
    );

    SioSerial
  }
}

/// Serial IO error type.
#[cfg(feature = "serial")]
#[derive(Debug)]
pub enum SioError {
  /// * Error bit in SIOCNT is set
  ErrorBitSet,
}

#[cfg(feature = "serial")]
impl embedded_hal::serial::Read<u8> for SioSerial {
  type Error = SioError;

  fn read(&mut self) -> nb::Result<u8, Self::Error> {
    match SIOCNT.read() {
      siocnt if siocnt.error() => Err(nb::Error::Other(SioError::ErrorBitSet)),
      siocnt if siocnt.rx_empty() => Err(nb::Error::WouldBlock),
      _ => Ok(SIODATA8.read() as u8),
    }
  }
}

#[cfg(feature = "serial")]
impl embedded_hal::serial::Write<u8> for SioSerial {
  type Error = SioError;

  fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
    self.flush()?;
    SIODATA8.write(word as u16);
    Ok(())
  }

  fn flush(&mut self) -> nb::Result<(), Self::Error> {
    match SIOCNT.read() {
      siocnt if siocnt.error() => Err(nb::Error::Other(SioError::ErrorBitSet)),
      siocnt if siocnt.tx_full() => Err(nb::Error::WouldBlock),
      _ => Ok(()),
    }
  }
}
