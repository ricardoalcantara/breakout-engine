use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Property {
    pub(crate) name: String,
    pub(crate) value: String,
    #[serde(rename = "type")]
    pub(crate) property_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Chunk {
    pub(crate) data: Vec<u32>,
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Layer {
    pub(crate) data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Tileset {
    pub(crate) image: String,
    pub(crate) firstgid: u32,
    pub(crate) tilewidth: i32,
    pub(crate) tileheight: i32,
    pub(crate) tilecount: u32,
    pub(crate) columns: u32,
    pub(crate) spacing: u32,
    pub(crate) margin: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonMap {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) tilewidth: i32,
    pub(crate) tileheight: i32,
    pub(crate) layers: Vec<Layer>,
    pub(crate) tilesets: Vec<Tileset>,
    pub(crate) properties: Option<Vec<Property>>,
}
