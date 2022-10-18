use synth::{ocils::Table, Player};

fn main() {
    Player::play(Table::new_sine(1000, 200) * 0.5)
}
