use std::ops;

use crate::{ops::Amp, Input, Synth};

#[derive(Clone)]
pub struct Add {
    a: Input,
    b: Input,
}

impl Add {
    pub fn new(a: impl Into<Input>, b: impl Into<Input>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }

    pub fn from_vec(inputs: Vec<Input>) -> Self {
        enum Res {
            None,
            One(Input),
            Two(Input, Input),
            Many(Input, Vec<Input>),
        }

        let mut res = Res::None;
        for (i, input) in inputs.into_iter().enumerate() {
            match i {
                0 => res = Res::One(input),
                1 => {
                    res = Res::Two(
                        match res {
                            Res::One(a) => a,
                            _ => unreachable!(),
                        },
                        input,
                    )
                }
                2 => {
                    let mut vec = Vec::<Input>::new();
                    res = Res::Many(
                        match res {
                            Res::Two(a, b) => {
                                vec.push(b);
                                vec.push(input);
                                a
                            }
                            _ => unreachable!(),
                        },
                        vec,
                    )
                }
                _ => match res {
                    Res::Many(a, b) => {
                        let mut b = b;
                        b.push(input);
                        res = Res::Many(a, b);
                    }
                    _ => unreachable!(),
                },
            }
        }

        match res {
            Res::None => Add::new(0.0, 0.0),
            Res::One(a) => Add::new(a, 0.0),
            Res::Two(a, b) => Add::new(a, b),
            Res::Many(a, b) => Add::new(a, Add::from_vec(b)),
        }
    }
}

impl Synth for Add {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        Some(self.a.get_sample(rate)? + self.b.get_sample(rate)?)
    }
}

impl<T: Into<Input>> ops::Mul<T> for Add {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for Add {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
