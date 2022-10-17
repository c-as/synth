use synth::{ocils::Noise, player::Player};

fn main() {
    Player::play(Noise::new() * 0.05);
}
