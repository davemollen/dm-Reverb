mod allpass_filter;
mod average;
mod dc_block;
mod delay_read;
mod early_reflections;
mod grains;
mod one_pole_filter;
mod saturation;
mod shimmer;

use {
  crate::shared::{
    constants::{MAX_DEPTH, MAX_SIZE},
    delay_line::DelayLine,
    float_ext::FloatExt,
    phasor::Phasor,
  },
  allpass_filter::AllpassFilter,
  average::Average,
  dc_block::DcBlock,
  delay_read::DelayRead,
  early_reflections::EarlyReflections,
  grains::Grains,
  one_pole_filter::OnePoleFilter,
  saturation::Saturation,
  shimmer::Shimmer,
  std::simd::{f32x4, num::SimdFloat},
};

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
  average: Average,
  shimmer: Shimmer,
}

impl Taps {
  const MATRIX: [f32x4; 4] = [
    f32x4::from_array([0.5, -0.5, -0.5, 0.5]),
    f32x4::from_array([0.5, 0.5, -0.5, -0.5]),
    f32x4::from_array([0.5, -0.5, 0.5, -0.5]),
    f32x4::from_array([0.5, 0.5, 0.5, 0.5]),
  ];

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
      average: Average::new(21_f32.mstosamps(sample_rate) as usize),
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

    let delay_network_taps = self.read_from_delay_network(size, speed, depth);
    let delay_network_output = Self::retrieve_delay_network_output(delay_network_taps);
    let average = self.average.process(delay_network_taps.abs().reduce_max());

    let matrix_output = Self::apply_matrix(delay_network_taps);
    let shimmer_output = self.shimmer.process(input, delay_network_output, shimmer);
    let dc_block_output = self.dc_blocks.process(matrix_output);
    let absorb_output = self.absorbance.process(
      dc_block_output + f32x4::from_array([shimmer_output.0, shimmer_output.1, 0., 0.]),
      absorb,
    );
    self.diffuse_and_write(absorb_output, diffuse, decay, average);

    self.mix_delay_network_and_reflections(
      delay_network_output,
      early_reflections,
      Self::retrieve_gain_compensation(average, 0.5),
    )
  }

  fn read_from_delay_network(&mut self, size: f32, speed: f32, depth: f32) -> f32x4 {
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
    .into()
  }

  fn apply_matrix(input: f32x4) -> f32x4 {
    f32x4::from_array([
      (Self::MATRIX[0] * input).reduce_sum(),
      (Self::MATRIX[1] * input).reduce_sum(),
      (Self::MATRIX[2] * input).reduce_sum(),
      (Self::MATRIX[3] * input).reduce_sum(),
    ])
  }

  fn diffuse_and_write(&mut self, input: f32x4, diffuse: f32, decay: f32, saturation_mix: f32) {
    input.to_array().into_iter().enumerate().for_each(|(i, x)| {
      let diffuse_output = self.diffusers[i].process(x, self.diffuser_times[i], diffuse);
      let saturation_output = Saturation::process(diffuse_output * decay, saturation_mix);

      self.delay_lines[i].write(saturation_output);
    });
  }

  fn retrieve_delay_network_output(inputs: f32x4) -> (f32, f32) {
    ((inputs[0] + inputs[2]) * 0.5, (inputs[1] + inputs[3]) * 0.5)
  }

  fn mix_delay_network_and_reflections(
    &mut self,
    (left_delay_network_out, right_delay_network_out): (f32, f32),
    early_reflections: (f32, f32),
    gain_compensation: f32,
  ) -> (f32, f32) {
    let left_out = left_delay_network_out + early_reflections.0;
    let right_out = right_delay_network_out + early_reflections.1;
    (left_out * gain_compensation, right_out * gain_compensation)
  }

  fn retrieve_gain_compensation(average: f32, threshold: f32) -> f32 {
    if average > threshold {
      threshold / average
    } else {
      1.
    }
  }
}
