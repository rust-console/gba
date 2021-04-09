//! Contains types and definitions for Serial IO registers.

use super::*;

/// Serial IO Control. Read/Write.
pub const SIOCNT: VolAddress<SioControlSetting, Safe, Safe> =
  unsafe { VolAddress::new(0x400_0128) };

/// Serial IO Data. Read/Write.
pub const SIODATA8: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x400_012A) };

/// General IO Control. Read/Write.
pub const RCNT: VolAddress<IoControlSetting, Safe, Safe> = unsafe { VolAddress::new(0x400_0134) };

newtype!(
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
  SioControlSetting,
  u16
);

#[allow(missing_docs)]
impl SioControlSetting {
  phantom_fields! {
      self.0: u16,
      baud_rate: 0-1=BaudRate<Bps9600,Bps38400,Bps57600,Bps115200>,
      flow_control: 2,
      parity_odd: 3,
      tx_full: 4,
      rx_empty: 5,
      error: 6,
      data_length_8bit: 7,
      fifo_enable:8,
      parity_enable: 9,
      tx_enable: 10,
      rx_enable: 11,
      mode: 12-13=SioMode<Normal8Bit,MultiPlayer,Normal32Bit,Uart>,
      irq_enable: 14,
  }
}

newtype_enum! {
    /// Supported baud rates.
    BaudRate = u16,
    /// * 9600 bps
    Bps9600 = 0,
    /// * 38400 bps
    Bps38400 = 1,
    /// * 57600 bps
    Bps57600 = 2,
    /// * 115200 bps
    Bps115200 = 3,
}

newtype_enum! {
    /// Serial IO modes.
    SioMode = u16,
    /// * Normal mode: 8-bit data
    Normal8Bit = 0,
    /// * Multiplayer mode: 16-bit data
    MultiPlayer = 1,
    /// * Normal mode: 32-bit data
    Normal32Bit = 2,
    /// * UART (RS232) mode: 7 or 8-bit data
    Uart = 3,
}

newtype!(
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
  IoControlSetting,
  u16
);

newtype_enum! {
    /// General IO modes.
    IoMode = u16,
    /// * IO disabled
    Disabled = 0,
    /// * General Purpose IO
    GPIO = 2,
    /// * JoyBus mode
    JoyBus = 3,
}

#[allow(missing_docs)]
impl IoControlSetting {
  phantom_fields! {
      self.0: u16,
      sc: 0,
      sd: 1,
      si: 2,
      so: 3,
      sc_output_enable: 4,
      sd_output_enable: 5,
      si_output_enable: 6,
      so_output_enable: 7,
      si_irq_enable: 8,
      mode: 14-15=IoMode<Disabled,GPIO,JoyBus>,
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
