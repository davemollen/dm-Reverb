use crate::{
  allpass_filter::AllpassFilter,
  dc_block::DcBlock,
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
  grains::Grains,
  one_pole_filter::{Mode, OnePoleFilter},
  MAX_DEPTH, MAX_SIZE,
};
use std::f32::consts::TAU;

pub struct Tap {
  time_fraction: f32,
  delay_line: DelayLine,
  all_pass_filter: AllpassFilter,
  one_pole_filter: OnePoleFilter,
  diffuser_time: f32,
  lfo_phase_offset: f32,
  grains: Grains,
  dc_block: DcBlock,
}

impl Tap {
  pub fn new(
    sample_rate: f32,
    time_fraction: f32,
    diffuser_time: f32,
    lfo_phase_offset: f32,
  ) -> Self {
    Self {
      time_fraction,
      delay_line: DelayLine::new(
        (sample_rate * (MAX_SIZE * 0.001 * time_fraction + MAX_DEPTH)) as usize,
        sample_rate,
      ),
      all_pass_filter: AllpassFilter::new(sample_rate),
      diffuser_time,
      one_pole_filter: OnePoleFilter::new(sample_rate),
      lfo_phase_offset,
      grains: Grains::new(),
      dc_block: DcBlock::new(sample_rate),
    }
  }

  fn vibrato_read(&mut self, size: f32, lfo_phase: f32, lfo_depth: f32) -> f32 {
    let lfo_phase_input = (lfo_phase + self.lfo_phase_offset) % 1. * TAU;
    let lfo = lfo_phase_input.fast_sin() * lfo_depth.abs();

    self
      .delay_line
      .read(self.time_fraction * size + lfo, Interpolation::Linear)
  }

  fn grain_read(&mut self, size: f32, lfo_phase: f32, lfo_depth: f32) -> f32 {
    self.grains.run(
      &mut self.delay_line,
      size,
      self.time_fraction,
      lfo_phase,
      lfo_depth,
    )
  }

  pub fn delay_network_read(&mut self, size: f32, lfo_phase: f32, lfo_depth: f32) -> f32 {
    if lfo_depth == 0. {
      self
        .delay_line
        .read(size * self.time_fraction, Interpolation::Linear)
    } else if lfo_depth < 0. {
      self.vibrato_read(size, lfo_phase, lfo_depth)
    } else {
      self.grain_read(size, lfo_phase, lfo_depth)
    }
  }

  pub fn early_reflection_read(&mut self, size: f32, time_fraction: f32) -> f32 {
    self
      .delay_line
      .read(size * time_fraction, Interpolation::Linear)
  }

  pub fn write(&mut self, input: f32) {
    self.delay_line.write(input);
  }

  pub fn apply_absorb(&mut self, input: f32, absorb: f32) -> f32 {
    self.one_pole_filter.run(input, absorb, Mode::Linear)
  }

  pub fn apply_diffuse(&mut self, input: f32, diffuse: f32) -> f32 {
    self.all_pass_filter.run(input, self.diffuser_time, diffuse)
  }

  pub fn apply_saturation(&mut self, input: f32, decay: f32, saturation_gain: f32) -> f32 {
    let clean_gain = 1. - saturation_gain;
    let saturation_out = input * clean_gain + input.fast_atan2() * saturation_gain;
    self.dc_block.run(saturation_out * decay * 0.5)
  }
}
