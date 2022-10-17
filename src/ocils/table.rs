use std::ops::{Add, Mul};

use interpolation::Lerp;

use crate::{
    input::Input,
    ocils::{Saw, Sine, Square},
    ops::{Amp, Mix},
    Synth,
};

pub struct Table {
    table: Vec<f32>,
    freq: Input,
}

impl Table {
    pub fn new(table: Vec<f32>, freq: impl Into<Input>) -> Self {
        Self {
            table,
            freq: freq.into(),
        }
    }

    pub fn new_sine(size: u32, freq: impl Into<Input>) -> Self {
        Self::from_synth(size, Sine::new(1), freq)
    }

    pub fn new_square(size: u32, freq: impl Into<Input>) -> Self {
        Self::from_synth(size, Square::new(1), freq)
    }

    pub fn new_saw(size: u32, freq: impl Into<Input>) -> Self {
        Self::from_synth(size, Saw::new(1), freq)
    }

    pub fn from_synth(size: u32, input: impl Into<Input>, freq: impl Into<Input>) -> Self {
        let mut synth: Input = input.into();
        Self {
            table: (0..size)
                .map(|i| synth.get_sample(size, i).unwrap_or_default())
                .collect(),
            freq: freq.into(),
        }
    }
}

impl Synth for Table {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        let table_index = (index as f32 / rate as f32
            * self.table.len() as f32
            * self.freq.get_sample(rate, index)?)
            % self.table.len() as f32;

        let first = self.table.get(table_index.floor() as usize).unwrap();
        let second = self
            .table
            .get(if table_index.round() as usize == self.table.len() {
                0
            } else {
                table_index.round() as usize
            })
            .unwrap();
        Some(first.lerp(second, &(table_index % 1.0)))
    }
}

impl<T: Into<Input>> Mul<T> for Table {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Table {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
