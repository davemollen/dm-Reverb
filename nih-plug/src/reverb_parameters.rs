use nih_plug::{
  formatters::{s2v_f32_percentage, v2s_f32_percentage},
  prelude::{BoolParam, FloatParam, FloatRange, Params},
};
use nih_plug_vizia::ViziaState;
use reverb::shared::constants::{MAX_PREDELAY, MAX_SIZE, MIN_PREDELAY, MIN_SIZE};
use std::sync::Arc;
mod custom_formatters;
use crate::editor;
use custom_formatters::v2s_f32_digits;

#[derive(Params)]
pub struct ReverbParameters {
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "size"]
  pub size: FloatParam,

  #[id = "predelay"]
  pub predelay: FloatParam,

  #[id = "reverse"]
  pub reverse: BoolParam,

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

  #[id = "shimmer"]
  pub shimmer: FloatParam,

  #[id = "mix"]
  pub mix: FloatParam,
}

impl Default for ReverbParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      size: FloatParam::new(
        "Size",
        80.,
        FloatRange::Skewed {
          min: MIN_SIZE,
          max: MAX_SIZE,
          factor: 0.333333,
        },
      )
      .with_value_to_string(v2s_f32_digits(2)),

      predelay: FloatParam::new(
        "Predelay",
        MIN_PREDELAY,
        FloatRange::Skewed {
          min: MIN_PREDELAY,
          max: MAX_PREDELAY,
          factor: 0.5,
        },
      )
      .with_unit(" ms")
      .with_value_to_string(v2s_f32_digits(2)),

      reverse: BoolParam::new("Reverse", false),

      speed: FloatParam::new(
        "Speed",
        2.,
        FloatRange::SymmetricalSkewed {
          min: 0.02,
          max: 150.,
          factor: 0.333333,
          center: 5.,
        },
      )
      .with_unit(" Hz")
      .with_value_to_string(v2s_f32_digits(2)),

      depth: FloatParam::new("Depth", -0.1, FloatRange::Linear { min: -1., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      absorb: FloatParam::new("Absorb", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      decay: FloatParam::new("Decay", 0.9, FloatRange::Linear { min: 0., max: 1.2 })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      tilt: FloatParam::new("Tilt", 0., FloatRange::Linear { min: -1., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      shimmer: FloatParam::new("Shimmer", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
