use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Sprite, Transform2D},
        engine::{EngineBuilder, WindowSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        input::{Event, Input},
        scene::{InputHandled, Scene, Transition},
    },
    error::BreakoutResult,
    math,
    physics2d::components::physics::{
        Collision, KinematicBody2D, PhysicsBody2D, PhysicsBody2DType, StaticBody2D,
    },
};
use hecs::With;

extern crate log;
extern crate pretty_env_logger;

struct Player;

struct MainState {
    gravity: math::Vec2,
}

impl MainState {
    fn new() -> Self {
        Self {
            gravity: math::vec2(0.0, 900.0),
        }
    }
}

impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
        _engine: &mut EngineContext,
    ) -> BreakoutResult {
        let world = _context.get_world();

        world.spawn((
            Sprite {
                color: Some(math::vec4(1.0, 1.0, 1.0, 1.0)),
                ..Default::default()
            },
            Transform2D::from_position_rotation_scale(
                math::vec2(10.0, 10.0),
                0.0,
                math::vec2(16.0, 16.0),
            ),
            PhysicsBody2D {
                physics_body_type: PhysicsBody2DType::KinematicBody2D(KinematicBody2D {
                    move_by: None,
                }),
                collisions: vec![Collision::Rect],
            },
            Player,
        ));

        world.spawn((
            Sprite {
                color: Some(math::vec4(1.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            },
            Transform2D::from_position_rotation_scale(
                math::vec2(10.0, 450.0),
                0.0,
                math::vec2(250.0, 16.0),
            ),
            PhysicsBody2D {
                physics_body_type: PhysicsBody2DType::StaticBody2D(StaticBody2D {}),
                collisions: vec![Collision::Rect],
            },
        ));

        world.spawn((
            Sprite {
                color: Some(math::vec4(1.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            },
            Transform2D::from_position_rotation_scale(
                math::vec2(350.0, 450.0),
                0.0,
                math::vec2(250.0, 16.0),
            ),
        ));

        world.spawn((
            Sprite {
                color: Some(math::vec4(1.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            },
            Transform2D::from_position_rotation_scale(
                math::vec2(200.0, 375.0),
                0.0,
                math::vec2(200.0, 16.0),
            ),
        ));

        Ok(())
    }

    fn input(
        &mut self,
        _event: Event,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<InputHandled> {
        Ok(InputHandled::None)
    }

    fn update(
        &mut self,
        _dt: f32,
        _input: &mut Input,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<Transition> {
        let world = _context.get_world();

        for (_id, physics_body_2d) in &mut world.query::<With<Player, &mut PhysicsBody2D>>() {
            match &mut physics_body_2d.physics_body_type {
                PhysicsBody2DType::KinematicBody2D(kinematic_body_2d) => {
                    kinematic_body_2d.move_by = Some((math::vec2(0.0, 0.0) + self.gravity) * _dt)
                }
                _ => panic!(),
            }
        }

        Ok(Transition::None)
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_window_settings(WindowSettings::Title(String::from("Physics2d")))
        .with_window_settings(WindowSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
