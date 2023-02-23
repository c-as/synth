use std::ops;

use crate::{
    ops::{Add, Amp},
    Input, Synth,
};

#[derive(Clone)]
pub struct Closure<F>(F);

impl<F> Closure<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F: FnMut(u32) -> Option<f32> + Clone> Synth for Closure<F> {
    fn sample(&mut self, rate: u32) -> Option<f32> {
        self.0(rate)
    }
}

impl<F: FnMut(u32) -> Option<f32> + Send + Clone + 'static, T: Into<Input>> ops::Mul<T>
    for Closure<F>
{
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<F: FnMut(u32) -> Option<f32> + Send + Clone + 'static, T: Into<Input>> ops::Add<T>
    for Closure<F>
{
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
