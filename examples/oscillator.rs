use synth::{ocils::oscillator::Oscillator, player::Player};

fn main() {
    Player::play(Oscillator::new(200) * 0.5)
}
