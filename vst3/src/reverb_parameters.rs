use nih_plug::{
  formatters,
  prelude::{FloatParam, FloatRange, Params},
};
use reverb::{MAX_SIZE, MIN_SIZE};
mod custom_formatters;
use custom_formatters::v2s_f32_digits;

#[derive(Params)]
pub struct ReverbParameters {
  #[id = "predelay"]
  pub predelay: FloatParam,

  #[id = "size"]
  pub size: FloatParam,

  #[id = "speed"]
  pub speed: FloatParam,

  #[id = "depth"]
  pub depth: FloatParam,

  #[id = "absorb"]
  pub absorb: FloatParam,

  #[id = "decay"]
  pub decay: FloatParam,

  #[id = "tilt"]
  pub tilt: FloatParam,

  #[id = "mix"]
  pub mix: FloatParam,
}

impl Default for ReverbParameters {
  fn default() -> Self {
    Self {
      predelay: FloatParam::new(
        "Predelay",
        7.,
        FloatRange::Skewed {
          min: 7.,
          max: 500.,
          factor: 0.5,
        },
      )
      .with_unit("ms")
      .with_value_to_string(v2s_f32_digits(2)),

      size: FloatParam::new(
        "Size",
        40.,
        FloatRange::Skewed {
          min: MIN_SIZE,
          max: MAX_SIZE,
          factor: 0.333333,
        },
      )
      .with_unit("m2")
      .with_value_to_string(v2s_f32_digits(2)),

      speed: FloatParam::new(
        "Speed",
        2.,
        FloatRange::Skewed {
          min: 0.02,
          max: 50.,
          factor: 0.333333,
        },
      )
      .with_unit("Hz")
      .with_value_to_string(v2s_f32_digits(2)),

      depth: FloatParam::new("Depth", -0.25, FloatRange::Linear { min: -1., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),

      absorb: FloatParam::new("Absorb", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),

      decay: FloatParam::new("Decay", 0.9, FloatRange::Linear { min: 0., max: 1.2 })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),

      tilt: FloatParam::new("Tilt", 0., FloatRange::Linear { min: -1., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),

      mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),
    }
  }
}
