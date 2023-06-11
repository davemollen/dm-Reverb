use crate::{
  delay_line::{DelayLine, Interpolation},
  delta::Delta,
  float_ext::FloatExt,
  MAX_DEPTH,
};
use rand::random;
use std::f32::consts::PI;

const FADE_THRESHOLD_FACTOR: f32 = 0.05;
const FADE_THRESHOLD: f32 = MAX_DEPTH * FADE_THRESHOLD_FACTOR;

#[derive(Clone, Copy)]
struct Grain {
  start_position: f32,
  delta: Delta,
}

pub struct Grains {
  grains: [Grain; 2],
}

impl Grains {
  pub fn new() -> Self {
    Self {
      grains: [Grain {
        start_position: 0.,
        delta: Delta::new(),
      }; 2],
    }
  }

  pub fn run(
    &mut self,
    delay_line: &mut DelayLine,
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
    delay_line: &mut DelayLine,
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_depth: f32,
  ) -> f32 {
    self
      .grains
      .iter_mut()
      .enumerate()
      .map(|(index, grain)| {
        let phase = (lfo_phase + index as f32 * 0.5) % 1.;
        let trigger = grain.delta.run(phase) < 0.;
        if trigger {
          grain.start_position = random::<f32>() * lfo_depth;
        };
        let window = (phase * PI).fast_sin();
        let time = size * time_fraction + grain.start_position;

        delay_line.read(time, Interpolation::Linear) * window * window
      })
      .sum()
  }
}
