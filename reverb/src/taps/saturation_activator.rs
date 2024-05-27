use crate::shared::param_filter::ParamFilter;

const SATURATION_THRESHOLD: f32 = 0.75;

pub struct SaturationActivator {
  amplitude: f32,
  smooth_saturation_gain: ParamFilter,
}

impl SaturationActivator {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      amplitude: 0.,
      smooth_saturation_gain: ParamFilter::new(sample_rate, 3.),
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

    self.smooth_saturation_gain.process(saturation_gain)
  }
}
