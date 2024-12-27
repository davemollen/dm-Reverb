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
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      reverse: ExponentialSmooth::new(sample_rate, 12.),
      predelay: ExponentialSmooth::new(sample_rate, 7.),
      size: ExponentialSmooth::new(sample_rate, 2.),
      speed: 0.,
      depth: ExponentialSmooth::new(sample_rate, 12.),
      absorb: ExponentialSmooth::new(sample_rate, 12.),
      decay: ExponentialSmooth::new(sample_rate, 12.),
      tilt: ExponentialSmooth::new(sample_rate, 12.),
      shimmer: ExponentialSmooth::new(sample_rate, 12.),
      mix: ExponentialSmooth::new(sample_rate, 12.),
      is_initialized: false,
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
    self.speed = speed;
    let depth = depth * depth.abs() * MAX_DEPTH;
    let tilt = tilt * tilt.abs() * 0.5 + 0.5;

    if self.is_initialized {
      self.reverse.set_target(reverse);
      self.predelay.set_target(predelay);
      self.size.set_target(size);
      self.depth.set_target(depth);
      self.absorb.set_target(absorb);
      self.decay.set_target(decay);
      self.tilt.set_target(tilt);
      self.shimmer.set_target(shimmer);
      self.mix.set_target(mix);
    } else {
      self.reverse.reset(reverse);
      self.predelay.reset(predelay);
      self.size.reset(size);
      self.depth.reset(depth);
      self.absorb.reset(absorb);
      self.decay.reset(decay);
      self.tilt.reset(tilt);
      self.shimmer.reset(shimmer);
      self.mix.reset(mix);
      self.is_initialized = true;
    }
  }
}
