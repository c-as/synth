use std::{thread::sleep, time::Duration};

use rodio::{OutputStream, OutputStreamHandle, Source};

use crate::{source::SynthSourcer, Input};

pub struct Player {
    _stream: OutputStream,
    _handle: OutputStreamHandle,
}

impl Player {
    pub fn play(input: impl Into<Input>) {
        let _player = Self::play_threaded(input);

        sleep(Duration::MAX);
    }

    pub fn play_threaded(input: impl Into<Input>) -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();

        handle
            .play_raw(SynthSourcer::new(input, 44100).convert_samples())
            .unwrap();

        Self {
            _stream: stream,
            _handle: handle,
        }
    }
}
