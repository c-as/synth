use synth::{player::Player, synths::table::WaveTable};

fn main() {
    Player::play(WaveTable::new_sine(1000, 200))
}
