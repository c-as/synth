use rand::{thread_rng, Rng};
use synth::{
    modifiers::{amp::Amp, mix::Mix},
    player::Player,
    synths::table::WaveTable,
};

fn main() {
    let mut rng = thread_rng();

    Player::play(Amp::new(
        0.05,
        Mix((0..10)
            .map(|_| WaveTable::new_sine(1000, rng.gen_range(1.0..250.0)).into())
            .collect()),
    ));
}
