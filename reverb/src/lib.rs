pub mod shared {
  pub mod constants;
  pub mod delay_line;
  pub mod float_ext;
  pub mod one_pole_filter;
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
use {
  mix::Mix, reverse::Reverse, smooth_parameters::SmoothParameters, taps::Taps,
  tilt_filter::TiltFilter,
};

const TWELVE_DB: f32 = 3.981072;
const TWENTY_FOUR_DB: f32 = 15.848932;

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

  pub fn run(
    &mut self,
    input: (f32, f32),
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
  ) -> (f32, f32) {
    let (reverse, predelay, size, speed, depth, absorb, diffuse, decay, tilt, shimmer, mix) =
      self.smooth_parameters.run(
        reverse, predelay, size, speed, depth, absorb, decay, tilt, shimmer, mix,
      );

    let predelay_output = self.get_predelay_output(input, predelay, reverse);
    let taps_output = self.taps.run(
      predelay_output,
      size,
      speed,
      depth,
      diffuse,
      absorb,
      decay,
      shimmer,
    );

    let tilt_filter_output = self.apply_tilt_filter(taps_output, tilt);
    Mix::run(input, tilt_filter_output, mix)
  }

  fn get_predelay_output(&mut self, input: (f32, f32), time: f32, reverse: f32) -> f32 {
    let predelay_output = if reverse == 0. {
      self.predelay_tap.read(time, Interpolation::Linear)
    } else if reverse == 1. {
      self.reverse.run(&mut self.predelay_tap, time)
    } else {
      self
        .predelay_tap
        .read(time, Interpolation::Linear)
        .mix(self.reverse.run(&mut self.predelay_tap, time), reverse)
    };
    self.predelay_tap.write((input.0 + input.1) * 0.5);
    predelay_output
  }

  fn apply_tilt_filter(&mut self, input: (f32, f32), tilt: f32) -> (f32, f32) {
    if tilt == 0. {
      input
    } else {
      self
        .tilt_filter
        .run(input, 520., 4000., TWELVE_DB, TWENTY_FOUR_DB, tilt)
    }
  }
}
