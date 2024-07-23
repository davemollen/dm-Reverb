mod average;
use {
  crate::shared::param_filter::ParamFilter,
  average::Average,
  std::simd::{f32x4, num::SimdFloat},
};

const THRESHOLD: f32 = 0.075;

pub struct Saturation {
  average: Average,
  enabled: ParamFilter,
}

impl Saturation {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      average: Average::new((1000. / 44100. * sample_rate) as usize),
      enabled: ParamFilter::new(sample_rate, 5.),
    }
  }

  pub fn process(&mut self, sidechain_input: (f32, f32), taps: f32x4) -> (f32x4, f32) {
    let average = self.average.process(sidechain_input.0 + sidechain_input.1);
    let saturation_gain = self
      .enabled
      .process(if average > THRESHOLD { 1. } else { 0. });

    let gain_compensation = (1. + THRESHOLD - average).min(1.);
    let sat_gain = f32x4::splat(saturation_gain);
    let clean_gain = f32x4::splat(1. - saturation_gain);
    let clean_out = taps * clean_gain;

    let saturation_out = if saturation_gain > 0. {
      Self::fast_atan2(taps) * sat_gain + clean_out
    } else {
      clean_out
    };

    (saturation_out, gain_compensation)
  }

  fn fast_atan2(x: f32x4) -> f32x4 {
    let limit = f32x4::splat(1.);
    let n1 = f32x4::splat(0.97239411);
    let n2 = f32x4::splat(-0.19194795);
    ((n1 + n2 * x * x) * x).simd_clamp(-limit, limit)
  }
}
