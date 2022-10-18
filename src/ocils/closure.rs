use std::ops::{Add, Mul};

use crate::{
    input::Input,
    ops::{Amp, Mix},
    Synth,
};

pub struct Closure<F>(F);

impl<F> Closure<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F: FnMut(u32) -> Option<f32>> Synth for Closure<F> {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        self.0(rate)
    }
}

impl<F: FnMut(u32) -> Option<f32> + Send + 'static, T: Into<Input>> Mul<T> for Closure<F> {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<F: FnMut(u32) -> Option<f32> + Send + 'static, T: Into<Input>> Add<T> for Closure<F> {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
