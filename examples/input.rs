use synth::{ocils::oscillator::Oscillator, player::Player};

fn main() {
    Player::play(Oscillator::new(Oscillator::new(1.0) * 100) * 0.05);
}
