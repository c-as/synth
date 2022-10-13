use crate::Synth;

pub struct Mix<T>(pub Vec<T>);

impl<T: Synth> Synth for Mix<T> {
    fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        let mut sample = 0.0;

        for synth in self.0.iter_mut() {
            sample += synth.get_sample(rate, index)?;
        }

        Some(sample)
    }
}
