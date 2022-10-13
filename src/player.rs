use std::{thread::sleep, time::Duration};

use rodio::{OutputStream, Source};

use crate::source::SynthSourcer;

use super::Synth;

pub struct Player;

impl Player {
    pub fn play<T: Synth + Send + 'static>(synth: T) {
        let (_stream, handle) = OutputStream::try_default().unwrap();

        handle
            .play_raw(SynthSourcer::new(synth, 44100).convert_samples())
            .unwrap();

        sleep(Duration::MAX);
    }
}
