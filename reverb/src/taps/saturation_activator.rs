use crate::shared::one_pole_filter::{Mode, OnePoleFilter};

const SATURATION_THRESHOLD: f32 = 0.75;

pub struct SaturationActivator {
  amplitude: f32,
  smooth_saturation_gain: OnePoleFilter,
}

impl SaturationActivator {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      amplitude: 0.,
      smooth_saturation_gain: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn set_amplitude(&mut self, input: (f32, f32)) {
    self.amplitude = input.0.abs().max(input.1.abs());
  }

  pub fn get_saturation_gain(&mut self) -> f32 {
    let saturation_gain = if self.amplitude > SATURATION_THRESHOLD {
      1.
    } else {
      0.
    };

    self
      .smooth_saturation_gain
      .process(saturation_gain, 3., Mode::Hertz)
  }
}
