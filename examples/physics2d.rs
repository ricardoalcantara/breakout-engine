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
        Collision, KinematicBody2D, PhysicsBody2D, PhysicsBody2DId, PhysicsBody2DType, Shapes,
        StaticBody2D,
    },
    shapes::rectangle::Rect,
};
use hecs::With;
use log::warn;
use winit::event::VirtualKeyCode;

extern crate log;
extern crate pretty_env_logger;

struct Player {
    speed: f32,
    velocity: math::Vec2,
}

struct MainState {
    gravity: math::Vec2,
}

fn get_input_direction(_input: &mut Input) -> math::Vec2 {
    let mut direction = math::Vec2::ZERO;

    if _input.is_key_pressed(VirtualKeyCode::Up) || _input.is_key_pressed(VirtualKeyCode::W) {
        direction.y = -1.0;
    }
    if _input.is_key_pressed(VirtualKeyCode::Down) || _input.is_key_pressed(VirtualKeyCode::S) {
        direction.y = 1.0;
    }
    if _input.is_key_pressed(VirtualKeyCode::Left) || _input.is_key_pressed(VirtualKeyCode::A) {
        direction.x = -1.0;
    }
    if _input.is_key_pressed(VirtualKeyCode::Right) || _input.is_key_pressed(VirtualKeyCode::D) {
        direction.x = 1.0;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize()
    }

    direction
}

impl MainState {
    fn new() -> Self {
        Self {
            gravity: math::vec2(0.0, -900.0),
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
        let mut world = _context.get_world_mut();
        let mut physics_world = _context.get_physics_world_mut();

        world.spawn((
            Sprite {
                color: Some(math::vec4(1.0, 1.0, 1.0, 1.0)),
                ..Default::default()
            },
            Transform2D::from_position_rotation_scale(
                math::vec2(200.0, 10.0),
                0.0,
                math::vec2(16.0, 16.0),
            ),
            physics_world.spawn(PhysicsBody2D {
                physics_body_type: PhysicsBody2DType::kinematic_body_2d(),
                collision: Collision::from_rect(Rect::from_position_size(
                    math::vec2(0.0, 0.0).into(),
                    math::vec2(16.0, 16.0).into(),
                )),
                position: math::vec2(200.0, 10.0),
            }),
            Player {
                speed: 150.0,
                velocity: math::Vec2::ZERO,
            },
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
            physics_world.spawn(PhysicsBody2D {
                physics_body_type: PhysicsBody2DType::static_body_2d(),
                collision: Collision::from_rect(Rect::from_position_size(
                    math::vec2(0.0, 0.0).into(),
                    math::vec2(250.0, 16.0).into(),
                )),
                position: math::vec2(10.0, 450.0),
            }),
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
            physics_world.spawn(PhysicsBody2D {
                physics_body_type: PhysicsBody2DType::static_body_2d(),
                collision: Collision::from_rect(Rect::from_position_size(
                    math::vec2(0.0, 0.0).into(),
                    math::vec2(250.0, 16.0).into(),
                )),
                position: math::vec2(350.0, 450.0),
            }),
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
            physics_world.spawn(PhysicsBody2D {
                physics_body_type: PhysicsBody2DType::static_body_2d(),
                collision: Collision::from_rect(Rect::from_position_size(
                    math::vec2(0.0, 0.0).into(),
                    math::vec2(200.0, 16.0).into(),
                )),
                position: math::vec2(200.0, 375.0),
            }),
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
        let mut physics_world = _context.get_physics_world_mut();

        let direction = get_input_direction(_input);

        for (_id, (physics_body_2d_id, player)) in
            &mut world.query::<(&mut PhysicsBody2DId, &mut Player)>()
        {
            if let Some(mut body) = physics_world.get_mut(physics_body_2d_id) {
                if let PhysicsBody2DType::KinematicBody2D(kinematic_body_2d) =
                    &mut body.physics_body_type
                {
                    if _input.is_key_pressed(VirtualKeyCode::Space) {
                        player.velocity += math::vec2(0.0, 900.0);
                    }

                    player.velocity += direction * player.speed;

                    if player.velocity.length() < 0.01 {
                        player.velocity = math::Vec2::ZERO;
                    }

                    let dt_velocity = player.velocity * _dt;

                    player.velocity -= dt_velocity;
                    kinematic_body_2d.move_by = Some(dt_velocity + (self.gravity * _dt));
                }
            } else {
                warn!("Missing body somehow!");
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
