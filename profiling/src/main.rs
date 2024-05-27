use reverb::Reverb;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut reverb = Reverb::new(44100.);

  loop {
    let input = (generate_signal(), generate_signal());
    reverb.process(input, false, 7., 80., 3., -0.2, 0.8, 0.8, 0.1, 0., 0.5);
  }
}
