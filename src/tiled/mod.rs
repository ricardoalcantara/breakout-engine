use std::collections::HashMap;

use self::json_map::JsonMap;
use crate::{
    core::{
        asset_manager::TextureId,
        components::{Sprite, SubTexture, Transform2D},
        game_context::GameContext,
    },
    error::{BreakoutError, BreakoutResult},
    math,
    shapes::rectangle::Rect,
};

mod json_map;

pub struct Tile {
    size: math::Vec2,
}

pub struct Tiled {
    map: JsonMap,
    textures: HashMap<String, TextureId>,
}

impl Tiled {
    pub fn load_map(path: &str, textures: &[(&str, TextureId)]) -> BreakoutResult<Tiled> {
        let json_string = std::fs::read_to_string(path)
            .expect(&format!("Something went wrong reading {:}", path));

        // Convert the JSON string back to a Point.
        let map: JsonMap = serde_json::from_str(&json_string)
            .map_err(|_| BreakoutError::GenericError("serde_json::from_str failed"))?;

        let mut t = HashMap::new();
        for texture in textures {
            t.insert(texture.0.to_string(), texture.1.clone());
        }

        Ok(Tiled { map, textures: t })
    }

    pub fn spawn(&self, context: &mut GameContext) -> BreakoutResult {
        let world = &mut context.get_world();
        let texture = self.textures.values().next().unwrap().clone();
        let mut x = 0.0;
        let mut y = 0.0;
        for layer in &self.map.layers {
            for data_id in &layer.data {
                let tileset = self
                    .map
                    .tilesets
                    .iter()
                    .find(|ts| *data_id >= ts.firstgid && *data_id < ts.firstgid + ts.tilecount)
                    .unwrap();

                let tile_id = data_id - tileset.firstgid;
                let tile_x = tile_id % tileset.columns;
                let tile_y = tile_id / tileset.columns;
                let tile_size = math::vec2(tileset.tilewidth as f32, tileset.tileheight as f32);

                world.spawn((
                    Tile { size: tile_size },
                    Sprite {
                        texture_id: Some(texture.clone()),
                        sub_texture: Some(SubTexture::new(Rect::new(
                            ((tile_x * tile_size.x as u32)
                                + (tile_x * tileset.spacing)
                                + tileset.margin) as f32,
                            ((tile_y * tile_size.y as u32)
                                + (tile_y * tileset.spacing)
                                + tileset.margin) as f32,
                            tile_size.x,
                            tile_size.y,
                        ))),
                        ..Default::default()
                    },
                    Transform2D::from_position(math::vec2(x * 16.0, y * 16.0)),
                ));

                x += 1.0;
                if x >= self.map.height as f32 {
                    x = 0.0;
                    y += 1.0;
                }
            }
        }
        Ok(())
    }

    pub fn update(&self, context: &mut GameContext) {
        let camera_rect = context.get_camera_rect();
        let world = &mut context.get_world();

        if let Some(camera) = camera_rect {
            for (_id, (sprite, tile, transform)) in
                &mut world.query::<(&mut Sprite, &Tile, &Transform2D)>()
            {
                sprite.visible = camera.intersects(&Rect::from_position_size(
                    transform.position().into(),
                    tile.size.into(),
                ));
            }
        }
    }
}
