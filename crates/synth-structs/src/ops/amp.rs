use std::ops;

use crate::{ops::Add, Context, Input, Synth};

#[derive(Clone)]
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
    fn sample(&mut self, context: Context) -> Option<f32> {
        Some(self.a.sample(context)? * self.b.sample(context)?)
    }
}

impl<T: Into<Input>> ops::Mul<T> for Amp {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for Amp {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
