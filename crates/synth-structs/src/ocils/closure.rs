use std::ops;

use crate::{
    ops::{Add, Amp},
    Context, Input, Synth,
};

#[derive(Clone)]
pub struct Closure<F: FnMut(Context) -> Option<f32>>(F);

impl<F: FnMut(Context) -> Option<f32>> Closure<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F: FnMut(Context) -> Option<f32> + Clone> Synth for Closure<F> {
    fn sample(&mut self, context: Context) -> Option<f32> {
        self.0(context)
    }
}

impl<F: FnMut(Context) -> Option<f32> + Send + Clone + 'static, T: Into<Input>> ops::Mul<T>
    for Closure<F>
{
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<F: FnMut(Context) -> Option<f32> + Send + Clone + 'static, T: Into<Input>> ops::Add<T>
    for Closure<F>
{
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
