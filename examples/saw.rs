use synth::{ocils::Saw, Player};

fn main() {
    Player::play(Saw::new(500) * 0.05);
}
