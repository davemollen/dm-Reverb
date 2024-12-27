#[path = "./editor/components/param_checkbox.rs"]
mod param_checkbox;
use nih_plug_vizia::vizia::style::FontWeightKeyword;
use param_checkbox::ParamCheckbox;
#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
mod ui_data;
use crate::reverb_parameters::ReverbParameters;
use nih_plug::params::Param;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  views::{HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (480, 296))
}

pub(crate) fn create(
  params: Arc<ReverbParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      let _ = cx.add_stylesheet(STYLE);

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.size.name(),
            UiData::params,
            params.size.as_ptr(),
            |params| &params.size,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Large,
          )
          .top(Stretch(1.0))
          .bottom(Stretch(1.0));

          VStack::new(cx, |cx| {
            ParamKnob::new(
              cx,
              params.predelay.name(),
              UiData::params,
              params.predelay.as_ptr(),
              |params| &params.predelay,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            );
            ParamCheckbox::new(
              cx,
              params.reverse.name(),
              UiData::params,
              params.reverse.as_ptr(),
              |params| &params.reverse,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            )
            .top(Pixels(4.))
            .left(Stretch(1.0))
            .right(Stretch(1.0))
            .height(Pixels(92.));
          })
          .child_space(Stretch(1.0))
          .left(Pixels(16.0));

          VStack::new(cx, |cx| {
            ParamKnob::new(
              cx,
              params.speed.name(),
              UiData::params,
              params.speed.as_ptr(),
              |params| &params.speed,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            );
            ParamKnob::new(
              cx,
              params.depth.name(),
              UiData::params,
              params.depth.as_ptr(),
              |params| &params.depth,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            )
            .top(Pixels(4.));
          })
          .child_space(Stretch(1.0));

          VStack::new(cx, |cx| {
            ParamKnob::new(
              cx,
              params.absorb.name(),
              UiData::params,
              params.absorb.as_ptr(),
              |params| &params.absorb,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            );
            ParamKnob::new(
              cx,
              params.decay.name(),
              UiData::params,
              params.decay.as_ptr(),
              |params| &params.decay,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            )
            .top(Pixels(4.));
          })
          .child_space(Stretch(1.0));

          VStack::new(cx, |cx| {
            ParamKnob::new(
              cx,
              params.tilt.name(),
              UiData::params,
              params.tilt.as_ptr(),
              |params| &params.tilt,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            );
            ParamKnob::new(
              cx,
              params.shimmer.name(),
              UiData::params,
              params.shimmer.as_ptr(),
              |params| &params.shimmer,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            )
            .top(Pixels(4.));
          })
          .child_space(Stretch(1.0));

          ParamKnob::new(
            cx,
            params.mix.name(),
            UiData::params,
            params.mix.as_ptr(),
            |params| &params.mix,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );
        });

        Label::new(cx, "dm-Reverb")
          .font_size(22.0)
          .font_weight(FontWeightKeyword::Bold)
          .border_radius(Pixels(16.0))
          .border_width(Pixels(1.))
          .border_color("#2c5494")
          .background_color("#3c6ab5")
          .child_space(Stretch(1.0))
          .child_top(Pixels(1.0))
          .child_bottom(Pixels(5.0))
          .width(Pixels(144.0))
          .left(Stretch(1.0));
      })
      .child_space(Pixels(16.0))
      .background_color("#505050");
    },
  )
}
