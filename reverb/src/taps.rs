use crate::{
  envelope_follower::EnvelopeFollower,
  float_ext::FloatExt,
  one_pole_filter::{Mode, OnePoleFilter},
  phasor::Phasor,
  shimmer::Shimmer,
  tap::Tap,
  MAX_SIZE, MIN_SIZE,
};

pub struct Taps {
  taps: [Tap; 4],
  lfo_phasor: Phasor,
  shimmer: Shimmer,
  envelope_follower: EnvelopeFollower,
  smooth_saturation_gain: OnePoleFilter,
}

impl Taps {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      taps: [
        Tap::new(
          sample_rate,
          0.34306569343065696,
          vec![(0., 1., 0.), (0.226, 0.917, -8.)],
          5.75,
          0.,
        ),
        Tap::new(
          sample_rate,
          0.48905109489051096,
          vec![(0.453, 0.891, 8.)],
          9.416666666666668,
          0.25,
        ),
        Tap::new(
          sample_rate,
          0.7372262773722628,
          vec![(0., 0.841, 16.), (0.113, 0.794, -16.)],
          13.083333333333332,
          0.5,
        ),
        Tap::new(
          sample_rate,
          1.,
          vec![(0., 0.75, -32.), (0.155, 0.7071, 32.)],
          14.916666666666666,
          0.75,
        ),
      ],
      lfo_phasor: Phasor::new(sample_rate),
      shimmer: Shimmer::new(sample_rate),
      envelope_follower: EnvelopeFollower::new(sample_rate),
      smooth_saturation_gain: OnePoleFilter::new(sample_rate),
    }
  }

  fn read_early_reflections(&mut self, size: f32) -> (f32, f32) {
    let early_reflections = self.taps.iter_mut().fold((0., 0.), |sum, tap| {
      let early_reflections = tap.read_early_reflections(size);
      (sum.0 + early_reflections.0, sum.1 + early_reflections.1)
    });
    let gain = size.scale(MIN_SIZE, MAX_SIZE, -6., -15.).dbtoa();

    (early_reflections.0 * gain, early_reflections.1 * gain)
  }

  fn read_from_delay_network(&mut self, size: f32, speed: f32, depth: f32) -> Vec<f32> {
    let phase = self.lfo_phasor.run(speed);
    self
      .taps
      .iter_mut()
      .map(|tap| tap.read(size, phase, depth))
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
        .zip(inputs)
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
    let saturation_gain = self.get_saturation_gain();

    self
      .taps
      .iter_mut()
      .zip([input, input, 0., 0.])
      .zip(feedback_matrix_outputs)
      .for_each(|((tap, dry_signal), feedback_matrix_output)| {
        let saturation_output =
          tap.apply_saturation(feedback_matrix_output, decay, saturation_gain);
        let absorb_output = tap.apply_absorb(dry_signal + saturation_output, absorb);
        let diffuse_output = tap.apply_diffuse(absorb_output, diffuse);
        tap.write(diffuse_output);
      });
  }

  fn get_saturation_gain(&mut self) -> f32 {
    let saturation_gain = if self.envelope_follower.get_value() > 0.5 {
      1.
    } else {
      0.
    };

    self
      .smooth_saturation_gain
      .run(saturation_gain, 7., Mode::Hertz)
  }

  fn get_stereo_output(
    &mut self,
    inputs: Vec<f32>,
    early_reflections_output: (f32, f32),
    shimmer: f32,
  ) -> (f32, f32) {
    let left_delay_network_out = inputs[0] + inputs[2];
    let right_delay_network_out = inputs[1] + inputs[3];
    self
      .envelope_follower
      .run(left_delay_network_out + right_delay_network_out);

    let left_out = (left_delay_network_out + early_reflections_output.0) * 0.5;
    let right_out = (right_delay_network_out + early_reflections_output.1) * 0.5;
    let out = (left_out, right_out);
    self.shimmer.run(out, shimmer)
  }

  pub fn run(
    &mut self,
    input: f32,
    size: f32,
    speed: f32,
    depth: f32,
    shimmer: f32,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) -> (f32, f32) {
    let delay_network_outputs = self.read_from_delay_network(size, speed, depth);
    let early_reflections_outputs = self.read_early_reflections(size);
    let feedback_matrix_outputs = self.apply_feedback_matrix(&delay_network_outputs);
    self.process_and_write_taps(input, feedback_matrix_outputs, diffuse, absorb, decay);
    self.get_stereo_output(delay_network_outputs, early_reflections_outputs, shimmer)
  }
}
