use std::{
    f32::consts::PI,
    ops::{Add, Mul},
};

use interpolation::Lerp;

use crate::{
    input::Input,
    modifiers::{amp::Amp, mix::Mix},
    Synth,
};

pub struct WaveTable {
    table: Vec<f32>,
    freq: Input,
}

impl WaveTable {
    pub fn new(table: Vec<f32>, freq: impl Into<Input>) -> Self {
        Self {
            table,
            freq: freq.into(),
        }
    }

    pub fn new_sine(size: u32, freq: impl Into<Input>) -> Self {
        Self {
            table: (0..size)
                .map(|i| (i as f32 / size as f32 * 2.0 * PI).sin())
                .collect(),
            freq: freq.into(),
        }
    }
}

impl Synth for WaveTable {
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

impl<T: Into<Input>> Mul<T> for WaveTable {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for WaveTable {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
