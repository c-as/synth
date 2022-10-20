use std::ops;

use crate::{
    ops::{Add, Amp},
    Input, Synth,
};

#[derive(Clone)]
enum Type {
    Simple,
    Freq {
        freq: Input,
        index: f32,
        last_sample: Option<f32>,
    },
}

#[derive(Clone)]
pub struct Noise(Type);

impl Noise {
    pub fn new() -> Self {
        Self(Type::Simple)
    }

    pub fn new_freq(freq: impl Into<Input>) -> Self {
        Self(Type::Freq {
            freq: freq.into(),
            index: 0.0,
            last_sample: None,
        })
    }
}

impl Synth for Noise {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        match &mut self.0 {
            Type::Simple => Some(rand::random()),
            Type::Freq {
                freq,
                index,
                last_sample,
            } => {
                let len = 1.0 / rate as f32;
                *index += len * freq.get_sample(rate)?;
                if *index >= 1.0 {
                    *index %= 1.0;
                    *last_sample = Some(rand::random());
                }

                if last_sample.is_none() {
                    *last_sample = Some(rand::random());
                }

                *last_sample
            }
        }
    }
}

impl<T: Into<Input>> ops::Mul<T> for Noise {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for Noise {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
