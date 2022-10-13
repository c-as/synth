use synth::{modifiers::amp::Amp, player::Player, synths::oscillator::Oscillator};

fn main() {
    Player::play(Amp::new(
        0.05,
        Oscillator::new(Amp::new(100.0, Oscillator::new(1.0))),
    ));
}
