#[macro_use]
extern crate vst;
mod editor;
use editor::ReverbEditor;
mod reverb_parameters;
use reverb::Reverb;
use reverb_parameters::{Params, ReverbParameters};
use std::sync::Arc;
use vst::{
  buffer::AudioBuffer,
  editor::Editor,
  plugin::{Category, Info, Plugin, PluginParameters},
  prelude::HostCallback,
};

struct DmReverb {
  params: Arc<ReverbParameters>,
  reverb: Reverb,
  editor: Option<ReverbEditor>,
}

impl Plugin for DmReverb {
  fn new(host: HostCallback) -> Self {
    let params = Arc::new(ReverbParameters::default());

    Self {
      params: params.clone(),
      reverb: Reverb::new(44100.),
      editor: Some(ReverbEditor {
        params: params.clone(),
        is_open: false,
        host: Some(host),
      }),
    }
  }

  fn set_sample_rate(&mut self, sample_rate: f32) {
    self.reverb = Reverb::new(sample_rate);
  }

  fn get_info(&self) -> Info {
    Info {
      name: "dm-Reverb".to_string(),
      vendor: "DM".to_string(),
      version: 1,
      inputs: 2,
      outputs: 2,
      parameters: 10,
      unique_id: 1358,
      f64_precision: true,
      category: Category::Effect,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let reverse = self.params.reverse.get_value();
    let predelay = self.params.predelay.get_value();
    let size = self.params.size.get_value();
    let speed = self.params.speed.get_value();
    let depth = self.params.depth.get_value();
    let absorb = self.params.absorb.get_value();
    let decay = self.params.decay.get_value();
    let tilt = self.params.tilt.get_normalized_value().max(0.0001);
    let shimmer = self.params.shimmer.get_value();
    let mix = self.params.mix.get_value();

    let (input_channels, mut output_channels) = buffer.split();
    let zipped_input_channels = input_channels.get(0).iter().zip(input_channels.get(1));
    let zipped_output_channels = output_channels
      .get_mut(0)
      .iter_mut()
      .zip(output_channels.get_mut(1));

    zipped_input_channels.zip(zipped_output_channels).for_each(
      |((input_left, input_right), (output_left, output_right))| {
        let (reverb_left, reverb_right) = self.reverb.process(
          (*input_left, *input_right),
          reverse,
          predelay,
          size,
          speed,
          depth,
          absorb,
          decay,
          tilt,
          shimmer,
          mix,
        );
        *output_left = reverb_left;
        *output_right = reverb_right;
      },
    );
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }

  fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
    if let Some(editor) = self.editor.take() {
      Some(Box::new(editor) as Box<dyn Editor>)
    } else {
      None
    }
  }
}

plugin_main!(DmReverb);
