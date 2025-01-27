use std::f32::consts::TAU;

use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};

use super::grains::Grains;

pub trait DelayRead {
  fn delay_network_read(
    &self,
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_phase_offset: f32,
    lfo_depth: f32,
    grains: &mut Grains,
  ) -> f32;

  fn vibrato_read(
    &self,
    size: f32,
    lfo_phase: f32,
    lfo_phase_offset: f32,
    lfo_depth: f32,
    time_fraction: f32,
  ) -> f32;

  fn grain_read(
    &self,
    grains: &mut Grains,
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_depth: f32,
  ) -> f32;
}

impl DelayRead for DelayLine {
  fn delay_network_read(
    &self,
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_phase_offset: f32,
    lfo_depth: f32,
    grains: &mut Grains,
  ) -> f32 {
    if lfo_depth == 0. {
      self.read(size * time_fraction, Interpolation::Linear)
    } else if lfo_depth < 0. {
      self.vibrato_read(size, lfo_phase, lfo_phase_offset, lfo_depth, time_fraction)
    } else {
      self.grain_read(grains, size, time_fraction, lfo_phase, lfo_depth)
    }
  }

  fn vibrato_read(
    &self,
    size: f32,
    lfo_phase: f32,
    lfo_phase_offset: f32,
    lfo_depth: f32,
    time_fraction: f32,
  ) -> f32 {
    let lfo_phase_input = lfo_phase + lfo_phase_offset;
    let phase = if lfo_phase_input > 1. {
      lfo_phase_input - 1.
    } else {
      lfo_phase_input
    } * TAU;
    let lfo = phase.fast_sin() * lfo_depth.abs();

    self.read(time_fraction * size + lfo, Interpolation::Linear)
  }

  fn grain_read(
    &self,
    grains: &mut Grains,
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_depth: f32,
  ) -> f32 {
    grains.process(self, size, time_fraction, lfo_phase, lfo_depth)
  }
}
