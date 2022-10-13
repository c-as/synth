use rodio::Source;

use crate::Synth;

pub struct SynthSourcer<T> {
    rate: u32,
    index: u32,
    synth: T,
}

impl<T> SynthSourcer<T> {
    pub fn new(synth: T, rate: u32) -> Self {
        Self {
            rate,
            index: 0,
            synth: synth,
        }
    }
}

impl<T: Synth> Source for SynthSourcer<T> {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl<T: Synth> Iterator for SynthSourcer<T> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        self.synth.get_sample(self.rate, self.index)
    }
}
