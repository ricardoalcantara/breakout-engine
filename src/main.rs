extern crate log;
extern crate pretty_env_logger;

use core::{
    components::{Sprite, Transform2D},
    AssetManager, EngineBuilder, GameContext, Scene,
};

struct MainState {}

impl MainState {
    fn new() -> Self {
        Self {}
    }
}
impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
    ) -> Result<(), ()> {
        let texture_id_1 = _asset_manager.load_sprite("assets/awesomeface.png");
        let texture_id_2 = _asset_manager.load_sprite("assets/happy-tree.png");

        let world = &mut _context.get_world();
        world.spawn((
            Sprite {
                texture_id: texture_id_1.clone(),
            },
            Transform2D {
                position: glam::vec2(0.0, 0.0),
                scale: glam::vec2(300.0, 400.0),
                rotate: 0.0,
            },
        ));
        world.spawn((
            Sprite {
                texture_id: texture_id_2,
            },
            Transform2D {
                position: glam::vec2(600.0, 100.0),
                scale: glam::vec2(300.0, 400.0),
                rotate: 45.0,
            },
        ));
        world.spawn((
            Sprite {
                texture_id: texture_id_1,
            },
            Transform2D {
                position: glam::vec2(250.0, 400.0),
                scale: glam::vec2(150.0, 200.0),
                rotate: 0.0,
            },
        ));

        Ok(())
    }
}

fn main() {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_title(String::from("Hello Engine"))
        .with_size(800, 600)
        .build()
        .unwrap()
        .run(MainState::new());
}
