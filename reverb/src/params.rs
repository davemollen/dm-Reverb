mod smooth;
use smooth::ExponentialSmooth;
pub use smooth::Smoother;

use crate::shared::constants::MAX_DEPTH;

pub struct Params {
  pub reverse: ExponentialSmooth,
  pub predelay: ExponentialSmooth,
  pub size: ExponentialSmooth,
  pub speed: f32,
  pub depth: ExponentialSmooth,
  pub absorb: ExponentialSmooth,
  pub decay: ExponentialSmooth,
  pub tilt: ExponentialSmooth,
  pub shimmer: ExponentialSmooth,
  pub mix: ExponentialSmooth,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      reverse: ExponentialSmooth::new(12., sample_rate),
      predelay: ExponentialSmooth::new(7., sample_rate),
      size: ExponentialSmooth::new(2., sample_rate),
      speed: 0.,
      depth: ExponentialSmooth::new(12., sample_rate),
      absorb: ExponentialSmooth::new(12., sample_rate),
      decay: ExponentialSmooth::new(12., sample_rate),
      tilt: ExponentialSmooth::new(12., sample_rate),
      shimmer: ExponentialSmooth::new(12., sample_rate),
      mix: ExponentialSmooth::new(12., sample_rate),
    }
  }

  pub fn set(
    &mut self,
    reverse: f32,
    predelay: f32,
    size: f32,
    speed: f32,
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
    self.speed = speed;
    self.depth.set_target(depth * depth.abs() * MAX_DEPTH);
    self.absorb.set_target(absorb);
    self.decay.set_target(decay);
    self.tilt.set_target(tilt * tilt.abs() * 0.5 + 0.5);
    self.shimmer.set_target(shimmer);
    self.mix.set_target(mix);
  }
}
