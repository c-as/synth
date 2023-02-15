use std::ops;

use crate::{
    ops::{Add, Amp},
    Input, Synth,
};

pub struct QuadDamp {
    input: Input,
    max_accel: Input,
    last_ampl: f32,
    last_diff: f32,
}

impl QuadDamp {
    pub fn new(max_accel: impl Into<Input>, input: impl Into<Input>) -> Self {
        Self {
            input: input.into(),
            max_accel: max_accel.into(),
            last_ampl: 0.0,
            last_diff: 0.0,
        }
    }
}

impl Synth for QuadDamp {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        let mut sample = self.input.get_sample(rate)?;
        let max_accel = self.max_accel.get_sample(rate)?;

        let mut diff = sample - self.last_ampl;
        let accel = diff - self.last_diff;

        if accel.abs() > max_accel {
            diff = self.last_diff + max_accel * accel.signum();
            sample = self.last_ampl + diff;
        }

        self.last_ampl = sample;
        self.last_diff = diff;

        Some(sample)
    }
}

impl<T: Into<Input>> ops::Mul<T> for QuadDamp {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for QuadDamp {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
