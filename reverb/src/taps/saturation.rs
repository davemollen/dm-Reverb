pub struct Saturation;

impl Saturation {
  pub fn process(input: f32, mix: f32) -> f32 {
    let mix = mix.clamp(0., 1.);
    input + (Self::fast_atan2(input) - input) * mix
  }

  fn fast_atan2(x: f32) -> f32 {
    let n1 = 0.97239411;
    let n2 = -0.19194795;
    ((n1 + n2 * x * x) * x).clamp(-1., 1.)
  }
}
