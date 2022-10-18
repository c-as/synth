use std::ops::{Add, Mul};

use crate::{
    ops::{Amp, Mix},
    Input, Synth,
};

pub struct Square {
    freq: Input,
    index: f32,
}

impl Square {
    pub fn new(freq: impl Into<Input>) -> Self {
        Self {
            freq: freq.into(),
            index: 0.0,
        }
    }
}

impl Synth for Square {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        let len = 1.0 / rate as f32;
        self.index += len * self.freq.get_sample(rate)?;
        self.index %= 1.0;
        let ampl = self.index.round() * 2.0 - 1.0;
        Some(ampl)
    }
}

impl<T: Into<Input>> Mul<T> for Square {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Square {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
