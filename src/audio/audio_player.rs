use super::audio::{Audio, AudioSettings};
use rodio::{OutputStream, OutputStreamHandle, Source};

pub struct AudioPlayer {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    audio_settings: AudioSettings,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        Self {
            _stream: stream,
            handle,
            audio_settings: AudioSettings::default(),
        }
    }

    pub fn play(&mut self, audio: &Audio) {
        let settings = audio.settings.as_ref().unwrap_or(&self.audio_settings);

        if settings.repeat_infinite {
            let source = audio.decoder().repeat_infinite();
            self.handle.play_raw(source.convert_samples()).unwrap();
        } else {
            let source = audio.decoder();
            self.handle.play_raw(source.convert_samples()).unwrap();
        }
    }
}
