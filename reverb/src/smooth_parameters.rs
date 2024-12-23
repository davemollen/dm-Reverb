mod smooth;
use smooth::ExponentialSmooth;
pub use smooth::Smoother;

pub struct SmoothParameters {
  pub reverse: ExponentialSmooth,
  pub predelay: ExponentialSmooth,
  pub size: ExponentialSmooth,
  pub depth: ExponentialSmooth,
  pub absorb: ExponentialSmooth,
  pub decay: ExponentialSmooth,
  pub tilt: ExponentialSmooth,
  pub shimmer: ExponentialSmooth,
  pub mix: ExponentialSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      reverse: ExponentialSmooth::new(12., sample_rate),
      predelay: ExponentialSmooth::new(7., sample_rate),
      size: ExponentialSmooth::new(2., sample_rate),
      depth: ExponentialSmooth::new(12., sample_rate),
      absorb: ExponentialSmooth::new(12., sample_rate),
      decay: ExponentialSmooth::new(12., sample_rate),
      tilt: ExponentialSmooth::new(12., sample_rate),
      shimmer: ExponentialSmooth::new(12., sample_rate),
      mix: ExponentialSmooth::new(12., sample_rate),
    }
  }

  pub fn set_targets(
    &mut self,
    reverse: f32,
    predelay: f32,
    size: f32,
    depth: f32,
    absorb: f32,
    decay: f32,
    tilt: f32,
    shimmer: f32,
    mix: f32,
  ) {
    self.reverse.set_target(reverse);
    self.predelay.set_target(predelay);
    self.size.set_target(size);
    self.depth.set_target(depth);
    self.absorb.set_target(absorb);
    self.decay.set_target(decay);
    self.tilt.set_target(tilt);
    self.shimmer.set_target(shimmer);
    self.mix.set_target(mix);
  }
}
