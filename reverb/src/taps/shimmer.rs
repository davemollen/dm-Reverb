use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
  phasor::Phasor,
};
use std::f32::consts::PI;

pub struct Shimmer {
  delay_line: Vec<DelayLine>,
  phasor: Phasor,
}

impl Shimmer {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: vec![DelayLine::new((sample_rate * 0.2) as usize, sample_rate); 2],
      phasor: Phasor::new(sample_rate),
    }
  }

  pub fn run(&mut self, dry: (f32, f32), wet: (f32, f32), mix: f32) -> (f32, f32) {
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
    self.delay_line[0].write(input.0);
    self.delay_line[1].write(input.1);
  }

  fn mix(&self, a: (f32, f32), b: (f32, f32), factor: f32) -> (f32, f32) {
    (
      a.0 * (1. - factor) + b.0 * factor,
      a.1 * (1. - factor) + b.1 * factor,
    )
  }

  fn apply_shimmer(&mut self) -> (f32, f32) {
    let main_phase = self.phasor.run(-5.);

    (0..2)
      .map(|index| {
        let phase = if index == 0 { main_phase } else { (main_phase + 0.5) % 1. };
        let time = phase * 200.;
        let window = (phase * PI).fast_sin();
        let window = window * window;
        (
          self.delay_line[0].read(time, Interpolation::Linear) * window,
          self.delay_line[1].read(time, Interpolation::Linear) * window
        )
      })
      .fold((0., 0.), | result, item| {
        (result.0 + item.0, result.1 + item.1)
      })
  }
}
