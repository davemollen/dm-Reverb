use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
  phasor::Phasor,
};
use std::f32::consts::PI;

const FREQUENCY: f32 = -5.;
const WINDOW_SIZE: f32 = 200.;

pub struct Shimmer {
  delay_lines: Vec<DelayLine>,
  phasor: Phasor,
}

impl Shimmer {
  pub fn new(sample_rate: f32) -> Self {
    let delay_length = (sample_rate * WINDOW_SIZE / 1000.) as usize;

    Self {
      delay_lines: vec![DelayLine::new(delay_length, sample_rate); 2],
      phasor: Phasor::new(sample_rate),
    }
  }

  pub fn process(&mut self, dry: (f32, f32), wet: (f32, f32), mix: f32) -> (f32, f32) {
    let out = if mix > 0. {
      let grains_out = self.apply_shimmer();
      self.mix(dry, grains_out, mix)
    } else {
      dry
    };
    self.write(wet);
    out
  }

  fn write(&mut self, input: (f32, f32)) {
    self.delay_lines[0].write(input.0);
    self.delay_lines[1].write(input.1);
  }

  fn mix(&self, a: (f32, f32), b: (f32, f32), factor: f32) -> (f32, f32) {
    let inverted_factor = 1. - factor;

    (
      a.0 * inverted_factor + b.0 * factor,
      a.1 * inverted_factor + b.1 * factor,
    )
  }

  fn apply_shimmer(&mut self) -> (f32, f32) {
    let main_phase = self.phasor.process(FREQUENCY);

    (0..2)
      .map(|index| {
        let phase = if index == 0 {
          main_phase
        } else {
          Self::wrap(main_phase + 0.5)
        };
        let time = phase * WINDOW_SIZE;
        let window = (phase * PI).fast_sin();
        let window = window * window;
        (
          self.delay_lines[0].read(time, Interpolation::Linear) * window,
          self.delay_lines[1].read(time, Interpolation::Linear) * window,
        )
      })
      .fold((0., 0.), |result, item| {
        (result.0 + item.0, result.1 + item.1)
      })
  }

  fn wrap(x: f32) -> f32 {
    if x >= 1. {
      x - 1.
    } else {
      x
    }
  }
}
