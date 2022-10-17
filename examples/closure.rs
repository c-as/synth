use std::f32::consts::PI;

use synth::{ocils::closure::Closure, player::Player};
fn main() {
    Player::play(
        Closure::new(|rate, index| Some((index as f32 / rate as f32 * 2.0 * PI * 200.0).sin()))
            * 0.5,
    );
}
