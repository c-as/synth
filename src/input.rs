use super::Synth;

pub enum Input {
    Static(f32),
    Dynamic(Box<dyn Synth + Send>),
}

impl Input {
    pub fn new(input: impl Into<Input>) -> Self {
        input.into()
    }

    pub fn get_sample(&mut self, rate: u32, index: u32) -> Option<f32> {
        match self {
            Input::Static(value) => Some(value.to_owned()),
            Input::Dynamic(synth) => synth.get_sample(rate, index),
        }
    }
}

impl From<f32> for Input {
    fn from(value: f32) -> Self {
        Input::Static(value)
    }
}

impl From<i32> for Input {
    fn from(value: i32) -> Self {
        Input::Static(value as f32)
    }
}

impl<U: Synth + Send + 'static> From<U> for Input {
    fn from(value: U) -> Self {
        Self::Dynamic(Box::new(value))
    }
}
