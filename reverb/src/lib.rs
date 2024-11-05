#![feature(portable_simd)]
pub mod shared {
  pub mod constants;
  pub mod delay_line;
  pub mod float_ext;
  pub mod param_filter;
  pub mod phasor;
  pub mod stereo_delay_line;
}
mod mix;
mod predelay;
mod smooth_parameters;
mod taps;
mod tilt_filter;

pub use taps::{EarlyReflections, Taps};
use {
  mix::Mix, predelay::PreDelay, shared::float_ext::FloatExt, smooth_parameters::SmoothParameters,
  tilt_filter::TiltFilter,
};

pub struct Reverb {
  predelay: PreDelay,
  taps: Taps,
  tilt_filter: TiltFilter,
  smooth_parameters: SmoothParameters,
}

impl Reverb {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      predelay: PreDelay::new(sample_rate),
      taps: Taps::new(sample_rate),
      tilt_filter: TiltFilter::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn initialize_params(
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
    self.smooth_parameters.initialize(
      reverse, predelay, size, depth, absorb, decay, tilt, shimmer, mix,
    );
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
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
  ) -> (f32, f32) {
    let (reverse, predelay, size, depth, absorb, decay, diffuse, tilt, shimmer, mix) =
      self.smooth_parameters.process(
        reverse, predelay, size, depth, absorb, decay, tilt, shimmer, mix,
      );

    let predelay_output = self.predelay.process(input, predelay, reverse);
    let taps_output = self.taps.process(
      predelay_output,
      size,
      speed,
      depth,
      diffuse,
      absorb,
      decay,
      shimmer,
    );

    let tilt_filter_output = self.tilt_filter.process(taps_output, tilt);
    Mix::process(input, tilt_filter_output, mix)
  }
}
