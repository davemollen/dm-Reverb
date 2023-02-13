use std::f32::consts::PI;

pub struct Mix;

impl Mix {
  pub fn run(dry: (f32, f32), wet: (f32, f32), mix: f32) -> (f32, f32) {
    let twopi = PI * 2.;
    let phase = mix * 0.25;
    let dry_gain = (phase * twopi).cos();
    let wet_gain = ((phase + 0.75) * twopi).cos();
    let dry_left = dry.0 * dry_gain;
    let dry_right = dry.1 * dry_gain;
    let wet_left = wet.0 * wet_gain;
    let wet_right = wet.1 * wet_gain;
    (dry_left + wet_left, dry_right + wet_right)
  }
}

#[cfg(test)]
mod tests {
  use crate::mix::Mix;

  #[test]
  fn mix() {
    let first = Mix::run((0., 0.), (1., 1.), 0.);
    let second = Mix::run((0., 0.), (1., 1.), 0.5);
    let third = Mix::run((0., 0.), (1., 1.), 1.);
    assert_eq!((first.0 * 1000.).floor() / 1000., 0.);
    assert_eq!((first.1 * 1000.).floor() / 1000., 0.);
    assert_eq!((second.0 * 1000.).floor() / 1000., 0.707);
    assert_eq!((second.1 * 1000.).floor() / 1000., 0.707);
    assert_eq!((third.0 * 1000.).floor() / 1000., 1.);
    assert_eq!((third.1 * 1000.).floor() / 1000., 1.);
  }
}
