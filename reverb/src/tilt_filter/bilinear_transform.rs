pub struct BilinearTransform {
  s: [f32; 2],
}

impl BilinearTransform {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4.],
    }
  }

  fn bilinear_transform(&self, mut x: [f32; 3]) -> [f32; 3] {
    x[1] *= self.s[0];
    x[2] *= self.s[1];

    [
      x[0] + x[1] + x[2],
      -2. * x[0] + 2. * x[2],
      x[0] - x[1] + x[2],
    ]
  }

  pub fn process(&self, (b, a): ([f32; 3], [f32; 3])) -> ([f32; 3], [f32; 3]) {
    let b = self.bilinear_transform(b);
    let a = self.bilinear_transform(a);
    (b.map(|x| x / a[0]), a.map(|x| x / a[0]))
  }
}
