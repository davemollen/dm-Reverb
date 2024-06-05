mod biquad_filter;
use biquad_filter::BiquadFilter;
mod bilinear_transform;
use bilinear_transform::BilinearTransform;

const C1: f32 = 5.6e-9;
const C2: f32 = 5.6e-9;
const R1: f32 = 2250.;
const R2: f32 = 2250.;
const RF1: f32 = 47000.;
const RF2: f32 = 47000.;
const R_TILT: f32 = 140000.;

pub struct TiltFilter {
  bilinear_transform: BilinearTransform,
  biquad_filter: BiquadFilter,
}

impl TiltFilter {
  const C1C2: f32 = C1 * C2;
  const C1C2R1: f32 = Self::C1C2 * R1;
  const C1C2R1R2: f32 = Self::C1C2R1 * R2;
  const C1C2R2: f32 = Self::C1C2 * R2;
  const C1C2R2RF2: f32 = Self::C1C2R2 * RF2;
  const C2R2: f32 = C2 * R2;
  const C1R1: f32 = C1 * R1;
  const C1RF2: f32 = C1 * RF2;
  const C1C2RF1RF2: f32 = Self::C1C2 * RF1 * RF2;
  const C2RF1: f32 = C2 * RF1;

  pub fn new(sample_rate: f32) -> Self {
    Self {
      bilinear_transform: BilinearTransform::new(sample_rate),
      biquad_filter: BiquadFilter::new(),
    }
  }

  pub fn process(&mut self, input: (f32, f32), tilt: f32) -> (f32, f32) {
    let s_domain_coefficients = self.get_s_domain_coefficients(tilt);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.biquad_filter.process(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, tilt: f32) -> ([f32; 3], [f32; 3]) {
    let r_tilt_a = R_TILT * tilt;
    let r_tilt_b = R_TILT * (1. - tilt);

    let b0 = -Self::C1C2R2RF2 * RF1
      - Self::C1C2R2RF2 * r_tilt_b
      - Self::C1C2R1R2 * r_tilt_b
      - Self::C1C2R2RF2 * R1
      - Self::C1C2RF1RF2 * r_tilt_a
      - Self::C1C2R2RF2 * r_tilt_a;
    let b1 = -Self::C1RF2 * RF1
      - Self::C1RF2 * r_tilt_b
      - Self::C2R2 * r_tilt_b
      - Self::C2R2 * RF2
      - Self::C1R1 * r_tilt_b
      - Self::C1R1 * RF2
      - Self::C1RF2 * r_tilt_a;
    let b2 = -r_tilt_b + -RF2;
    let a0 = Self::C1C2RF1RF2 * r_tilt_b
      + Self::C1C2R1 * RF1 * r_tilt_b
      + Self::C1C2RF1RF2 * R1
      + Self::C1C2R1R2 * RF1
      + Self::C1C2R1 * RF1 * r_tilt_a
      + Self::C1C2R1R2 * r_tilt_a;
    let a1 = Self::C2RF1 * r_tilt_b
      + Self::C2RF1 * RF2
      + Self::C2R2 * RF1
      + Self::C1R1 * RF1
      + Self::C2RF1 * r_tilt_a
      + Self::C2R2 * r_tilt_a
      + Self::C1R1 * r_tilt_a;
    let a2 = RF1 + r_tilt_a;

    ([b0, b1, b2], [a0, a1, a2])
  }
}
