use std::{
    f32::consts::PI,
    ops::{Add, Mul},
};

use crate::{
    input::Input,
    ops::{Amp, Mix},
    Synth,
};

pub struct Sine(Input);

impl Sine {
    pub fn new(freq: impl Into<Input>) -> Self {
        Self(freq.into())
    }
}

impl Synth for Sine {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        let secs = index as f32 / rate as f32;
        let wavs = secs * self.0.get_sample(rate, index)?;
        let angle = (wavs % 1.0) * 2.0 * PI;
        let ampl = angle.sin();
        Some(ampl)
    }
}

impl<T: Into<Input>> Mul<T> for Sine {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Sine {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
