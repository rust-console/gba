use crate::macros::{
  pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field, u16_int_field,
  u8_bool_field, u8_int_field,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SweepControl(u8);
impl SweepControl {
  pub_const_fn_new_zeroed!();
  u8_int_field!(0 - 2, sweep_num, with_sweep_num);
  u8_bool_field!(3, sweep_increasing, with_sweep_increasing);
  u8_int_field!(4 - 6, sweep_time, with_sweep_time);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TonePattern(u16);
impl TonePattern {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 5, length, with_length);
  u16_int_field!(6 - 7, duty, with_duty);
  u16_int_field!(8 - 10, step_time, with_step_time);
  u16_bool_field!(11, step_increasing, with_step_increasing);
  u16_int_field!(12 - 15, volume, with_volume);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ToneFrequency(u16);
impl ToneFrequency {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 10, frequency, with_frequency);
  u16_bool_field!(14, stop_when_expired, with_stop_when_expired);
  u16_bool_field!(15, enabled, with_enabled);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WaveBank(u8);
impl WaveBank {
  pub_const_fn_new_zeroed!();
  u8_bool_field!(5, two_banks, with_two_banks);
  u8_bool_field!(6, bank1, with_bank1);
  u8_bool_field!(7, enabled, with_enabled);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WaveLenVolume(u16);
impl WaveLenVolume {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 7, length, with_length);
  u16_int_field!(13 - 14, volume, with_volume);
  u16_bool_field!(15, force75, with_force75);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WaveFrequency(u16);
impl WaveFrequency {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 10, sample_rate, with_length);
  u16_bool_field!(14, stop_when_expired, with_stop_when_expired);
  u16_bool_field!(15, enabled, with_enabled);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NoiseLenEnvelope(u16);
impl NoiseLenEnvelope {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 5, length, with_length);
  u16_int_field!(8 - 10, step_time, with_step_time);
  u16_bool_field!(11, step_increasing, with_step_increasing);
  u16_int_field!(12 - 15, volume, with_volume);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NoiseFrequency(u16);
impl NoiseFrequency {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 2, r, with_r);
  u16_bool_field!(3, counter7, with_counter7);
  u16_int_field!(4 - 7, s, with_s);
  u16_bool_field!(14, stop_when_expired, with_stop_when_expired);
  u16_bool_field!(15, enabled, with_enabled);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LeftRightVolume(u16);
impl LeftRightVolume {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 2, right_volume, with_right_volume);
  u16_int_field!(4 - 6, left_volume, with_left_volume);

  u16_bool_field!(8, tone1_right, with_tone1_right);
  u16_bool_field!(9, tone2_right, with_tone2_right);
  u16_bool_field!(10, wave_right, with_wave_right);
  u16_bool_field!(11, noise_right, with_noise_right);

  u16_bool_field!(12, tone1_left, with_tone1_left);
  u16_bool_field!(13, tone2_left, with_tone2_left);
  u16_bool_field!(14, wave_left, with_wave_left);
  u16_bool_field!(15, noise_left, with_noise_left);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum PsgMix {
  #[default]
  _25 = 0,
  _50 = 1,
  _100 = 2,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SoundMix(u16);
impl SoundMix {
  pub_const_fn_new_zeroed!();
  u16_enum_field!(0 - 2: PsgMix, psg, with_psg);
  u16_bool_field!(2, sound_a_full, with_sound_a_full);
  u16_bool_field!(3, sound_b_full, with_sound_b_full);

  u16_bool_field!(8, sound_a_right, with_sound_a_right);
  u16_bool_field!(9, sound_a_left, with_sound_a_left);
  u16_bool_field!(10, sound_a_timer, with_sound_a_timer);
  u16_bool_field!(11, sound_a_reset, with_sound_a_reset);

  u16_bool_field!(12, sound_b_right, with_sound_b_right);
  u16_bool_field!(13, sound_b_left, with_sound_b_left);
  u16_bool_field!(14, sound_b_timer, with_sound_b_timer);
  u16_bool_field!(15, sound_b_reset, with_sound_b_reset);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SoundEnable(u8);
impl SoundEnable {
  pub_const_fn_new_zeroed!();
  u8_bool_field!(0, tone1_playing, with_tone1_playing);
  u8_bool_field!(1, tone2_playing, with_tone2_playing);
  u8_bool_field!(2, wave_playing, with_wave_playing);
  u8_bool_field!(3, noise_playing, with_noise_playing);

  u8_bool_field!(7, enabled, with_enabled);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SampleCycle {
  #[default]
  _9bit = 0 << 14,
  _8bit = 1 << 14,
  _7bit = 2 << 14,
  _6bit = 3 << 14,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SoundBias(u16);
impl SoundBias {
  pub_const_fn_new_zeroed!();
  u16_int_field!(1 - 9, bias_level, with_bias_level);
  u16_enum_field!(14 - 15: SampleCycle, sample_cycle, with_sample_cycle);
}
