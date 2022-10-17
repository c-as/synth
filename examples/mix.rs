use rand::{thread_rng, Rng};
use synth::{input::Input, ocils::table::Table, ops::mix::Mix, player::Player};

fn main() {
    let mut rng = thread_rng();

    Player::play(Mix::from_vec(
        (0..10)
            .map(|_| Input::from(Table::new_sine(1000, rng.gen_range(1.0..250.0)) * 0.05))
            .collect(),
    ));
}
