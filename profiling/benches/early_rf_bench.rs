use criterion::{criterion_group, criterion_main, Criterion};
use reverb::{shared::delay_line::DelayLine, EarlyReflections};

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_signal_stream(length: usize) -> Vec<f32> {
  (0..length).map(|_| generate_signal()).collect()
}

fn early_rf_bench(c: &mut Criterion) {
  let sample_rate = 44100.;
  let early_rf = EarlyReflections::new();
  let mut delay_lines = [
    DelayLine::new((sample_rate * 1.) as usize, sample_rate),
    DelayLine::new((sample_rate * 1.) as usize, sample_rate),
    DelayLine::new((sample_rate * 1.) as usize, sample_rate),
    DelayLine::new((sample_rate * 1.) as usize, sample_rate),
  ];
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("early_rf", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        delay_lines.iter_mut().for_each(|delay_line| {
          delay_line.write(*signal);
        });
        early_rf.process(40., &delay_lines);
      }
    })
  });
}

criterion_group!(benches, early_rf_bench);
criterion_main!(benches);
