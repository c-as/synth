use std::ops::{Add, Mul};

use crate::{input::Input, Synth};

use super::amp::Amp;

pub struct Mix(pub Vec<Input>);

impl Synth for Mix {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        let mut sample = 0.0;

        for synth in self.0.iter_mut() {
            sample += synth.get_sample(rate, index)?;
        }

        Some(sample)
    }
}

impl<T: Into<Input>> Mul<T> for Mix {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Mix {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        let mut vec = self.0;
        vec.push(rhs.into());
        Mix(vec)
    }
}
