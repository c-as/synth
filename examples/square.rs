use synth::{ocils::Square, player::Player};

fn main() {
    Player::play(Square::new(100) * 0.1);
}
