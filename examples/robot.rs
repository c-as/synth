use synth::{
    ocils::{Noise, Sine},
    Input, Player,
};

fn main() {
    let lfo = Noise::new(Some(Input::from(30))) * 1000;
    let synth = Sine::new(lfo) * 0.2;
    Player::play(synth);
}
