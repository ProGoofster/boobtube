use std::f32::consts::TAU;

use crate::tube::TubeInputs;

/// Drive the yoke with quadrature sinusoids so the beam traces a circle.
/// `t` is time in seconds, `freq_hz` is revolutions per second, and
/// `amplitude_v` is the peak yoke voltage (sets the circle radius).
pub fn draw_circle(inputs: &mut TubeInputs, t: f32, freq_hz: f32, amplitude_v: f32) {
    let phase = TAU * freq_hz * t;
    inputs.v_yoke_h = amplitude_v * phase.cos();
    inputs.v_yoke_v = amplitude_v * phase.sin();
}
