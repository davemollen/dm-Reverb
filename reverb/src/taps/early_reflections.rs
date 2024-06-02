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
  attenuations: [f32; 6],
}

impl EarlyReflections {
  pub fn new() -> Self {
    Self {
      reflections: [
        [0., 0.188, 0.278, 0.38, 0.482, 0.584],
        [0.018, 0.086, 0.29, 0.392, 0.494, 0.597],
      ],
      attenuations: [
        Self::get_attenuation(0),
        Self::get_attenuation(1),
        Self::get_attenuation(2),
        Self::get_attenuation(3),
        Self::get_attenuation(4),
        Self::get_attenuation(5),
      ],
    }
  }

  pub fn process(&mut self, size: f32, taps: &mut [Tap; 4]) -> (f32, f32) {
    let gain = size.scale(MIN_SIZE, MAX_SIZE, MINUS_THREE_DB, MINUS_FIFTEEN_DB);

    let reflections = (
      taps[0].early_reflection_read(size, self.reflections[0][0]) * self.attenuations[0]
        + taps[0].early_reflection_read(size, self.reflections[0][1]) * self.attenuations[1]
        + taps[0].early_reflection_read(size, self.reflections[0][2]) * self.attenuations[2]
        + taps[0].early_reflection_read(size, self.reflections[0][3]) * self.attenuations[3]
        + taps[0].early_reflection_read(size, self.reflections[0][4]) * self.attenuations[4]
        + taps[0].early_reflection_read(size, self.reflections[0][5]) * self.attenuations[5],
      taps[1].early_reflection_read(size, self.reflections[1][0]) * self.attenuations[0]
        + taps[1].early_reflection_read(size, self.reflections[1][1]) * self.attenuations[1]
        + taps[1].early_reflection_read(size, self.reflections[1][2]) * self.attenuations[2]
        + taps[1].early_reflection_read(size, self.reflections[1][3]) * self.attenuations[3]
        + taps[1].early_reflection_read(size, self.reflections[1][4]) * self.attenuations[4]
        + taps[1].early_reflection_read(size, self.reflections[1][5]) * self.attenuations[5],
    );
    (reflections.0 * gain, reflections.1 * gain)
  }

  fn get_attenuation(index: usize) -> f32 {
    1. - (index as f32 / 5. * (1. - LAST_EARLY_REFLECTION_GAIN))
  }
}
