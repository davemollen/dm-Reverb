pub struct Average {
  buffer: Vec<f32>,
  write_pointer: usize,
  previous_mean: f32,
}

impl Average {
  pub fn new(length: usize) -> Self {
    Self {
      buffer: vec![0.0; length],
      write_pointer: 0,
      previous_mean: 0.,
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let n = self.buffer.len();
    let squared = input * input;
    let oldest_buffer_entry = self.get_oldest_buffer_entry();
    let mean = squared + self.previous_mean - oldest_buffer_entry;

    self.previous_mean = mean;
    self.write(squared);

    if mean <= 0. {
      0.
    } else {
      (mean / n as f32).sqrt()
    }
  }

  fn wrap(&self, index: usize) -> usize {
    let buffer_len = self.buffer.len();
    if index >= buffer_len {
      index - buffer_len
    } else {
      index
    }
  }

  fn write(&mut self, value: f32) {
    self.buffer[self.write_pointer] = value;
    self.write_pointer = self.wrap(self.write_pointer + 1);
  }

  fn get_oldest_buffer_entry(&self) -> f32 {
    self.buffer[self.write_pointer]
  }
}
