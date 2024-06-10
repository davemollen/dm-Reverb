mod allpass_filter;
mod dc_block;
mod delay_read;
mod early_reflections;
mod grains;
mod one_pole_filter;
mod saturation_activator;
mod shimmer;

use crate::shared::{
  constants::{MAX_DEPTH, MAX_SIZE},
  delay_line::DelayLine,
  phasor::Phasor,
};
use {
  allpass_filter::AllpassFilter, dc_block::DcBlock, delay_read::DelayRead,
  early_reflections::EarlyReflections, grains::Grains, one_pole_filter::OnePoleFilter,
  saturation_activator::SaturationActivator, shimmer::Shimmer, std::simd::f32x4,
};

const MATRIX: [[f32; 4]; 4] = [
  [1.0, -1.0, -1.0, 1.0],
  [1.0, 1.0, -1.0, -1.0],
  [1.0, -1.0, 1.0, -1.0],
  [1.0, 1.0, 1.0, 1.0],
];

pub struct Taps {
  early_reflections: EarlyReflections,
  delay_lines: [DelayLine; 4],
  time_fractions: [f32; 4],
  diffuser_times: [f32; 4],
  lfo_phase_offsets: [f32; 4],
  grains: [Grains; 4],
  dc_blocks: DcBlock,
  absorbance: OnePoleFilter,
  diffusers: [AllpassFilter; 4],
  lfo_phasor: Phasor,
  saturation_activator: SaturationActivator,
  shimmer: Shimmer,
}

impl Taps {
  pub fn new(sample_rate: f32) -> Self {
    let time_fractions = [
      0.34306569343065696,
      0.48905109489051096,
      0.7372262773722628,
      1.,
    ];

    Self {
      early_reflections: EarlyReflections::new(),
      delay_lines: time_fractions.map(|t| {
        DelayLine::new(
          (sample_rate * (MAX_SIZE * 0.001 * t + MAX_DEPTH)) as usize,
          sample_rate,
        )
      }),
      time_fractions,
      diffuser_times: [
        5.75,
        9.416666666666668,
        13.083333333333332,
        14.916666666666666,
      ],
      grains: [Grains::new(); 4],
      dc_blocks: DcBlock::new(sample_rate),
      absorbance: OnePoleFilter::new(sample_rate),
      diffusers: [
        AllpassFilter::new(sample_rate),
        AllpassFilter::new(sample_rate),
        AllpassFilter::new(sample_rate),
        AllpassFilter::new(sample_rate),
      ],
      lfo_phase_offsets: [0., 0.25, 0.5, 0.75],
      lfo_phasor: Phasor::new(sample_rate),
      shimmer: Shimmer::new(sample_rate),
      saturation_activator: SaturationActivator::new(sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    size: f32,
    speed: f32,
    depth: f32,
    diffuse: f32,
    absorb: f32,
    decay: f32,
    shimmer: f32,
  ) -> (f32, f32) {
    let early_reflections = self.early_reflections.process(size, &mut self.delay_lines);

    let delay_network_outputs = self.read_from_delay_network(size, speed, depth);
    let delay_network_channels = Self::retrieve_channels_from_delay_network(delay_network_outputs);
    self
      .saturation_activator
      .set_amplitude(delay_network_channels);
    let shimmer = self.shimmer.process(input, delay_network_channels, shimmer);
    let feedback_matrix_outputs = Self::apply_matrix(delay_network_outputs);
    self.process_and_write_taps(shimmer, feedback_matrix_outputs, diffuse, absorb, decay);

    self.mix_delay_network_and_reflections(delay_network_channels, early_reflections)
  }

  fn read_from_delay_network(&mut self, size: f32, speed: f32, depth: f32) -> [f32; 4] {
    let phase = self.lfo_phasor.process(speed);

    [
      self.delay_lines[0].delay_network_read(
        size,
        self.time_fractions[0],
        phase,
        self.lfo_phase_offsets[0],
        depth,
        &mut self.grains[0],
      ),
      self.delay_lines[1].delay_network_read(
        size,
        self.time_fractions[1],
        phase,
        self.lfo_phase_offsets[1],
        depth,
        &mut self.grains[1],
      ),
      self.delay_lines[2].delay_network_read(
        size,
        self.time_fractions[2],
        phase,
        self.lfo_phase_offsets[2],
        depth,
        &mut self.grains[2],
      ),
      self.delay_lines[3].delay_network_read(
        size,
        self.time_fractions[3],
        phase,
        self.lfo_phase_offsets[3],
        depth,
        &mut self.grains[3],
      ),
    ]
  }

  fn apply_matrix(input: [f32; 4]) -> f32x4 {
    [
      Self::get_matrix_result(input, MATRIX[0]),
      Self::get_matrix_result(input, MATRIX[1]),
      Self::get_matrix_result(input, MATRIX[2]),
      Self::get_matrix_result(input, MATRIX[3]),
    ]
    .into()
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
    feedback_matrix_outputs: f32x4,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) {
    let saturation_gain = self.saturation_activator.get_saturation_gain();

    let saturated = Self::apply_saturation(feedback_matrix_outputs, saturation_gain);
    let dc_blocked = self.dc_blocks.process(saturated);
    let filtered = self.absorbance.process(
      dc_blocked + f32x4::from_array([input.0, input.1, 0., 0.]),
      absorb,
    );

    filtered
      .to_array()
      .into_iter()
      .enumerate()
      .for_each(|(i, x)| {
        let diffuse_output = self.diffusers[i].process(x, self.diffuser_times[i], diffuse);

        self.delay_lines[i].write(diffuse_output * decay);
      });
  }

  fn retrieve_channels_from_delay_network(inputs: [f32; 4]) -> (f32, f32) {
    ((inputs[0] + inputs[2]) * 0.5, (inputs[1] + inputs[3]) * 0.5)
  }

  fn mix_delay_network_and_reflections(
    &mut self,
    (left_delay_network_out, right_delay_network_out): (f32, f32),
    early_reflections: (f32, f32),
  ) -> (f32, f32) {
    let left_out = left_delay_network_out + early_reflections.0;
    let right_out = right_delay_network_out + early_reflections.1;
    (left_out, right_out)
  }

  fn apply_saturation(input: f32x4, saturation_gain: f32) -> f32x4 {
    let sat_gain = f32x4::splat(saturation_gain);
    let clean_gain = f32x4::splat(1. - saturation_gain);
    let clean_out = input * clean_gain;

    if saturation_gain > 0. {
      Self::fast_atan2(input) * sat_gain + clean_out
    } else {
      clean_out
    }
  }

  fn fast_atan2(x: f32x4) -> f32x4 {
    let n1 = f32x4::splat(0.97239411);
    let n2 = f32x4::splat(-0.19194795);
    (n1 + n2 * x * x) * x
  }
}
