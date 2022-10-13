use std::f32::consts::PI;

use crate::input::Input;

use crate::Synth;

pub struct Oscillator(Input);

impl Oscillator {
    pub fn new(freq: impl Into<Input>) -> Self {
        Self(freq.into())
    }
}

impl Synth for Oscillator {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        Some((index as f32 / rate as f32 * 2.0 * PI * self.0.get_input(rate, index)?).sin())
    }
}
