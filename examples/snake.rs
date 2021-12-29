extern crate log;
extern crate pretty_env_logger;

use breakout_engine::audio::AudioSettings;
use breakout_engine::math;
use breakout_engine::{
    core::{
        asset_manager::{AssetManager, AudioId},
        components::{Sprite, Transform2D},
        engine::{EngineBuilder, EngineSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        input::{Event, Input, VirtualKeyCode},
        scene::{InputHandled, Scene, Transition},
    },
    error::BreakoutResult,
};
use hecs::{With, World};
use log::{error, info};
use rand::Rng;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TILE_SIZE: u32 = 16;
const GRID_WIDTH: u32 = (WIDTH / TILE_SIZE) - 1;
const GRID_HEIGHT: u32 = (HEIGHT / TILE_SIZE) - 1;

enum GameMode {
    ACube,
    Else,
}

enum SnakeState {
    Unknow,
    Moving(math::Vec2),
    New,
    Dead,
}

struct Snake;
struct Frute;
struct MainState {
    snake: Vec<math::Vec2>,
    frute: math::Vec2,
    current_direction: math::Vec2,
    input_direction: math::Vec2,
    time: f32,
    delay: f32,
    game_mode: GameMode,
    effect_audio_id: Option<AudioId>,
}

fn get_input_direction(_input: &mut Input) -> math::Vec2 {
    let mut direction = math::Vec2::ZERO;

    if _input.is_key_pressed(VirtualKeyCode::Up) {
        direction.y = -1.0;
    }
    if _input.is_key_pressed(VirtualKeyCode::Down) {
        direction.y = 1.0;
    }
    if _input.is_key_pressed(VirtualKeyCode::Left) {
        direction.x = -1.0;
    }
    if _input.is_key_pressed(VirtualKeyCode::Right) {
        direction.x = 1.0;
    }

    direction
}

impl MainState {
    fn new() -> Self {
        Self {
            snake: Vec::new(),
            current_direction: math::vec2(1.0, 0.0),
            input_direction: math::vec2(1.0, 0.0),
            frute: math::Vec2::ZERO,
            time: f32::MAX,
            delay: 0.1,
            game_mode: GameMode::ACube,
            effect_audio_id: None,
        }
    }

    fn spawn_snake_part(&self, world: &mut World, position: math::Vec2) {
        world.spawn((
            Sprite {
                ..Default::default()
            },
            Transform2D {
                position,
                scale: math::vec2(TILE_SIZE as f32, TILE_SIZE as f32),
                rotate: 0.0,
            },
            Snake,
        ));
    }

    fn refresh_frute(&mut self) {
        let mut rng = rand::thread_rng();

        self.frute.x = rng.gen_range(0..GRID_WIDTH - 1) as f32;
        self.frute.y = rng.gen_range(0..GRID_HEIGHT - 1) as f32;
    }

    fn start(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() > 0.5 {
            self.current_direction = math::vec2(1.0, 0.0);
        } else {
            self.current_direction = math::vec2(0.0, 1.0);
        }
        self.snake.push(math::Vec2::ZERO);

        self.refresh_frute();
    }

    fn restart(&mut self) {
        self.snake.clear();
        self.start();
    }
}

impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
        _engine: &mut EngineContext,
    ) -> BreakoutResult {
        let args: Vec<String> = std::env::args().collect();
        info!("{:?}", args);
        if args.len() > 2 && args[1] == "game_mode" && args[2] == "else" {
            self.game_mode = GameMode::Else;
        }

        self.start();

        let music_audio_id = _asset_manager.load_audio(
            "assets/slow-piano-intermission.ogg",
            Some(AudioSettings {
                repeat_infinite: true,
                ..Default::default()
            }),
        )?;
        self.effect_audio_id = Some(_asset_manager.load_audio("assets/coin.wav", None)?);

        _context.play_audio(music_audio_id);

        let world = &mut _context.get_world();

        world.spawn((
            Sprite {
                color: Some(math::vec4(1.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            },
            Transform2D {
                position: math::vec2(
                    (self.frute.x as u32 * TILE_SIZE) as f32,
                    (self.frute.y as u32 * TILE_SIZE) as f32,
                ),
                scale: math::vec2(TILE_SIZE as f32, TILE_SIZE as f32),
                rotate: 0.0,
            },
            Frute,
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
        let direction = get_input_direction(_input);

        if self.current_direction.x != 0.0 {
            if direction.y != 0.0 {
                self.input_direction.x = 0.0;
                self.input_direction.y = direction.y;
            }
        } else if self.current_direction.y != 0.0 {
            if direction.x != 0.0 {
                self.input_direction.y = 0.0;
                self.input_direction.x = direction.x;
            }
        }

        // Next Tick
        self.time += _dt;
        if self.time < self.delay {
            return Ok(Transition::None);
        }
        self.current_direction = self.input_direction;
        self.time = 0.0;

        // Update Snake Pieces
        for i in (1..self.snake.len()).rev() {
            let next = self.snake[i - 1];
            let current = &mut self.snake[i];
            current.x = next.x;
            current.y = next.y;
        }

        // Move Snake Head
        let mut snake_state = SnakeState::Unknow;

        if let Some(snake) = self.snake.first_mut() {
            *snake = *snake + self.current_direction;

            snake_state = SnakeState::Moving(*snake);

            if snake.x < 0.0 {
                match self.game_mode {
                    GameMode::ACube => {
                        snake.x = GRID_WIDTH as f32;
                        self.current_direction.x *= 1.0;
                    }
                    GameMode::Else => snake_state = SnakeState::Dead,
                }
            }
            if snake.y < 0.0 {
                match self.game_mode {
                    GameMode::ACube => {
                        snake.y = GRID_HEIGHT as f32;
                        self.current_direction.y *= 1.0;
                    }
                    GameMode::Else => snake_state = SnakeState::Dead,
                }
            }
            if snake.x > GRID_WIDTH as f32 {
                match self.game_mode {
                    GameMode::ACube => {
                        snake.x = 0.0;
                        self.current_direction.x *= 1.0;
                    }
                    GameMode::Else => snake_state = SnakeState::Dead,
                }
            }
            if snake.y > GRID_HEIGHT as f32 {
                match self.game_mode {
                    GameMode::ACube => {
                        snake.y = 0.0;
                        self.current_direction.y *= 1.0;
                    }
                    GameMode::Else => snake_state = SnakeState::Dead,
                }
            }

            if *snake == self.frute {
                snake_state = SnakeState::New
            }
        }

        match snake_state {
            SnakeState::New => {
                self.snake.push(self.frute);
                self.refresh_frute();

                if let Some(effect_audio_id) = self.effect_audio_id.as_ref() {
                    _context.play_audio(effect_audio_id.clone());
                }
            }
            SnakeState::Dead => {
                self.restart();
            }
            SnakeState::Moving(head) => {
                for snake in self.snake.iter().skip(1) {
                    if *snake == head {
                        self.restart();
                        break;
                    }
                }
            }
            _ => (),
        }

        let world = &mut _context.get_world();
        let mut despawn = Vec::new();

        let mut snake_copy = self.snake.clone();

        for (_id, transform) in &mut world.query::<With<Snake, &mut Transform2D>>() {
            if let Some(snake) = snake_copy.pop() {
                transform.position = snake * TILE_SIZE as f32;
            } else {
                despawn.push(_id);
            }
        }

        for snake in snake_copy.drain(..) {
            self.spawn_snake_part(world, snake * TILE_SIZE as f32)
        }

        for id in despawn {
            if let Err(e) = world.despawn(id) {
                error!("{:?}", e);
            }
        }

        for (_id, transform) in &mut world.query::<With<Frute, &mut Transform2D>>() {
            transform.position = self.frute * TILE_SIZE as f32;
        }

        Ok(Transition::None)
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("Snake")))
        .with_settings(EngineSettings::WindowSize((WIDTH, HEIGHT)))
        .build()?
        .run(MainState::new())
}
