use reverb::{Reverb, SmoothParameters};

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut reverb = Reverb::new(44100.);
  let mut smooth_parameters = SmoothParameters::new(44100.);
  smooth_parameters.set_targets(0., 7., 80., -0.2, 0.8, 0.8, 0.1, 0.5, 0.5);

  loop {
    let input = (generate_signal(), generate_signal());
    reverb.process(input, 3., &mut smooth_parameters);
  }
}
