mod biquad_filter;
use biquad_filter::BiquadFilter;
mod bilinear_transform;
use bilinear_transform::BilinearTransform;

pub struct TiltFilter {
  bilinear_transform: BilinearTransform,
  biquad_filter: BiquadFilter,
}

impl TiltFilter {
  const C1: f32 = 5.6e-9;
  const C2: f32 = 5.6e-9;
  const R1: f32 = 2250.;
  const R2: f32 = 2250.;
  const RF1: f32 = 47000.;
  const RF2: f32 = 47000.;
  const R_TILT: f32 = 140000.;

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
    let r_tilt_a = Self::R_TILT * tilt;
    let r_tilt_b = Self::R_TILT * (1. - tilt);

    let c1c2 = Self::C1 * Self::C2;
    let c1c2r1 = c1c2 * Self::R1;
    let c1c2r1r2 = c1c2r1 * Self::R2;
    let c1c2r2 = c1c2 * Self::R2;
    let c1c2r2rf2 = c1c2r2 * Self::RF2;
    let c2r2 = Self::C2 * Self::R2;
    let c1r1 = Self::C1 * Self::R1;
    let c1rf2 = Self::C1 * Self::RF2;
    let c1c2rf1rf2 = c1c2 * Self::RF1 * Self::RF2;
    let c2rf1 = Self::C2 * Self::RF1;

    let b0 = -c1c2r2rf2 * Self::RF1
      + -c1c2r2rf2 * r_tilt_b
      + -c1c2r1r2 * r_tilt_b
      + -c1c2r2rf2 * Self::R1
      + -c1c2rf1rf2 * r_tilt_a
      + -c1c2r2rf2 * r_tilt_a;
    let b1 = -c1rf2 * Self::RF1
      + -c1rf2 * r_tilt_b
      + -c2r2 * r_tilt_b
      + -c2r2 * Self::RF2
      + -c1r1 * r_tilt_b
      + -c1r1 * Self::RF2
      + -c1rf2 * r_tilt_a;
    let b2 = -r_tilt_b + -Self::RF2;
    let a0 = c1c2rf1rf2 * r_tilt_b
      + c1c2r1 * Self::RF1 * r_tilt_b
      + c1c2rf1rf2 * Self::R1
      + c1c2r1r2 * Self::RF1
      + c1c2r1 * Self::RF1 * r_tilt_a
      + c1c2r1r2 * r_tilt_a;
    let a1 = c2rf1 * r_tilt_b
      + c2rf1 * Self::RF2
      + c2r2 * Self::RF1
      + c1r1 * Self::RF1
      + c2rf1 * r_tilt_a
      + c2r2 * r_tilt_a
      + c1r1 * r_tilt_a;
    let a2 = Self::RF1 + r_tilt_a;

    ([b0, b1, b2], [a0, a1, a2])
  }
}
