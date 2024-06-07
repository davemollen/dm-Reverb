use criterion::{criterion_group, criterion_main, Criterion};
use reverb::Taps;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_stereo_signal_stream(length: usize) -> Vec<(f32, f32)> {
  (0..length)
    .map(|_| (generate_signal(), generate_signal()))
    .collect()
}

fn taps_bench(c: &mut Criterion) {
  let mut taps = Taps::new(44100.);
  let signal_stream = generate_stereo_signal_stream(44100);
  c.bench_function("taps", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        taps.process(*signal, 80., 2., -0.1, 0.5, 0.5, 0.8, 0.5);
      }
    })
  });
}

criterion_group!(benches, taps_bench);
criterion_main!(benches);
