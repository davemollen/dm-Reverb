use std::{
  f32,
  simd::{f32x4, num::SimdFloat},
};

pub struct DcBlock {
  coeff: f32x4,
  xm1: f32x4,
  ym1: f32x4,
}

impl DcBlock {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      coeff: f32x4::splat(1. - (220.5 / sample_rate)),
      xm1: f32x4::splat(0.),
      ym1: f32x4::splat(0.),
    }
  }

  pub fn process(&mut self, x: f32x4) -> f32x4 {
    if (x - self.xm1).abs() <= f32x4::splat(f32::EPSILON) {
      self.xm1 = x;
      return x;
    }
    let y = x - self.xm1 + self.coeff * self.ym1;
    self.xm1 = x;
    self.ym1 = y;
    y
  }
}
