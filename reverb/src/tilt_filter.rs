mod biquad_filter;
use biquad_filter::BiquadFilter;
use std::f32::consts::TAU;

pub struct TiltFilter {
  double_sr: f32,
  biquad_filter_left: BiquadFilter,
  biquad_filter_right: BiquadFilter,
}

impl TiltFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      double_sr: sample_rate * 2.,
      biquad_filter_left: BiquadFilter::new(),
      biquad_filter_right: BiquadFilter::new(),
    }
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    low_frequency: f32,
    high_frequency: f32,
    low_gain: f32,
    high_gain: f32,
    tilt: f32,
  ) -> (f32, f32) {
    if tilt == 0. {
      input
    } else {
      let s_domain_coefficients =
        self.get_s_domain_coefficients(low_frequency, high_frequency, low_gain, high_gain, tilt);
      let z_domain_coefficients = self.apply_bilinear_transform(s_domain_coefficients);
      self.get_biquad_filters_output(input, z_domain_coefficients)
    }
  }

  fn get_s_domain_coefficients(
    &self,
    low_frequency: f32,
    high_frequency: f32,
    low_gain: f32,
    high_gain: f32,
    tilt: f32,
  ) -> (f32, f32, f32, f32, f32, f32) {
    let squared_double_sr = self.double_sr * self.double_sr;
    let normalized_tilt = tilt * 0.5 + 0.5;

    let low_radians = low_frequency * TAU;
    let low_range = low_radians * low_gain - low_radians;
    let low_a = low_range * normalized_tilt + low_radians;
    let low_b = low_range * (1. - normalized_tilt) + low_radians;

    let high_radians = high_frequency * TAU;
    let high_offset = 1. / high_gain * high_radians;
    let high_range = high_radians - high_offset;
    let high_a = high_gain.powf(tilt);
    let high_b = high_range * normalized_tilt + high_offset;

    let b0 = high_a * squared_double_sr;
    let b1 = low_b * high_a + high_b * self.double_sr;
    let b2 = low_b * high_b;
    let a0 = squared_double_sr;
    let a1 = (low_a + high_b) * self.double_sr;
    let a2 = low_a * high_b;

    (a0, a1, a2, b0, b1, b2)
  }

  fn apply_bilinear_transform(
    &self,
    params: (f32, f32, f32, f32, f32, f32),
  ) -> (f32, f32, f32, f32, f32) {
    let (a0, a1, a2, b0, b1, b2) = params;
    let bzt_b0 = a0 + a1 + a2;
    let bzt_a0 = (b0 + b1 + b2) / bzt_b0;
    let bzt_a1 = (2. * b2 - 2. * b0) / bzt_b0;
    let bzt_a2 = (b2 - b1 + b0) / bzt_b0;
    let bzt_b1 = (2. * a2 - 2. * a0) / bzt_b0;
    let bzt_b2 = (a2 - a1 + a0) / bzt_b0;

    (bzt_a0, bzt_a1, bzt_a2, bzt_b1, bzt_b2)
  }

  fn get_biquad_filters_output(
    &mut self,
    input: (f32, f32),
    biquad_params: (f32, f32, f32, f32, f32),
  ) -> (f32, f32) {
    let (a0, a1, a2, b1, b2) = biquad_params;
    (
      self.biquad_filter_left.process(input.0, a0, a1, a2, b1, b2),
      self
        .biquad_filter_right
        .process(input.1, a0, a1, a2, b1, b2),
    )
  }
}
