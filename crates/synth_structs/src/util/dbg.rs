use std::ops;

use crate::{ops::Add, Input, Synth};

#[derive(Clone)]
pub struct Dbg {
    input: Input,
    interval: Input,
    index: u32,
}

impl Dbg {
    pub fn new(input: impl Into<Input>) -> Self {
        Self {
            input: input.into(),
            interval: Input::from(1.0),
            index: 0,
        }
    }

    pub fn new_interval(interval: impl Into<Input>, input: impl Into<Input>) -> Self {
        Self {
            input: input.into(),
            interval: interval.into(),
            index: 0,
        }
    }
}

impl Synth for Dbg {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        let sample = self.input.get_sample(rate);

        if self.index % self.interval.get_sample(rate)?.round() as u32 == 0 {
            if let Some(sample) = sample {
                dbg!(sample);
            } else {
                dbg!(sample);
            }
        }

        self.index += 1;
        sample
    }
}

impl<T: Into<Input>> ops::Mul<T> for Dbg {
    type Output = Add;

    fn mul(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for Dbg {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
