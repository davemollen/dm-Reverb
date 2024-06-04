pub struct BiquadFilter {
  z: [(f32, f32); 2],
}

impl BiquadFilter {
  pub fn new() -> Self {
    Self { z: [(0.0, 0.0); 2] }
  }

  pub fn process(&mut self, x: (f32, f32), (b, a): ([f32; 3], [f32; 3])) -> (f32, f32) {
    let y = (x.0 * b[0] + self.z[0].0, x.1 * b[0] + self.z[0].1);
    self.z[0] = (
      x.0 * b[1] - y.0 * a[1] + self.z[1].0,
      x.1 * b[1] - y.1 * a[1] + self.z[1].1,
    );
    self.z[1] = (x.0 * b[2] - y.0 * a[2], x.1 * b[2] - y.1 * a[2]);

    y
  }
}
