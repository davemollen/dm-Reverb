mod early_reflections;
mod saturation_activator;
mod shimmer;
mod tap;
use crate::shared::phasor::Phasor;
use {
  early_reflections::EarlyReflections, saturation_activator::SaturationActivator, shimmer::Shimmer,
  tap::Tap,
};

const MATRIX: [[f32; 4]; 4] = [
  [1.0, -1.0, -1.0, 1.0],
  [1.0, 1.0, -1.0, -1.0],
  [1.0, -1.0, 1.0, -1.0],
  [1.0, 1.0, 1.0, 1.0],
];

pub struct Taps {
  early_reflections: EarlyReflections,
  taps: [Tap; 4],
  lfo_phasor: Phasor,
  saturation_activator: SaturationActivator,
  shimmer: Shimmer,
}

impl Taps {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      early_reflections: EarlyReflections::new(),
      taps: [
        Tap::new(sample_rate, 0.34306569343065696, 5.75, 0.),
        Tap::new(sample_rate, 0.48905109489051096, 9.416666666666668, 0.25),
        Tap::new(sample_rate, 0.7372262773722628, 13.083333333333332, 0.5),
        Tap::new(sample_rate, 1., 14.916666666666666, 0.75),
      ],
      lfo_phasor: Phasor::new(sample_rate),
      shimmer: Shimmer::new(sample_rate),
      saturation_activator: SaturationActivator::new(sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: f32,
    size: f32,
    speed: f32,
    depth: f32,
    diffuse: f32,
    absorb: f32,
    decay: f32,
    shimmer: f32,
  ) -> (f32, f32) {
    let early_reflections_outputs = self.early_reflections.process(size, &mut self.taps);

    let delay_network_outputs = self.read_from_delay_network(size, speed, depth);
    let delay_network_channels = Self::retrieve_channels_from_delay_network(delay_network_outputs);
    self
      .saturation_activator
      .set_amplitude(delay_network_channels);

    let shimmer = self
      .shimmer
      .process((input, input), delay_network_channels, shimmer);
    let feedback_matrix_outputs = Self::apply_matrix(delay_network_outputs);
    self.process_and_write_taps(shimmer, feedback_matrix_outputs, diffuse, absorb, decay);

    self.mix_delay_network_and_reflections(delay_network_channels, early_reflections_outputs)
  }

  fn read_from_delay_network(&mut self, size: f32, speed: f32, depth: f32) -> [f32; 4] {
    let phase = self.lfo_phasor.process(speed);

    [
      self.taps[0].delay_network_read(size, phase, depth),
      self.taps[1].delay_network_read(size, phase, depth),
      self.taps[2].delay_network_read(size, phase, depth),
      self.taps[3].delay_network_read(size, phase, depth),
    ]
  }

  fn apply_matrix(input: [f32; 4]) -> impl IntoIterator<Item = f32> {
    MATRIX
      .into_iter()
      .map(move |matrix_element| Self::get_matrix_result(input, matrix_element))
  }

  fn get_matrix_result(inputs: [f32; 4], matrix: [f32; 4]) -> f32 {
    inputs
      .into_iter()
      .zip(matrix)
      .map(|(input, factor)| input * factor)
      .sum()
  }

  fn process_and_write_taps(
    &mut self,
    input: (f32, f32),
    feedback_matrix_outputs: impl IntoIterator<Item = f32>,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) {
    let saturation_gain = self.saturation_activator.get_saturation_gain();

    feedback_matrix_outputs
      .into_iter()
      .zip(self.taps.iter_mut())
      .zip([input.0, input.1, 0., 0.])
      .for_each(|((feedback_matrix_output, tap), dry_signal)| {
        let saturation_output =
          tap.apply_saturation(feedback_matrix_output, decay, saturation_gain);
        let absorb_output = tap.apply_absorb(dry_signal + saturation_output, absorb);
        let diffuse_output = tap.apply_diffuse(absorb_output, diffuse);
        tap.write(diffuse_output);
      });
  }

  fn retrieve_channels_from_delay_network(inputs: [f32; 4]) -> (f32, f32) {
    (inputs[0] + inputs[2], inputs[1] + inputs[3])
  }

  fn mix_delay_network_and_reflections(
    &mut self,
    (left_delay_network_out, right_delay_network_out): (f32, f32),
    early_reflections_output: Vec<f32>,
  ) -> (f32, f32) {
    let left_out = (left_delay_network_out + early_reflections_output[0]) * 0.5;
    let right_out = (right_delay_network_out + early_reflections_output[1]) * 0.5;
    (left_out, right_out)
  }
}
