use synth::{ocils::Sine, player::Player};

fn main() {
    Player::play(Sine::new(Sine::new(1.0) * 200 + 300) * 0.2);
}
