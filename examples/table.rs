use synth::{ocils::table::WaveTable, player::Player};

fn main() {
    Player::play(WaveTable::new_sine(1000, 200) * 0.5)
}
