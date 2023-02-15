use std::ops;

use crate::{
    ops::{Add, Amp},
    Input, Synth,
};

pub struct LinDamp {
    input: Input,
    max_diff: Input,
    last: f32,
}

impl LinDamp {
    pub fn new(max_diff: impl Into<Input>, input: impl Into<Input>) -> Self {
        Self {
            input: input.into(),
            max_diff: max_diff.into(),
            last: 0.0,
        }
    }
}

impl Synth for LinDamp {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        let mut sample = self.input.get_sample(rate)?;
        let max_diff = self.max_diff.get_sample(rate)?;

        let diff = sample - self.last;

        if diff.abs() > max_diff {
            sample = self.last + max_diff * diff.signum()
        }

        self.last = sample;

        Some(sample)
    }
}

impl<T: Into<Input>> ops::Mul<T> for LinDamp {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for LinDamp {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
