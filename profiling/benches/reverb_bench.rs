use criterion::{criterion_group, criterion_main, Criterion};
use reverb::{Reverb, SmoothParameters};

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_stereo_signal_stream(length: usize) -> Vec<(f32, f32)> {
  (0..length)
    .map(|_| (generate_signal(), generate_signal()))
    .collect()
}

fn reverb_bench(c: &mut Criterion) {
  let mut reverb = Reverb::new(44100.);
  let mut smooth_parameters = SmoothParameters::new(44100.);
  smooth_parameters.set_targets(0., 7., 80., -0.2, 0.8, 0.8, 0.1, 0.5, 0.5);
  let signal_stream = generate_stereo_signal_stream(44100);

  c.bench_function("reverb", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        reverb.process(*signal, 3., &mut smooth_parameters);
      }
    })
  });
}

criterion_group!(benches, reverb_bench);
criterion_main!(benches);
