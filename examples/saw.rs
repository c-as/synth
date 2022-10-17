use synth::{ocils::Saw, player::Player};

fn main() {
    Player::play(Saw::new(500) * 0.05);
}
