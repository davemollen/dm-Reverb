use crate::shared::{
  constants::MIN_PREDELAY,
  phasor::Phasor,
  stereo_delay_line::{Interpolation, StereoDelayLine},
};

pub struct Reverse {
  phasor: Phasor,
}

impl Reverse {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      phasor: Phasor::new(sample_rate),
    }
  }

  pub fn process(&mut self, delay_line: &mut StereoDelayLine, time: f32) -> (f32, f32) {
    let freq = 1000. / time;
    let phasor_a = self.phasor.process(freq) * 2.;
    let phasor_b = Self::wrap(phasor_a + 1.);

    let xfade_factor = time / MIN_PREDELAY;
    let xfade_offset = xfade_factor.recip() + 1.;
    let ramp_up = (phasor_a * xfade_factor).min(1.);
    let ramp_down = ((xfade_offset - phasor_a) * xfade_factor).clamp(0., 1.);
    let xfade_a = ramp_up * ramp_down;
    let xfade_b = 1. - xfade_a;

    let reverse_delay_a = self.read_delay_line(delay_line, phasor_a, time, xfade_a);
    let reverse_delay_b = self.read_delay_line(delay_line, phasor_b, time, xfade_b);
    (
      reverse_delay_a.0 + reverse_delay_b.0,
      reverse_delay_a.1 + reverse_delay_b.1,
    )
  }

  fn read_delay_line(
    &mut self,
    delay_line: &mut StereoDelayLine,
    phasor: f32,
    time: f32,
    gain: f32,
  ) -> (f32, f32) {
    if gain == 0. {
      (0., 0.)
    } else {
      let delay_out = delay_line.read(phasor * time, Interpolation::Linear);
      (delay_out.0 * gain, delay_out.1 * gain)
    }
  }

  fn wrap(x: f32) -> f32 {
    if x >= 2. {
      x - 2.
    } else {
      x
    }
  }
}
