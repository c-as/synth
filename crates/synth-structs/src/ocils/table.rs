use std::ops;

use interpolation::Lerp;

use crate::{
    ocils::{Saw, Sine, Square},
    ops::{Add, Amp},
    Input, Synth,
};

#[derive(Clone)]
pub struct Table {
    table: Vec<f32>,
    freq: Input,
    index: f32,
}

impl Table {
    pub fn new(table: Vec<f32>, freq: impl Into<Input>) -> Self {
        Self {
            table,
            freq: freq.into(),
            index: 0.0,
        }
    }

    pub fn with_sine(size: u32, freq: impl Into<Input>) -> Self {
        Self::from_synth(size, Sine::new(1), freq)
    }

    pub fn with_square(size: u32, freq: impl Into<Input>) -> Self {
        Self::from_synth(size, Square::new(1), freq)
    }

    pub fn with_saw(size: u32, freq: impl Into<Input>) -> Self {
        Self::from_synth(size, Saw::new(1), freq)
    }

    pub fn from_synth(size: u32, input: impl Into<Input>, freq: impl Into<Input>) -> Self {
        let mut synth: Input = input.into();

        Self::new(
            (0..size)
                .map(|_| synth.get_sample(size).unwrap_or_default())
                .collect(),
            freq,
        )
    }
}

impl Synth for Table {
    fn sample(&mut self, rate: u32) -> Option<f32> {
        let first = self.table.get(self.index.floor() as usize).unwrap();
        let second = self
            .table
            .get(if self.index.round() as usize == self.table.len() {
                0
            } else {
                self.index.round() as usize
            })
            .unwrap();

        let len = 1.0 / rate as f32;
        self.index += len * self.table.len() as f32 * self.freq.get_sample(rate)?;
        self.index %= self.table.len() as f32;

        Some(first.lerp(second, &(self.index % 1.0)))
    }
}

impl<T: Into<Input>> ops::Mul<T> for Table {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for Table {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
