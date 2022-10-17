use std::{
    f32::consts::PI,
    ops::{Add, Mul},
};

use crate::{
    input::Input,
    modifiers::{amp::Amp, mix::Mix},
};

use crate::Synth;

pub struct Oscillator(Input);

impl Oscillator {
    pub fn new(freq: impl Into<Input>) -> Self {
        Self(freq.into())
    }
}

impl Synth for Oscillator {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        Some((index as f32 / rate as f32 * 2.0 * PI * self.0.get_sample(rate, index)?).sin())
    }
}

impl<T: Into<Input>> Mul<T> for Oscillator {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Oscillator {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
