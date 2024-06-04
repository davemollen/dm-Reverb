use nih_plug::prelude::*;
use reverb::Reverb;
mod reverb_parameters;
use reverb_parameters::ReverbParameters;
use std::sync::Arc;
mod editor;

struct DmReverb {
  params: Arc<ReverbParameters>,
  reverb: Reverb,
}

impl Default for DmReverb {
  fn default() -> Self {
    let params = Arc::new(ReverbParameters::default());
    Self {
      params: params.clone(),
      reverb: Reverb::new(44100.),
    }
  }
}

impl Plugin for DmReverb {
  const NAME: &'static str = "dm-Reverb";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-Reverb";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(2),
    main_output_channels: NonZeroU32::new(2),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.reverb = Reverb::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let reverse = self.params.reverse.value();
    let predelay = self.params.predelay.value();
    let size = self.params.size.value();
    let speed = self.params.speed.value();
    let depth = self.params.depth.value();
    let absorb = self.params.absorb.value();
    let decay = self.params.decay.value();
    let tilt = (self.params.tilt.value() * 0.5 + 0.5).max(0.0001);
    let shimmer = self.params.shimmer.value();
    let mix = self.params.mix.value();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let channel_iterator = &mut channel_samples.iter_mut();
      let left_channel = channel_iterator.next().unwrap();
      let right_channel = channel_iterator.next().unwrap();

      let (reverb_left, reverb_right) = self.reverb.process(
        (*left_channel, *right_channel),
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

      *left_channel = reverb_left;
      *right_channel = reverb_right;
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmReverb {
  const CLAP_ID: &'static str = "dm-Reverb";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A reverb plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Stereo,
    ClapFeature::Reverb,
  ];
}

impl Vst3Plugin for DmReverb {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-Reverb.......";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Reverb,
    Vst3SubCategory::Stereo,
  ];
}

nih_export_clap!(DmReverb);
nih_export_vst3!(DmReverb);
