mod delta;
use crate::shared::{
  constants::MAX_DEPTH,
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};
use delta::Delta;
use std::f32::consts::PI;

const FADE_THRESHOLD_FACTOR: f32 = 0.05;
const FADE_THRESHOLD: f32 = MAX_DEPTH * FADE_THRESHOLD_FACTOR;

#[derive(Clone, Copy)]
pub struct Grains {
  start_position: [f32; 2],
  delta: [Delta; 2],
  phase_offset: [f32; 2],
}

impl Grains {
  pub fn new() -> Self {
    Self {
      start_position: [0.; 2],
      delta: [Delta::new(); 2],
      phase_offset: [0., 0.5],
    }
  }

  pub fn process(
    &mut self,
    delay_line: &DelayLine,
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_depth: f32,
  ) -> f32 {
    let grains_out = self.apply_grains(delay_line, size, time_fraction, lfo_phase, lfo_depth);
    if lfo_depth < FADE_THRESHOLD {
      self.mix(
        delay_line.read(size * time_fraction, Interpolation::Linear),
        grains_out,
        lfo_depth,
        FADE_THRESHOLD_FACTOR,
      )
    } else {
      grains_out
    }
  }

  fn mix(&self, a: f32, b: f32, lfo_depth: f32, threshold: f32) -> f32 {
    let factor = lfo_depth / MAX_DEPTH * threshold.recip();
    a.mix(b, factor)
  }

  fn apply_grains(
    &mut self,
    delay_line: &DelayLine,
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_depth: f32,
  ) -> f32 {
    (0..2)
      .map(|i| {
        let phase = Self::wrap(lfo_phase + self.phase_offset[i]);
        let trigger = self.delta[i].process(phase) < 0.;
        if trigger {
          self.start_position[i] = fastrand::f32() * lfo_depth;
        };
        let window = (phase * PI).fast_sin();
        let time = size * time_fraction + self.start_position[i];

        delay_line.read(time, Interpolation::Linear) * window * window
      })
      .sum()
  }

  fn wrap(x: f32) -> f32 {
    if x >= 1. {
      x - 1.
    } else {
      x
    }
  }
}
