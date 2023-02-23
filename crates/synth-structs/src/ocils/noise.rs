use std::ops;

use crate::{
    ops::{Add, Amp},
    Context, Input, Synth,
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
        Default::default()
    }

    pub fn with_freq(freq: impl Into<Input>) -> Self {
        Self(Type::Freq {
            freq: freq.into(),
            index: 0.0,
            last_sample: None,
        })
    }

    fn get_random() -> f32 {
        rand::random::<f32>() * 2.0 - 1.0
    }
}

impl Default for Noise {
    fn default() -> Self {
        Self(Type::Simple)
    }
}

impl Synth for Noise {
    fn sample(&mut self, context: Context) -> Option<f32> {
        match &mut self.0 {
            Type::Simple => Some(Self::get_random()),
            Type::Freq {
                freq,
                index,
                last_sample,
            } => {
                let len = 1.0 / context.rate as f32;
                *index += len * freq.sample(context)?;
                if *index >= 1.0 {
                    *index %= 1.0;
                    *last_sample = Some(Self::get_random());
                }

                if last_sample.is_none() {
                    *last_sample = Some(Self::get_random());
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
