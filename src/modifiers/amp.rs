use std::ops::{Add, Mul};

use crate::{input::Input, Synth};

use super::mix::Mix;

pub struct Amp {
    a: Input,
    b: Input,
}

impl Amp {
    pub fn new(a: impl Into<Input>, b: impl Into<Input>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }
}

impl Synth for Amp {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        Some(self.a.get_sample(rate, index)? * self.b.get_sample(rate, index)?)
    }
}

impl<T: Into<Input>> Mul<T> for Amp {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Amp {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
