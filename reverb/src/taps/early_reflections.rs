use super::tap::Tap;
use crate::shared::{
  constants::{MAX_SIZE, MIN_SIZE},
  float_ext::FloatExt,
};

const LAST_EARLY_REFLECTION_GAIN: f32 = 0.501187;
const MINUS_THREE_DB: f32 = 0.707946;
const MINUS_FIFTEEN_DB: f32 = 0.177828;

pub struct EarlyReflections {
  reflections: [[f32; 6]; 2],
}

impl EarlyReflections {
  pub fn new() -> Self {
    Self {
      reflections: [
        [0., 0.188, 0.278, 0.38, 0.482, 0.584],
        [0.018, 0.086, 0.29, 0.392, 0.494, 0.597],
      ],
    }
  }

  pub fn run(&mut self, size: f32, taps: &mut [Tap; 4]) -> Vec<f32> {
    let gain = size.scale(MIN_SIZE, MAX_SIZE, MINUS_THREE_DB, MINUS_FIFTEEN_DB);

    taps
      .iter_mut()
      .take(2)
      .zip(self.reflections)
      .map(|(tap, reflections)| self.process_channel(reflections, size, tap, gain))
      .collect()
  }

  fn read_early_reflection(
    &self,
    index: usize,
    size: f32,
    time_fraction: &f32,
    tap: &mut Tap,
  ) -> f32 {
    let attenuation = 1. - (index as f32 / 5. * (1. - LAST_EARLY_REFLECTION_GAIN));
    tap.early_reflection_read(size, *time_fraction) * attenuation
  }

  fn process_channel(&mut self, reflections: [f32; 6], size: f32, tap: &mut Tap, gain: f32) -> f32 {
    reflections
      .iter()
      .enumerate()
      .map(|(index, time_fraction)| self.read_early_reflection(index, size, time_fraction, tap))
      .sum::<f32>()
      * gain
  }
}
