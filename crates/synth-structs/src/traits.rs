use dyn_clone::DynClone;

pub trait Synth: DynClone {
    fn sample(&mut self, rate: u32) -> Option<f32>;
}

dyn_clone::clone_trait_object!(Synth);

impl Synth for f32 {
    fn sample(&mut self, _: u32) -> Option<f32> {
        Some(*self)
    }
}

impl Synth for i32 {
    fn sample(&mut self, _: u32) -> Option<f32> {
        Some(*self as f32)
    }
}