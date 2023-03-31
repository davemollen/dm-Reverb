/*
    TODO's:
    - implement non linear wave table with envelope follower
    - implement pitchshift on taps_output
    - implement reverse on predelay
*/

include!(concat!(env!("OUT_DIR"), "/wave_table.rs"));
mod allpass_filter;
mod biquad_filter;
mod dc_block;
mod delay_line;
mod delta;
mod float_ext;
mod grains;
mod lfo;
mod mix;
mod one_pole_filter;
mod pan;
mod phasor;
mod reverb;
mod tap;
mod taps;
mod tilt_filter;
mod wave_table;

pub const MIN_SIZE: f32 = 1.;
pub const MAX_SIZE: f32 = 500.;
pub const MAX_DEPTH: f32 = 4.;
pub use self::reverb::Reverb;
