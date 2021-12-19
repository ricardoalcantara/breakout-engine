use rodio::{OutputStream, OutputStreamHandle, Source};

use crate::audio::Audio;

pub struct AudioPlayer {
    _stream: OutputStream,
    handle: OutputStreamHandle,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        Self {
            _stream: stream,
            handle,
        }
    }

    pub fn play(&mut self, audio: &Audio, repeat_infinite: bool) {
        if repeat_infinite {
            let source = audio.decoder().repeat_infinite();
            self.handle.play_raw(source.convert_samples()).unwrap();
        } else {
            let source = audio.decoder();
            self.handle.play_raw(source.convert_samples()).unwrap();
        }
    }
}
