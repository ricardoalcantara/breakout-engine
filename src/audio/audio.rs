use std::{
    io::{Cursor, Result},
    sync::Arc,
};

pub struct AudioSettings {
    pub repeat_infinite: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            repeat_infinite: false,
        }
    }
}

pub struct Audio {
    data: Arc<[u8]>,
    pub settings: Option<AudioSettings>,
}

impl Audio {
    fn new(data: Arc<[u8]>) -> Audio {
        Audio {
            data,
            settings: None,
        }
    }
}

impl AsRef<[u8]> for Audio {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Audio {
    pub fn load(file_name: &str) -> Result<Audio> {
        let buf = std::fs::read(file_name)?;
        Ok(Audio::new(buf.into()))
    }

    pub(crate) fn cursor(self: &Self) -> Cursor<Audio> {
        Cursor::new(Audio::new(Arc::clone(&self.data)))
    }
    pub(crate) fn decoder(self: &Self) -> rodio::Decoder<Cursor<Audio>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}
