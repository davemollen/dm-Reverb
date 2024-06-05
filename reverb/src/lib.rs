pub mod shared {
  pub mod constants;
  pub mod delay_line;
  pub mod float_ext;
  pub mod param_filter;
  pub mod phasor;
}
mod mix;
mod reverse;
mod smooth_parameters;
mod taps;
mod tilt_filter;
use crate::shared::{
  constants::{MAX_PREDELAY, MIN_PREDELAY},
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};
pub use taps::Taps;
use {mix::Mix, reverse::Reverse, smooth_parameters::SmoothParameters, tilt_filter::TiltFilter};

pub struct Reverb {
  predelay_tap: DelayLine,
  reverse: Reverse,
  taps: Taps,
  tilt_filter: TiltFilter,
  smooth_parameters: SmoothParameters,
}

impl Reverb {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      predelay_tap: DelayLine::new(
        (sample_rate * (MIN_PREDELAY + MAX_PREDELAY) / 1000.) as usize,
        sample_rate,
      ),
      reverse: Reverse::new(sample_rate),
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
    tilt: f32,
    shimmer: f32,
    mix: f32,
  ) {
    self
      .smooth_parameters
      .initialize(reverse, predelay, size, depth, absorb, tilt, shimmer, mix);
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
    let (reverse, predelay, size, depth, absorb, diffuse, tilt, shimmer, mix) = self
      .smooth_parameters
      .process(reverse, predelay, size, depth, absorb, tilt, shimmer, mix);

    let predelay_output = self.get_predelay_output(input, predelay, reverse);
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

  fn get_predelay_output(&mut self, input: (f32, f32), time: f32, reverse: f32) -> f32 {
    let predelay_output = if reverse == 0. {
      self.predelay_tap.read(time, Interpolation::Linear)
    } else if reverse == 1. {
      self.reverse.process(&mut self.predelay_tap, time)
    } else {
      self
        .predelay_tap
        .read(time, Interpolation::Linear)
        .mix(self.reverse.process(&mut self.predelay_tap, time), reverse)
    };
    self.predelay_tap.write((input.0 + input.1) * 0.5);
    predelay_output
  }
}
