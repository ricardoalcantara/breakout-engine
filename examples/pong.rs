extern crate log;
extern crate pretty_env_logger;

use core::{
    components::{Sprite, Transform2D},
    AssetManager, EngineBuilder, GameContext, Input, Scene, VirtualKeyCode,
};
use hecs::With;
use log::error;
use rand::Rng;
use shapes::rectangle::Rectangle;

struct MainState {}

impl MainState {
    fn new() -> Self {
        Self {}
    }
}

struct Ball {
    direction: glam::Vec2,
    speed: f32,
}

struct Player;

struct AI {
    direction: glam::Vec2,
    cooldown: f32,
    response_time: f32,
}

struct Paddles {
    speed: f32,
}

impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
    ) -> Result<(), ()> {
        let pong_texture = _asset_manager.load_sprite("assets/pong-ball.png");
        let paddles_texture = _asset_manager.load_sprite("assets/paddles.png");

        let world = &mut _context.get_world();
        let speed = 0.02;
        world.spawn((
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
                speed,
            },
        ));

        world.spawn((
            Player,
            Paddles { speed },
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
            AI {
                direction: glam::Vec2::ZERO,
                cooldown: 0.0,
                response_time: 60.0,
            },
            Paddles { speed },
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
        let mut rng = rand::thread_rng();
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

        {
            let (ball_position, ball_size) = if let Some((_id, transform)) =
                world.query::<With<Ball, &Transform2D>>().iter().next()
            {
                (transform.position, transform.scale)
            } else {
                error!("Where's the ball?");
                return Ok(core::Transition::None);
            };

            for (_id, (ai, transform, paddles)) in
                world.query_mut::<(&mut AI, &mut Transform2D, &Paddles)>()
            {
                ai.cooldown += 1.0;

                if ai.cooldown > ai.response_time {
                    ai.cooldown = 0.0;
                    ai.response_time = rng.gen_range(60.0..1000.0);
                    if transform.position.y > ball_position.y + ball_size.y / 2.0 {
                        ai.direction.y = -1.0;
                    } else if transform.position.y + transform.scale.y
                        < ball_position.y + ball_size.y / 2.0
                    {
                        ai.direction.y = 1.0;
                    }
                }

                transform.position += ai.direction * paddles.speed;
            }
        }

        let mut paddles_collider = Vec::new();
        for (_id, transform) in world.query_mut::<With<Paddles, &Transform2D>>() {
            paddles_collider.push(Rectangle::from_position_size(
                transform.position.into(),
                transform.scale.into(),
            ))
        }

        'ball: for (_id, (ball, transform)) in world.query_mut::<(&mut Ball, &mut Transform2D)>() {
            transform.position += ball.direction * ball.speed;

            if transform.position.y < 0.0 || transform.position.y > (600.0 - 32.0) {
                ball.direction.y *= -1.0;
            }

            let ball_collider =
                Rectangle::from_position_size(transform.position.into(), transform.scale.into());

            for paddles in &paddles_collider {
                if paddles.intersects(&ball_collider) {
                    ball.direction.x *= -1.0;

                    if transform.position.x > paddles.x {
                        transform.position.x = paddles.x + transform.scale.x;
                    } else {
                        transform.position.x = paddles.x - transform.scale.x;
                    }

                    if transform.position.y + transform.scale.y / 2.0 > paddles.center().y {
                        ball.direction.y = 1.0;
                    } else {
                        ball.direction.y = -1.0;
                    }
                    break 'ball;
                }
            }

            if transform.position.x < 0.0 || transform.position.x > (800.0 - 32.0) {
                transform.position.x = 416.0;
                transform.position.y = 316.0;
                let direction =
                    glam::vec2(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)).normalize();
                ball.direction = direction;
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
