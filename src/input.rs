use super::Synth;

pub struct Input(pub Box<dyn Synth + Send>);

impl Input {
    pub fn new(input: impl Into<Input>) -> Self {
        input.into()
    }

    pub fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        self.0.get_sample(rate, index)
    }
}

impl<U: Synth + Send + 'static> From<U> for Input {
    fn from(value: U) -> Self {
        Self(Box::new(value))
    }
}
