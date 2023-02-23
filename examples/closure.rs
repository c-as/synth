use std::f32::consts::PI;

use synth::{ocils::Closure, Player};
fn main() {
    const FREQ: f32 = 200.0;
    const VOL: f32 = 0.5;

    let mut index = 0.0;
    Player::play(Closure::new(move |context| {
        let len = 1.0 / context.rate as f32;
        index += len * FREQ;
        index %= 1.0;
        let angle = index * 2.0 * PI;
        let ampl = angle.sin() * VOL;

        Some(ampl)
    }));
}
