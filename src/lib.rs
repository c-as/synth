pub mod input;
pub mod ocils;
pub mod ops;
pub mod player;
mod source;

pub trait Synth {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32>;
}

impl Synth for f32 {
    fn get_sample(&mut self, _: u32, _: u32) -> Option<f32> {
        Some(*self)
    }
}

impl Synth for i32 {
    fn get_sample(&mut self, _: u32, _: u32) -> Option<f32> {
        Some(*self as f32)
    }
}
