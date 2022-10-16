use std::{thread::sleep, time::Duration};

use rodio::{OutputStream, Source};

use crate::{input::Input, source::SynthSourcer};

pub struct Player;

impl Player {
    pub fn play(input: impl Into<Input>) {
        let (_stream, handle) = OutputStream::try_default().unwrap();

        handle
            .play_raw(SynthSourcer::new(input, 44100).convert_samples())
            .unwrap();

        sleep(Duration::MAX);
    }
}
