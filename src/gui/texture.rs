use crate::core::asset_manager::TextureId;

pub(crate) struct Texture {
    texture_id: TextureId,
}

impl Texture {
    pub(crate) fn new(texture_id: TextureId) -> Texture {
        Texture { texture_id }
    }
}
