pub struct OnePoleFilter {
  t: f32,
  z: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      t: 44100_f32.recip() * sample_rate,
      z: 0.,
    }
  }

  pub fn process(&mut self, input: f32, r: f32) -> f32 {
    let a0 = (1. - r) * self.t;
    let b1 = 1.0 - a0;
    self.z = input * a0 + self.z * b1;
    self.z
  }
}
