use rand::{thread_rng, Rng};
use synth::{ocils::table::WaveTable, player::Player};

fn main() {
    let mut rng = thread_rng();

    Player::play(
        (WaveTable::new_sine(1000, rng.gen_range(1.0..250.0))
            + WaveTable::new_sine(1000, rng.gen_range(1.0..250.0)))
            * 0.25,
    );
}
