use crate::shared::{
  constants::MIN_PREDELAY,
  delay_line::{DelayLine, Interpolation},
  phasor::Phasor,
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

  pub fn process(&mut self, delay_line: &mut DelayLine, time: f32) -> f32 {
    let reverse_delay = self.reverse_delay(delay_line, time);
    reverse_delay
  }

  fn read_delay_line(
    &mut self,
    delay_line: &mut DelayLine,
    phasor: f32,
    time: f32,
    gain: f32,
  ) -> f32 {
    if gain == 0. {
      0.
    } else {
      delay_line.read(phasor * time, Interpolation::Linear) * gain
    }
  }

  fn reverse_delay(&mut self, delay_line: &mut DelayLine, time: f32) -> f32 {
    let freq = 1000. / time;
    let phasor_a = self.phasor.process(freq) * 2.;
    let phasor_b = (phasor_a + 1.) % 2.;

    let xfade_factor = time / MIN_PREDELAY;
    let xfade_offset = 1. / xfade_factor + 1.;
    let ramp_up = (phasor_a * xfade_factor).min(1.);
    let ramp_down = ((xfade_offset - phasor_a) * xfade_factor).clamp(0., 1.);
    let xfade_a = ramp_up * ramp_down;
    let xfade_b = 1. - xfade_a;

    let reverse_delay_a = self.read_delay_line(delay_line, phasor_a, time, xfade_a);
    let reverse_delay_b = self.read_delay_line(delay_line, phasor_b, time, xfade_b);
    reverse_delay_a + reverse_delay_b
  }
}
