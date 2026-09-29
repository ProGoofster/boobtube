mod driver;
mod tube;

use driver::draw_circle;
use tube::{TubeAge, TubeInputs, TubeOutputs};

fn main() {
    let mut inputs = TubeInputs::default();
    let outputs = TubeOutputs::default();

    const SAMPLE_RATE_HZ: f32 = 60.0;
    for n in 0..16 {
        let t = n as f32 / SAMPLE_RATE_HZ;
        // Quarter turn (90°) per sample.
        draw_circle(&mut inputs, t, SAMPLE_RATE_HZ / 4.0, 10.0);
        println!("t={t:.4} h={:+.3} v={:+.3}", inputs.v_yoke_h, inputs.v_yoke_v);
    }

    println!("{outputs:?}");
}
