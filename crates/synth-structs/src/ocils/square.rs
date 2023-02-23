use std::ops;

use crate::{
    ops::{Add, Amp},
    Context, Input, Synth,
};

#[derive(Clone)]
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
    fn sample(&mut self, context: Context) -> Option<f32> {
        let ampl = self.index.round() * 2.0 - 1.0;

        let len = 1.0 / context.rate as f32;
        self.index += len * self.freq.get_sample(context)?;
        self.index %= 1.0;

        Some(ampl)
    }
}

impl<T: Into<Input>> ops::Mul<T> for Square {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for Square {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
