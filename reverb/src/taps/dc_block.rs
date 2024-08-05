use std::simd::f32x4;

pub struct DcBlock {
  sample_period: f32,
  xm1: f32x4,
  ym1: f32x4,
}

impl DcBlock {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_period: sample_rate.recip(),
      xm1: f32x4::splat(0.),
      ym1: f32x4::splat(0.),
    }
  }

  pub fn process(&mut self, x: f32x4) -> f32x4 {
    let coeff = f32x4::splat(1. - (220.5 * self.sample_period));
    let y = x - self.xm1 + coeff * self.ym1;
    self.xm1 = x;
    self.ym1 = y;
    y
  }
}
