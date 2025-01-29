use std::simd::{f32x4, num::SimdFloat};

pub struct OnePoleFilter {
  t: f32,
  z: f32x4,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      t: sample_rate.recip() * 44100.,
      z: f32x4::splat(0.),
    }
  }

  pub fn process(&mut self, input: f32x4, absorb: f32) -> f32x4 {
    let b1 = f32x4::splat(absorb * self.t);
    let a0 = f32x4::splat(1.0) - b1;

    self.z = Self::flush_denormal(input * a0 + self.z * b1);
    self.z
  }

  fn flush_denormal(x: f32x4) -> f32x4 {
    let mask = x.is_subnormal();
    mask.select(f32x4::splat(0.0), x)
  }
}
