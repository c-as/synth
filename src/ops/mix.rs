use std::ops::{Add, Mul};

use crate::{ops::Amp, Input, Synth};

pub struct Mix {
    a: Input,
    b: Input,
}

impl Mix {
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
        let mut i = 0;
        for input in inputs {
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

            i += 1;
        }

        match res {
            Res::None => Mix::new(0.0, 0.0),
            Res::One(a) => Mix::new(a, 0.0),
            Res::Two(a, b) => Mix::new(a, b),
            Res::Many(a, b) => Mix::new(a, Mix::from_vec(b)),
        }
    }
}

impl Synth for Mix {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        Some(self.a.get_sample(rate)? + self.b.get_sample(rate)?)
    }
}

impl<T: Into<Input>> Mul<T> for Mix {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Mix {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
