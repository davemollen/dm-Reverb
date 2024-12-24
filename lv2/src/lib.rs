extern crate lv2;
extern crate reverb;
use lv2::prelude::*;
use reverb::{Reverb, Params};

#[derive(PortCollection)]
struct Ports {
  size: InputPort<Control>,
  predelay: InputPort<Control>,
  reverse: InputPort<Control>,
  speed: InputPort<Control>,
  depth: InputPort<Control>,
  absorb: InputPort<Control>,
  decay: InputPort<Control>,
  tilt: InputPort<Control>,
  shimmer: InputPort<Control>,
  mix: InputPort<Control>,
  input_left: InputPort<Audio>,
  input_right: InputPort<Audio>,
  output_left: OutputPort<Audio>,
  output_right: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Reverb")]
struct DmReverb {
  reverb: Reverb,
  params: Params
}

impl Plugin for DmReverb {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = _plugin_info.sample_rate() as f32;

    Some(Self {
      reverb: Reverb::new(sample_rate),
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(
      *ports.reverse,
      *ports.predelay,
      *ports.size,
      *ports.speed,
      *ports.depth * 0.01,
      *ports.absorb * 0.01,
      *ports.decay * 0.01,
      *ports.tilt * 0.01,
      *ports.shimmer * 0.01,
      *ports.mix * 0.01,
    );

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());

    input_channels.zip(output_channels).for_each(
      |((input_left, input_right), (output_left, output_right))| {
        (*output_left, *output_right) = self.reverb.process(
          (*input_left, *input_right),
          &mut self.params
        );
      },
    );
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmReverb);
