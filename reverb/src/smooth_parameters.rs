use crate::shared::param_filter::ParamFilter;

pub struct SmoothParameters {
  smooth_reverse: ParamFilter,
  smooth_predelay: ParamFilter,
  smooth_size: ParamFilter,
  smooth_depth: ParamFilter,
  smooth_absorb: ParamFilter,
  smooth_tilt: ParamFilter,
  smooth_shimmer: ParamFilter,
  smooth_mix: ParamFilter,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_reverse: ParamFilter::new(sample_rate, 12.),
      smooth_predelay: ParamFilter::new(sample_rate, 7.),
      smooth_size: ParamFilter::new(sample_rate, 2.),
      smooth_depth: ParamFilter::new(sample_rate, 12.),
      smooth_absorb: ParamFilter::new(sample_rate, 12.),
      smooth_tilt: ParamFilter::new(sample_rate, 12.),
      smooth_shimmer: ParamFilter::new(sample_rate, 12.),
      smooth_mix: ParamFilter::new(sample_rate, 12.),
    }
  }

  pub fn initialize(
    &mut self,
    reverse: f32,
    predelay: f32,
    size: f32,
    depth: f32,
    absorb: f32,
    tilt: f32,
    shimmer: f32,
    mix: f32,
  ) {
    self.smooth_reverse.initialize(reverse);
    self.smooth_predelay.initialize(predelay);
    self.smooth_size.initialize(size);
    self.smooth_depth.initialize(depth);
    self.smooth_absorb.initialize(absorb);
    self.smooth_tilt.initialize(tilt);
    self.smooth_shimmer.initialize(shimmer);
    self.smooth_mix.initialize(mix);
  }

  pub fn process(
    &mut self,
    reverse: f32,
    predelay: f32,
    size: f32,
    depth: f32,
    absorb: f32,
    tilt: f32,
    shimmer: f32,
    mix: f32,
  ) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    let reverse = self.smooth_reverse.process(reverse);
    let predelay = self.smooth_predelay.process(predelay);
    let size = self.smooth_size.process(size);
    let depth = self.smooth_depth.process(depth);
    let absorb = self.smooth_absorb.process(absorb);
    let tilt = self.smooth_tilt.process(tilt);
    let shimmer = self.smooth_shimmer.process(shimmer);
    let mix = self.smooth_mix.process(mix);

    let diffuse = (absorb * 3.).min(1.) * 0.8;
    let absorb = (absorb - 0.3333333).max(0.) * 1.5;

    (
      reverse, predelay, size, depth, absorb, diffuse, tilt, shimmer, mix,
    )
  }
}
