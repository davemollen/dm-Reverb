use std::simd::f32x4;

pub struct Saturation;

impl Saturation {
  pub fn process(taps: f32x4, mix: f32) -> f32x4 {
    let mix = f32x4::splat((mix * mix).clamp(0., 1.));
    taps + (Self::fast_atan2(taps) - taps) * mix
  }

  fn fast_atan2(x: f32x4) -> f32x4 {
    let input_limit = f32x4::splat(2.65155);
    let output_limit = f32x4::splat(1.);

    if x < -input_limit {
      output_limit
    } else if x > input_limit {
      -output_limit
    } else {
      (f32x4::splat(0.97239411) - f32x4::splat(0.19194795) * x * x) * x
    }
  }
}
