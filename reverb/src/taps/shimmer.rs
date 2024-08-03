use crate::shared::{
  float_ext::FloatExt,
  phasor::Phasor,
  stereo_delay_line::{Interpolation, StereoDelayLine},
};
use std::f32::consts::PI;

const FREQUENCY: f32 = -5.;
const WINDOW_SIZE: f32 = 200.;

pub struct Shimmer {
  delay_line: StereoDelayLine,
  phasor: Phasor,
}

impl Shimmer {
  pub fn new(sample_rate: f32) -> Self {
    let delay_length = (sample_rate * WINDOW_SIZE / 1000.) as usize;

    Self {
      delay_line: StereoDelayLine::new(delay_length, sample_rate),
      phasor: Phasor::new(sample_rate),
    }
  }

  pub fn process(&mut self, dry: (f32, f32), wet: (f32, f32), mix: f32) -> (f32, f32) {
    let out = if mix > 0. {
      let grains_out = self.apply_shimmer();
      Self::mix(dry, grains_out, mix)
    } else {
      dry
    };
    self
      .delay_line
      .write(((dry.0 + wet.0) * 0.5, (dry.1 + wet.1) * 0.5));
    out
  }

  fn mix(left: (f32, f32), right: (f32, f32), factor: f32) -> (f32, f32) {
    (
      left.0 + (right.0 - left.0) * factor,
      left.1 + (right.1 - left.1) * factor,
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

        let delay_line_out = self.delay_line.read(time, Interpolation::Linear);
        (delay_line_out.0 * window, delay_line_out.1 * window)
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
