use synth::{
    ocils::{Noise, Sine},
    Player,
};

fn main() {
    let lfo = Noise::with_freq(30) * 1000;
    let synth = Sine::new(lfo) * 0.2;
    Player::play(synth);
}
