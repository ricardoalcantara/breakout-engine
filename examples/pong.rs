extern crate log;
extern crate pretty_env_logger;

use breakout_engine::math;
use breakout_engine::shapes::rectangle::Rect;
use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Sprite, Transform2D},
        engine::{EngineBuilder, EngineSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        input::{Event, Input, VirtualKeyCode},
        scene::{InputHandled, Scene, Transition},
    },
    error::BreakoutResult,
};
use hecs::With;
use log::error;
use rand::Rng;

const MAX_LEVEL_TIME: f32 = 5.0;

struct MainState {
    #[allow(dead_code)]
    total_time: f32,
    level: f32,
    level_time: f32,
}

impl MainState {
    fn new() -> Self {
        Self {
            total_time: 0.0,
            level: 1.0,
            level_time: MAX_LEVEL_TIME,
        }
    }
}

struct Ball {
    direction: math::Vec2,
    speed: f32,
}

struct Player;

struct AI {
    direction: math::Vec2,
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
        _engine: &mut EngineContext,
    ) -> BreakoutResult {
        let pong_texture = _asset_manager.load_texture("assets/pong-ball.png")?;
        let paddles_texture = _asset_manager.load_texture("assets/paddles.png")?;

        let world = &mut _context.get_world();
        let speed = 200.0;
        world.spawn((
            Sprite {
                texture_id: Some(pong_texture),
                color: Some(math::vec3(1.0, 0.0, 0.0)),
                ..Default::default()
            },
            Transform2D {
                position: math::vec2(400.0, 300.0),
                ..Default::default()
            },
            Ball {
                direction: math::vec2(1.0, 1.0).normalize(),
                speed,
            },
            Rect::new_with_size(32.0, 32.0),
        ));

        world.spawn((
            Player,
            Paddles { speed },
            Sprite {
                texture_id: Some(paddles_texture.clone()),
                ..Default::default()
            },
            Transform2D {
                position: math::vec2(0.0, 100.0),
                ..Default::default()
            },
            Rect::new_with_size(32.0, 128.0),
        ));

        world.spawn((
            AI {
                direction: math::Vec2::ZERO,
                cooldown: 0.0,
                response_time: 1.0,
            },
            Paddles { speed },
            Sprite {
                texture_id: Some(paddles_texture),
                ..Default::default()
            },
            Transform2D {
                position: math::vec2(800.0 - 32.0, 100.0),
                ..Default::default()
            },
            Rect::new_with_size(32.0, 128.0),
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
        input: &mut Input,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<Transition> {
        let mut rng = rand::thread_rng();
        let world = &mut _context.get_world();

        self.level_time -= _dt;

        if self.level_time <= 0.0 {
            self.level_time = MAX_LEVEL_TIME;
            self.level += 0.2;
        }

        if input.is_key_pressed(VirtualKeyCode::F) {
            _engine.update_settings(EngineSettings::Fullscreen(true));
        }

        if input.is_key_pressed(VirtualKeyCode::G) {
            _engine.update_settings(EngineSettings::Fullscreen(false));
        }

        for (_id, (transform, paddles)) in world.query_mut::<(&mut Transform2D, &Paddles)>() {
            let mut direction = math::Vec2::ZERO;

            if input.is_key_pressed(VirtualKeyCode::Up) {
                direction.y = -1.0;
            }

            if input.is_key_pressed(VirtualKeyCode::Down) {
                direction.y = 1.0;
            }

            transform.position += direction * paddles.speed * self.level * _dt;
        }

        {
            let (ball_position, ball_size) = if let Some((_id, (transform, collider))) = world
                .query::<With<Ball, (&Transform2D, &Rect)>>()
                .iter()
                .next()
            {
                (transform.position, collider.clone())
            } else {
                error!("Where's the ball?");
                return Ok(Transition::None);
            };

            for (_id, (ai, transform, paddles, collider)) in
                world.query_mut::<(&mut AI, &mut Transform2D, &Paddles, &Rect)>()
            {
                ai.cooldown += 1.0;

                if ai.cooldown > ai.response_time {
                    ai.cooldown = 0.0;
                    ai.response_time = rng.gen_range(1.0..5.0);
                    if transform.position.y > ball_position.y + ball_size.height / 2.0 {
                        ai.direction.y = -1.0;
                    } else if transform.position.y + collider.width
                        < ball_position.y + ball_size.height / 2.0
                    {
                        ai.direction.y = 1.0;
                    }
                }

                transform.position += ai.direction * paddles.speed * self.level * _dt;
            }
        }

        let mut paddles_collider = Vec::new();
        for (_id, (transform, collider)) in
            world.query_mut::<With<Paddles, (&Transform2D, &Rect)>>()
        {
            paddles_collider.push(collider.moved_to(transform.position.into()));
        }

        'ball: for (_id, (ball, transform, collider)) in
            world.query_mut::<(&mut Ball, &mut Transform2D, &Rect)>()
        {
            transform.position += ball.direction * ball.speed * self.level * _dt;

            if transform.position.y < 0.0 || transform.position.y > (600.0 - 32.0) {
                ball.direction.y *= -1.0;
            }

            let ball_collider = collider.moved_to(transform.position.into());

            for paddles in &paddles_collider {
                if paddles.intersects(&ball_collider) {
                    ball.direction.x *= -1.0;

                    if transform.position.x > paddles.x {
                        transform.position.x = paddles.x + collider.width;
                    } else {
                        transform.position.x = paddles.x - collider.width;
                    }

                    if transform.position.y + collider.height / 2.0 > paddles.center().y {
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
                    math::vec2(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)).normalize();
                ball.direction = direction;
            }
        }

        Ok(Transition::None)
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("Pong")))
        .with_settings(EngineSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
