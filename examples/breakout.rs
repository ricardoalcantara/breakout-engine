extern crate log;
extern crate pretty_env_logger;

use core::{
    components::{Sprite, Transform2D},
    engine_context::EngineContext,
    AssetManager, EngineBuilder, EngineSettings, GameContext, Input, Scene, VirtualKeyCode,
};

use physics2d::systems::physics::system_update_physics;

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
        _engine: &mut EngineContext,
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

    fn input(
        &mut self,
        _event: core::Event,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> Result<core::InputHandled, ()> {
        Ok(core::InputHandled::None)
    }

    fn update(
        &mut self,
        _dt: f32,
        _input: &mut Input,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> Result<core::Transition, ()> {
        if _input.is_key_pressed(VirtualKeyCode::Space) {}
        if _input.is_key_released(VirtualKeyCode::Space) {}

        let world = &mut _context.get_world();

        system_update_physics(world);

        for (_id, transform) in world.query_mut::<&mut Transform2D>() {
            // transform.position += ball.direction * 0.01;
        }

        Ok(core::Transition::None)
    }
}

fn main() {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("Breakout")))
        .with_settings(EngineSettings::WindowSize((800, 600)))
        .build()
        .unwrap()
        .run(MainState::new());
}
