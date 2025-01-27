mod reverse;
use crate::shared::{
  constants::{MAX_PREDELAY, MIN_PREDELAY},
  stereo_delay_line::{Interpolation, StereoDelayLine},
};
use reverse::Reverse;

pub struct PreDelay {
  delay_line: StereoDelayLine,
  reverse: Reverse,
}

impl PreDelay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: StereoDelayLine::new(
        (sample_rate * (MIN_PREDELAY + MAX_PREDELAY) / 1000.) as usize,
        sample_rate,
      ),
      reverse: Reverse::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: (f32, f32), time: f32, reverse: f32) -> (f32, f32) {
    let predelay_output = if reverse == 0. {
      self.delay_line.read(time, Interpolation::Linear)
    } else if reverse == 1. {
      self.reverse.process(&self.delay_line, time)
    } else {
      Self::mix(
        self.delay_line.read(time, Interpolation::Linear),
        self.reverse.process(&self.delay_line, time),
        reverse,
      )
    };
    self.delay_line.write(input);
    predelay_output
  }

  fn mix(left: (f32, f32), right: (f32, f32), factor: f32) -> (f32, f32) {
    (
      left.0 + (right.0 - left.0) * factor,
      left.1 + (right.1 - left.1) * factor,
    )
  }
}
