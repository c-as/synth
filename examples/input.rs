use synth::{ocils::Sine, player::Player};

fn main() {
    Player::play(Sine::new(Sine::new(1.0) * 100) * 0.05);
}
