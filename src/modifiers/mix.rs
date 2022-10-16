use crate::{input::Input, Synth};

pub struct Mix(pub Vec<Input>);

impl Synth for Mix {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        let mut sample = 0.0;

        for synth in self.0.iter_mut() {
            sample += synth.get_input(rate, index)?;
        }

        Some(sample)
    }
}
