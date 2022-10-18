use synth::{input::Input, ocils::Noise, player::Player};

fn main() {
    Player::play(Noise::new(Some(Input::from(1000))) * 0.05);
}
