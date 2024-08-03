use std::simd::{f32x4, num::SimdFloat};

pub struct Saturation;

impl Saturation {
  pub fn process(taps: f32x4, average: f32) -> f32x4 {
    let mix = f32x4::splat(average.clamp(0., 1.));
    taps + (Self::fast_atan2(taps) - taps) * mix
  }

  fn fast_atan2(x: f32x4) -> f32x4 {
    let limit = f32x4::splat(1.);
    let n1 = f32x4::splat(0.97239411);
    let n2 = f32x4::splat(-0.19194795);
    ((n1 + n2 * x * x) * x).simd_clamp(-limit, limit)
  }
}
