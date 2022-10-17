use std::ops::{Add, Mul};

use crate::{
    input::Input,
    ops::{Amp, Mix},
    Synth,
};

pub struct Noise;

impl Noise {
    pub fn new() -> Self {
        Self
    }
}

impl Synth for Noise {
    fn get_sample(&mut self, _: u32, _: u32) -> Option<f32> {
        Some(rand::random())
    }
}

impl<T: Into<Input>> Mul<T> for Noise {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Noise {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
