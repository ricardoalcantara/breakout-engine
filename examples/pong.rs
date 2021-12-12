extern crate log;
extern crate pretty_env_logger;

use core::{
    components::{Sprite, Transform2D},
    AssetManager, EngineBuilder, GameContext, Input, Scene, VirtualKeyCode,
};

use hecs::With;
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

struct Player;

impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
    ) -> Result<(), ()> {
        let pong_texture = _asset_manager.load_sprite("assets/pong-ball.png");
        let paddles_texture = _asset_manager.load_sprite("assets/paddles.png");

        let world = &mut _context.get_world();
        world.spawn((
            Actor {},
            Sprite {
                texture_id: pong_texture,
            },
            Transform2D {
                position: glam::vec2(400.0, 300.0),
                scale: glam::vec2(32.0, 32.0),
                rotate: 0.0,
            },
            Ball {
                direction: glam::vec2(1.0, 1.0).normalize(),
            },
        ));

        world.spawn((
            Player,
            Actor {},
            Sprite {
                texture_id: paddles_texture.clone(),
            },
            Transform2D {
                position: glam::vec2(0.0, 100.0),
                scale: glam::vec2(32.0, 128.0),
                rotate: 0.0,
            },
        ));

        world.spawn((
            Actor {},
            Sprite {
                texture_id: paddles_texture,
            },
            Transform2D {
                position: glam::vec2(800.0 - 32.0, 100.0),
                scale: glam::vec2(32.0, 128.0),
                rotate: 0.0,
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
        input: &mut Input,
        _context: &mut GameContext,
        _dt: f32,
    ) -> Result<core::Transition, ()> {
        let world = &mut _context.get_world();

        for (_id, transform) in world.query_mut::<With<Player, &mut Transform2D>>() {
            let mut direction = glam::Vec2::ZERO;

            if input.is_key_pressed(VirtualKeyCode::Up) {
                direction.y = -1.0;
            }

            if input.is_key_pressed(VirtualKeyCode::Down) {
                direction.y = 1.0;
            }

            transform.position += direction * 0.02;
        }

        for (_id, (ball, transform)) in world.query_mut::<(&mut Ball, &mut Transform2D)>() {
            transform.position += ball.direction * 0.02;

            if transform.position.x < 0.0 || transform.position.x > (800.0 - 32.0) {
                ball.direction.x *= -1.0;
            }
            if transform.position.y < 0.0 || transform.position.y > (600.0 - 32.0) {
                ball.direction.y *= -1.0;
            }
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
