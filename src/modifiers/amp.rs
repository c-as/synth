use crate::{input::Input, Synth};

pub struct Amp {
    a: Input,
    b: Input,
}

impl Amp {
    pub fn new(a: impl Into<Input>, b: impl Into<Input>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }
}

impl Synth for Amp {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        Some(self.a.get_input(rate, index)? * self.b.get_input(rate, index)?)
    }
}
