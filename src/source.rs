use rodio::Source;

use crate::input::Input;

pub struct SynthSourcer {
    rate: u32,
    index: u32,
    input: Input,
}

impl SynthSourcer {
    pub fn new(input: impl Into<Input>, rate: u32) -> Self {
        Self {
            rate,
            index: 0,
            input: input.into(),
        }
    }
}

impl Source for SynthSourcer {
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

impl Iterator for SynthSourcer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        self.input.get_sample(self.rate, self.index)
    }
}
