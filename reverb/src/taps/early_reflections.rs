use crate::shared::{
  constants::{MAX_SIZE, MIN_SIZE},
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};

const LAST_REFLECTION_GAIN_IN_DB: f32 = -6.;
const MINUS_NINE_DB: f32 = 0.707946;
const MINUS_TWENTY_ONE_DB: f32 = 0.089125;

pub struct EarlyReflections {
  reflections: [[f32; 6]; 2],
  attenuations: [f32; 6],
}

impl EarlyReflections {
  const SIZE_MULTIPLIER: f32 = (-MINUS_NINE_DB + MINUS_TWENTY_ONE_DB) / MAX_SIZE;

  pub fn new() -> Self {
    let reflections = [
      [0., 0.188, 0.278, 0.38, 0.482, 0.584],
      [0.018, 0.086, 0.29, 0.392, 0.494, 0.597],
    ];
    debug_assert!(reflections[0].len() == reflections[1].len());
    let length = reflections[0].len();

    Self {
      reflections,
      attenuations: (0..length)
        .map(|index| (index as f32 / length as f32 * LAST_REFLECTION_GAIN_IN_DB).dbtoa())
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap(),
    }
  }

  pub fn process(&mut self, size: f32, taps: &mut [DelayLine; 4]) -> (f32, f32) {
    let gain = (size - MIN_SIZE) * Self::SIZE_MULTIPLIER + MINUS_NINE_DB;

    (
      self.reflections[0]
        .into_iter()
        .zip(self.attenuations)
        .map(|(time_factor, attenuation)| {
          taps[0].read(size * time_factor, Interpolation::Linear) * attenuation
        })
        .sum::<f32>()
        * gain,
      self.reflections[1]
        .into_iter()
        .zip(self.attenuations)
        .map(|(time_factor, attenuation)| {
          taps[1].read(size * time_factor, Interpolation::Linear) * attenuation
        })
        .sum::<f32>()
        * gain,
    )
  }
}
