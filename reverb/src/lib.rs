#![feature(portable_simd)]
pub mod shared {
  pub mod constants;
  pub mod delay_line;
  pub mod float_ext;
  pub mod phasor;
  pub mod stereo_delay_line;
}
mod mix;
mod predelay;
mod smooth_parameters;
mod taps;
mod tilt_filter;
use {
  mix::Mix, predelay::PreDelay,
  smooth_parameters::Smoother,
  tilt_filter::TiltFilter,
};
pub use {
  smooth_parameters::SmoothParameters,
  taps::{EarlyReflections, Taps}
};

pub struct Reverb {
  predelay: PreDelay,
  taps: Taps,
  tilt_filter: TiltFilter
}

impl Reverb {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      predelay: PreDelay::new(sample_rate),
      taps: Taps::new(sample_rate),
      tilt_filter: TiltFilter::new(sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    speed: f32,
    smooth_parameters: &mut SmoothParameters
  ) -> (f32, f32) {
    let reverse = smooth_parameters.reverse.next();
    let predelay = smooth_parameters.predelay.next();
    let size = smooth_parameters.size.next();
    let depth = smooth_parameters.depth.next();
    let absorb = smooth_parameters.absorb.next();
    let decay = smooth_parameters.decay.next();
    let tilt = smooth_parameters.tilt.next();
    let shimmer = smooth_parameters.shimmer.next();
    let mix = smooth_parameters.mix.next();
    let diffuse = (absorb * 3.).min(1.) * 0.8;
    let absorb = (absorb - 0.3333333).max(0.) * 1.490214; // maximum is 0.993476 which equals a cutoff freq of 50Hz

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
