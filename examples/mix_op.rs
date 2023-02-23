use rand::{thread_rng, Rng};
use synth::{ocils::Table, Player};

fn main() {
    let mut rng = thread_rng();

    Player::play(
        (Table::with_sine(1000, rng.gen_range(1.0..250.0))
            + Table::with_sine(1000, rng.gen_range(1.0..250.0)))
            * 0.25,
    );
}
