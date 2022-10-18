use synth::{ocils::Square, Player};

fn main() {
    Player::play(Square::new(100) * 0.1);
}
