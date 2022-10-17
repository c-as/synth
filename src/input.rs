use std::ops::{Add, Mul};

use crate::{
    ops::{Amp, Mix},
    Synth,
};

pub struct Input(pub Box<dyn Synth + Send>);

impl Input {
    pub fn new(input: impl Into<Input>) -> Self {
        input.into()
    }

    pub fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        self.0.get_sample(rate, index)
    }
}

impl<U: Synth + Send + 'static> From<U> for Input {
    fn from(value: U) -> Self {
        Self(Box::new(value))
    }
}

impl<T: Into<Input>> Mul<T> for Input {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Input {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
