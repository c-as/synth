pub mod input;
pub mod ocils;
pub mod ops;
pub mod player;
mod source;
pub mod util;

pub use input::Input;
pub use player::Player;

use dyn_clone::DynClone;

pub trait Synth: DynClone {
    fn get_sample(&mut self, rate: u32) -> Option<f32>;
}

dyn_clone::clone_trait_object!(Synth);

impl Synth for f32 {
    fn get_sample(&mut self, _: u32) -> Option<f32> {
        Some(*self)
    }
}

impl Synth for i32 {
    fn get_sample(&mut self, _: u32) -> Option<f32> {
        Some(*self as f32)
    }
}
