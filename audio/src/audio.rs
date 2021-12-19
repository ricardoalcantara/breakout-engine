use std::{
    io::{Cursor, Result},
    sync::Arc,
};

pub struct Audio(Arc<[u8]>);

impl AsRef<[u8]> for Audio {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Audio {
    pub fn load(file_name: &str) -> Result<Audio> {
        let buf = std::fs::read(file_name)?;
        Ok(Audio(buf.into()))
    }

    pub(crate) fn cursor(self: &Self) -> Cursor<Audio> {
        Cursor::new(Audio(Arc::clone(&self.0)))
    }
    pub(crate) fn decoder(self: &Self) -> rodio::Decoder<Cursor<Audio>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}
