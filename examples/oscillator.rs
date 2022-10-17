use synth::{ocils::sine::Sine, player::Player};

fn main() {
    Player::play(Sine::new(200) * 0.5)
}
