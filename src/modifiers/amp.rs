use crate::{input::Input, Synth};

pub struct Amp<T> {
    input: T,
    vol: Input,
}

impl<T> Amp<T> {
    pub fn new(vol: impl Into<Input>, input: T) -> Self {
        Self {
            input,
            vol: vol.into(),
        }
    }
}

impl<T: Synth> Synth for Amp<T> {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        Some(self.vol.get_input(rate, index)? * self.input.get_sample(rate, index)?)
    }
}
