use synth::{player::Player, synths::oscillator::Oscillator};

fn main() {
    Player::play(Oscillator::new(200) * 0.5)
}
