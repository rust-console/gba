//! Module for sound registers.

use super::*;

//TODO within these "read/write" registers only some bits are actually read/write!

/// Sound Channel 1 Sweep Register (`NR10`). Read/Write.
pub const SOUND1CNT_L: VolAddress<SweepRegisterSetting> = unsafe { VolAddress::new(0x400_0060) };

newtype! {
  /// TODO: docs
  SweepRegisterSetting, u16
}

impl SweepRegisterSetting {
  phantom_fields! {
    self.0: u16,
    sweep_shift: 0-2,
    sweep_decreasing: 3,
    sweep_time: 4-6,
  }
}

/// Sound Channel 1 Duty/Length/Envelope (`NR11`, `NR12`). Read/Write.
pub const SOUND1CNT_H: VolAddress<DutyLenEnvelopeSetting> = unsafe { VolAddress::new(0x400_0062) };

newtype! {
  /// TODO: docs
  DutyLenEnvelopeSetting, u16
}

impl DutyLenEnvelopeSetting {
  phantom_fields! {
    self.0: u16,
    sound_length: 0-5,
    wave_pattern_duty: 6-7, //TODO: enum this
    envelope_step_time: 8-10,
    envelope_increasing: 11,
    initial_envelope_volume: 12-15,
  }
}

/// Sound Channel 1 Frequency/Control (`NR13`, `NR14`). Read/Write.
pub const SOUND1CNT_X: VolAddress<FrequencyControlSetting> = unsafe { VolAddress::new(0x400_0064) };

newtype! {
  /// TODO: docs
  FrequencyControlSetting, u32 // TODO: u16 or u32?
}

impl FrequencyControlSetting {
  phantom_fields! {
    self.0: u32,
    frequency: 0-10,
    length_flag: 14,
    is_initial: 15,
  }
}

/// Sound Channel 2 Channel 2 Duty/Length/Envelope (`NR21`, `NR22`). Read/Write.
pub const SOUND2CNT_L: VolAddress<DutyLenEnvelopeSetting> = unsafe { VolAddress::new(0x400_0068) };

/// Sound Channel 2 Frequency/Control (`NR23`, `NR24`). Read/Write.
pub const SOUND2CNT_H: VolAddress<FrequencyControlSetting> = unsafe { VolAddress::new(0x400_006C) };

/// Sound Channel 3 Stop/Wave RAM select (`NR23`, `NR24`). Read/Write.
pub const SOUND3CNT_L: VolAddress<StopWaveRAMSelectSetting> =
  unsafe { VolAddress::new(0x400_0070) };

newtype! {
  /// TODO: docs
  StopWaveRAMSelectSetting, u16
}

impl StopWaveRAMSelectSetting {
  phantom_fields! {
    self.0: u16,
    wave_ram_dimension_2d: 5,
    wave_ram_bank_number: 6,
    sound_channel_3_playing: 7,
  }
}

/// Sound Channel 3 Length/Volume (`NR23`, `NR24`). Read/Write.
pub const SOUND3CNT_H: VolAddress<LengthVolumeSetting> = unsafe { VolAddress::new(0x400_0072) };

newtype! {
  /// TODO: docs
  LengthVolumeSetting, u16
}

impl LengthVolumeSetting {
  phantom_fields! {
    self.0: u16,
    sound_length: 0-7,
    sound_volume: 13-14,
    force_75percent: 15,
  }
}

/// Sound Channel 3 Frequency/Control (`NR33`, `NR34`). Read/Write.
pub const SOUND3CNT_X: VolAddress<FrequencyControlSetting> = unsafe { VolAddress::new(0x400_0074) };

/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM0_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_0090) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM0_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_0092) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM1_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_0094) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM1_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_0096) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM2_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_0098) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM2_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_009A) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM3_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_009C) };
/// Channel 3 Wave Pattern RAM (W/R)
pub const WAVE_RAM3_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_009E) };

/// Sound Channel 4 Length/Envelope (`NR41`, `NR42`). Read/Write.
pub const SOUND4CNT_L: VolAddress<LengthEnvelopeSetting> = unsafe { VolAddress::new(0x400_0078) };

newtype! {
  /// TODO: docs
  LengthEnvelopeSetting, u32 // TODO: is this u32?
}

impl LengthEnvelopeSetting {
  phantom_fields! {
    self.0: u32,
    sound_length: 0-5,
    envelope_step_time: 8-10,
    envelope_increasing: 11,
    initial_envelope_volume: 12-15,
  }
}

/// Sound Channel 4 Frequency/Control (`NR43`, `NR44`). Read/Write.
pub const SOUND4CNT_H: VolAddress<NoiseFrequencySetting> = unsafe { VolAddress::new(0x400_007C) };

newtype! {
  /// TODO: docs
  NoiseFrequencySetting, u32 // TODO: is this u32?
}

impl NoiseFrequencySetting {
  phantom_fields! {
    self.0: u32,
    frequency_divide_ratio: 0-2,
    counter_step_width_7bit: 3,
    shift_clock_frequency: 4-7,
    length_flag_stop: 14,
    initial_restart: 15,
  }
}

// TODO: unify FIFO as

/// Sound A FIFO, Data 0 and Data 1 (W)
pub const FIFO_A_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_00A0) };
/// Sound A FIFO, Data 2 and Data 3 (W)
pub const FIFO_A_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_00A2) };
/// Sound B FIFO, Data 0 and Data 1 (W)
pub const FIFO_B_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_00A4) };
/// Sound B FIFO, Data 2 and Data 3 (W)
pub const FIFO_B_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_00A6) };

/// Channel L/R Volume/Enable (`NR50`, `NR51`). Read/Write.
pub const SOUNDCNT_L: VolAddress<NonWaveVolumeEnableSetting> =
  unsafe { VolAddress::new(0x400_0080) };

newtype! {
  /// TODO: docs
  NonWaveVolumeEnableSetting, u16
}

impl NonWaveVolumeEnableSetting {
  phantom_fields! {
    self.0: u16,
    right_master_volume: 0-2,
    left_master_volume: 4-6,
    right_enable_flags: 8-11, // TODO: this is junk
    left_enable_flags: 12-15, // TODO: junk
  }
}

/// DMA Sound Control/Mixing. Read/Write.
pub const SOUNDCNT_H: VolAddress<WaveVolumeEnableSetting> = unsafe { VolAddress::new(0x400_0082) };

newtype! {
  /// TODO: docs
  WaveVolumeEnableSetting, u16
}

impl WaveVolumeEnableSetting {
  phantom_fields! {
    self.0: u16,
    sound_number_volume: 0-1=NumberSoundVolume<Quarter, Half, Full>,
    dma_sound_a_full_volume: 2,
    dma_sound_b_full_volume: 3,
    dma_sound_a_enable_right: 8,
    dma_sound_a_enable_left: 9,
    dma_sound_a_timer_select: 10,
    dma_sound_a_reset_fifo: 11,
    dma_sound_b_enable_right: 12,
    dma_sound_b_enable_left: 13,
    dma_sound_b_timer_select: 14,
    dma_sound_b_reset_fifo: 15,
  }
}

newtype_enum! {
  /// TODO: docs
  NumberSoundVolume = u16,
  /// TODO: docs
  Quarter = 0,
  /// TODO: docs
  Half = 1,
  /// TODO: docs
  Full = 2,
}

/// Sound on/off (`NR52`). Read/Write.
pub const SOUNDCNT_X: VolAddress<SoundMasterSetting> = unsafe { VolAddress::new(0x400_0084) };

newtype! {
  /// TODO: docs
  SoundMasterSetting, u16
}

impl SoundMasterSetting {
  phantom_fields! {
    self.0: u16,
    sound1_on: 0,
    sound2_on: 1,
    sound3_on: 2,
    sound4_on: 3,
    psg_fifo_master_enabled: 7,
  }
}

/// Sound on/off (`NR52`). Read/Write.
pub const SOUNDBIAS: VolAddress<SoundPWMSetting> = unsafe { VolAddress::new(0x400_0088) };

newtype! {
  /// TODO: docs
  SoundPWMSetting, u16
}

impl SoundMasterSetting {
  phantom_fields! {
    self.0: u16,
    bias_level: 1-9,
    amplitude_resolution: 14-15, // TODO: enum this
  }
}
