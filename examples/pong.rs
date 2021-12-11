extern crate log;
extern crate pretty_env_logger;

use core::{
    components::{Sprite, Transform2D},
    AssetManager, EngineBuilder, GameContext, Input, KeyCode, Scene,
};

use physics2d::{components::physics::Actor, systems::physics::system_update_physics};

struct MainState {}

impl MainState {
    fn new() -> Self {
        Self {}
    }
}

struct Ball {
    direction: glam::Vec2,
}

impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
    ) -> Result<(), ()> {
        let pong_texture = _asset_manager.load_sprite("assets/pong-ball.png");

        let world = &mut _context.get_world();
        world.spawn((
            Actor {},
            Sprite {
                texture_id: pong_texture,
            },
            Transform2D {
                position: glam::vec2(100.0, 100.0),
                scale: glam::vec2(32.0, 32.0),
                rotate: 0.0,
            },
            Ball {
                direction: glam::vec2(0.0, 1.0),
            },
        ));

        Ok(())
    }

    fn input(
        &mut self,
        _event: core::Event,
        _context: &mut GameContext,
    ) -> Result<core::InputHandled, ()> {
        Ok(core::InputHandled::None)
    }

    fn update(
        &mut self,
        _input: &mut Input,
        _context: &mut GameContext,
        _dt: f32,
    ) -> Result<core::Transition, ()> {
        if _input.is_key_pressed(KeyCode::Space) {}
        if _input.is_key_released(KeyCode::Space) {}

        let world = &mut _context.get_world();

        system_update_physics(world);

        for (_id, (ball, transform)) in world.query_mut::<(&mut Ball, &mut Transform2D)>() {
            transform.position += ball.direction * 0.01;
        }

        Ok(core::Transition::None)
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
