extern crate lv2;
extern crate reverb;
use lv2::prelude::*;
use reverb::{Bypass, Params, Reverb};

#[derive(PortCollection)]
struct Ports {
  size: InputPort<InPlaceControl>,
  predelay: InputPort<InPlaceControl>,
  reverse: InputPort<InPlaceControl>,
  speed: InputPort<InPlaceControl>,
  depth: InputPort<InPlaceControl>,
  absorb: InputPort<InPlaceControl>,
  decay: InputPort<InPlaceControl>,
  tilt: InputPort<InPlaceControl>,
  shimmer: InputPort<InPlaceControl>,
  mix: InputPort<InPlaceControl>,
  input_left: InputPort<InPlaceAudio>,
  input_right: InputPort<InPlaceAudio>,
  output_left: OutputPort<InPlaceAudio>,
  output_right: OutputPort<InPlaceAudio>,
  enabled: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-Reverb")]
struct DmReverb {
  reverb: Reverb,
  params: Params,
  bypass: Bypass,
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
      bypass: Bypass::new(sample_rate),
    })
  }

  /// Reset the internal state of the plugin.
  fn activate(&mut self, _features: &mut Self::InitFeatures) {
    self.reverb.reset();
    self.params.reset();
    self.bypass.reset();
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), sample_count: u32) {
    self.bypass.set_enabled(ports.enabled.get() == 0., || {
      self.reverb.reset();
      self.params.reset();
    });

    if self.bypass.is_done_activating() {
      ports.output_left.copy_from(&ports.input_left, sample_count);
      ports
        .output_right
        .copy_from(&ports.input_right, sample_count);
      return;
    }

    self.params.set(
      ports.reverse.get(),
      ports.predelay.get(),
      ports.size.get(),
      ports.speed.get(),
      ports.depth.get() * 0.01,
      ports.absorb.get() * 0.01,
      ports.decay.get() * 0.01,
      ports.tilt.get() * 0.01,
      ports.shimmer.get() * 0.01,
      ports.mix.get() * 0.01,
    );

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports.output_left.iter().zip(ports.output_right.iter());
    input_channels.zip(output_channels).for_each(
      |((input_left, input_right), (output_left, output_right))| {
        let dry = (input_left.get(), input_right.get());
        let wet = self.reverb.process(dry, &mut self.params);
        let output = self.bypass.process(dry, wet);
        output_left.set(output.0);
        output_right.set(output.1);
      },
    );
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmReverb);
