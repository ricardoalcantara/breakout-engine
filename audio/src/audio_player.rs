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

    pub fn play(&mut self, audio: &Audio) {
        let source = audio.decoder();
        self.handle.play_raw(source.convert_samples()).unwrap();
    }

    // Todo: Understand the play_once difference
    // https://github.com/RustAudio/rodio/blob/master/src/lib.rs#L1
    pub fn play_once(&mut self, audio: &Audio) {
        let cursor = audio.cursor();
        self.handle.play_once(cursor).unwrap();
    }
}
