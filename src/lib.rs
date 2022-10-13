pub mod input;
pub mod modifiers;
pub mod player;
mod source;
pub mod synths;

pub trait Synth {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32>;
}
