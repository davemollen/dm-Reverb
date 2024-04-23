pub struct BiquadFilter {
  z1: f32,
  z2: f32,
}

impl BiquadFilter {
  pub fn new() -> Self {
    Self { z1: 0., z2: 0. }
  }

  pub fn process(&mut self, input: f32, a0: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
    let output = input * a0 + self.z1;
    self.z1 = input * a1 + self.z2 - b1 * output;
    self.z2 = input * a2 - b2 * output;
    output
  }
}
