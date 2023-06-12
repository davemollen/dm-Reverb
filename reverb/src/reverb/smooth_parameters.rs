use crate::shared::{
  constants::MAX_DEPTH,
  one_pole_filter::{Mode, OnePoleFilter},
};

pub struct SmoothParameters {
  smooth_reverse: OnePoleFilter,
  smooth_predelay: OnePoleFilter,
  smooth_size: OnePoleFilter,
  smooth_depth: OnePoleFilter,
  smooth_absorb: OnePoleFilter,
  smooth_tilt: OnePoleFilter,
  smooth_shimmer: OnePoleFilter,
  smooth_mix: OnePoleFilter,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_reverse: OnePoleFilter::new(sample_rate),
      smooth_predelay: OnePoleFilter::new(sample_rate),
      smooth_size: OnePoleFilter::new(sample_rate),
      smooth_depth: OnePoleFilter::new(sample_rate),
      smooth_absorb: OnePoleFilter::new(sample_rate),
      smooth_tilt: OnePoleFilter::new(sample_rate),
      smooth_shimmer: OnePoleFilter::new(sample_rate),
      smooth_mix: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn run(
    &mut self,
    reverse: bool,
    predelay: f32,
    size: f32,
    speed: f32,
    depth: f32,
    absorb: f32,
    decay: f32,
    tilt: f32,
    shimmer: f32,
    mix: f32,
  ) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    let reverse = self
      .smooth_reverse
      .run(if reverse { 1. } else { 0. }, 12., Mode::Hertz);
    let predelay = self.smooth_predelay.run(predelay, 7., Mode::Hertz);
    let size = self.smooth_size.run(size, 4., Mode::Hertz);
    let depth = self
      .smooth_depth
      .run(depth * depth * depth.signum() * MAX_DEPTH, 12., Mode::Hertz);
    let absorb = self.smooth_absorb.run(absorb, 12., Mode::Hertz);
    let tilt = self.smooth_tilt.run(tilt, 12., Mode::Hertz);
    let shimmer = self.smooth_shimmer.run(shimmer, 12., Mode::Hertz);
    let mix = self.smooth_mix.run(mix, 12., Mode::Hertz);
    let diffuse = (absorb * 3.).min(1.) * 0.8;
    let absorb = (absorb - 0.3333333).max(0.) * 1.5;

    (
      reverse, predelay, size, speed, depth, absorb, diffuse, decay, tilt, shimmer, mix,
    )
  }
}
