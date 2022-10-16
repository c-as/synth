use synth::{player::Player, synths::oscillator::Oscillator};

fn main() {
    Player::play(Oscillator::new(Oscillator::new(1.0) * 100) * 0.05);
}
