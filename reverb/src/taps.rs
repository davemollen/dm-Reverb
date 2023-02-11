use crate::{phasor::Phasor, tap::Tap};

pub struct Taps {
  taps: Vec<Tap>,
  lfo_phasor: Phasor,
}

impl Taps {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      taps: vec![
        Tap::new(sample_rate, 0.68, 11., 0.),
        Tap::new(sample_rate, 0.77, 13., 0.25),
        Tap::new(sample_rate, 0.90, 7., 0.5),
        Tap::new(sample_rate, 0.99, 5., 0.75),
      ],
      lfo_phasor: Phasor::new(sample_rate),
    }
  }

  // TODO: add early reflections
  fn read_from_delay_taps(&mut self, size: f32, speed: f32, depth: f32) -> Vec<f32> {
    let phase = self.lfo_phasor.run(speed);
    self
      .taps
      .iter_mut()
      .map(|tap| -> f32 { tap.read(size, phase, depth) })
      .collect()
  }

  fn apply_feedback_matrix<'a>(&self, inputs: &'a Vec<f32>) -> impl Iterator<Item = f32> + 'a {
    [
      [1.0, -1.0, -1.0, 1.0],
      [1.0, 1.0, -1.0, -1.0],
      [1.0, -1.0, 1.0, -1.0],
      [1.0, 1.0, 1.0, 1.0],
    ]
    .iter()
    .map(move |feedback_values| -> f32 {
      feedback_values
        .iter()
        .zip(inputs.iter())
        .map(|(feedback, input)| input * feedback)
        .sum()
    })
  }
  // TODO: test if this has an impact on performance
  //
  // fn apply_feedback_matrix<'a>(&self, inputs: &'a Vec<f32>) -> [f32; 4] {
  //   if let [first, second, third, fourth] = inputs.as_slice() {
  //     let a = first - second;
  //     let b = first + second;
  //     let c = third - fourth;
  //     let d = third + fourth;
  //     [a - c, b - d, a + c, b + d]
  //   } else {
  //     panic!("Feedback matrix should receive a vector with four input signals")
  //   }
  // }

  fn process_and_write_taps<'a>(
    &'a mut self,
    input: f32,
    feedback_matrix_outputs: impl Iterator<Item = f32> + 'a,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) {
    self
      .taps
      .iter_mut()
      .zip([input, input, 0., 0.].iter())
      .zip(feedback_matrix_outputs)
      .for_each(|((tap, dry_signal), feedback_matrix_output)| {
        let saturation_output = tap.apply_saturation(feedback_matrix_output, decay);
        let absorb_output = tap.apply_absorb(dry_signal + saturation_output, absorb);
        let diffuse_output = tap.apply_diffuse(absorb_output, diffuse);
        tap.write(diffuse_output);
      });
  }

  fn get_stereo_output(&mut self, inputs: Vec<f32>) -> (f32, f32) {
    let left_out = (inputs[0] + inputs[2]) * 0.7071;
    let right_out = (inputs[1] + inputs[3]) * 0.7071;
    (left_out, right_out)
  }

  pub fn run(
    &mut self,
    input: f32,
    size: f32,
    speed: f32,
    depth: f32,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) -> (f32, f32) {
    let read_outputs = self.read_from_delay_taps(size, speed, depth);
    let feedback_matrix_outputs = self.apply_feedback_matrix(&read_outputs);
    self.process_and_write_taps(input, feedback_matrix_outputs, diffuse, absorb, decay);
    self.get_stereo_output(read_outputs)
  }
}
