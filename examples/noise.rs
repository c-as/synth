use synth::{ocils::Noise, Player};

fn main() {
    Player::play(Noise::new_freq(1000) * 0.05);
}
