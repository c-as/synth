use crate::Synth;

pub struct Closure<F: Fn(u32, u32) -> Option<f32>>(F);

impl<F: Fn(u32, u32) -> Option<f32>> Closure<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F: Fn(u32, u32) -> Option<f32>> Synth for Closure<F> {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        self.0(rate, index)
    }
}
