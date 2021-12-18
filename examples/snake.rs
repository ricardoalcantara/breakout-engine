extern crate log;
extern crate pretty_env_logger;

use core::{
    components::{Sprite, Transform2D},
    engine_context::EngineContext,
    AssetManager, EngineBuilder, EngineSettings, GameContext, Input, Scene, VirtualKeyCode,
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
    Moving(glam::Vec2),
    New,
    Dead,
}

struct Snake;
struct Frute;
struct MainState {
    snake: Vec<glam::Vec2>,
    frute: glam::Vec2,
    direction: glam::Vec2,
    time: f32,
    delay: f32,
    game_mode: GameMode,
}

fn get_input_direction(_input: &mut Input) -> glam::Vec2 {
    let mut direction = glam::Vec2::ZERO;

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
            direction: glam::vec2(1.0, 0.0),
            frute: glam::Vec2::ZERO,
            time: f32::MAX,
            delay: 0.1,
            game_mode: GameMode::ACube,
        }
    }

    fn spawn_snake_part(&self, world: &mut World, position: glam::Vec2) {
        world.spawn((
            Sprite {
                ..Default::default()
            },
            Transform2D {
                position,
                scale: glam::vec2(TILE_SIZE as f32, TILE_SIZE as f32),
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
            self.direction = glam::vec2(1.0, 0.0);
        } else {
            self.direction = glam::vec2(0.0, 1.0);
        }
        self.snake.push(glam::Vec2::ZERO);

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
    ) -> Result<(), ()> {
        let args: Vec<String> = std::env::args().collect();
        info!("{:?}", args);
        if args.len() > 2 && args[1] == "game_mode" && args[2] == "else" {
            self.game_mode = GameMode::Else;
        }

        let world = &mut _context.get_world();

        self.start();
        world.spawn((
            Sprite {
                color: Some(glam::vec3(1.0, 0.0, 0.0)),
                ..Default::default()
            },
            Transform2D {
                position: glam::vec2(
                    (self.frute.x as u32 * TILE_SIZE) as f32,
                    (self.frute.y as u32 * TILE_SIZE) as f32,
                ),
                scale: glam::vec2(TILE_SIZE as f32, TILE_SIZE as f32),
                rotate: 0.0,
            },
            Frute,
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
        let direction = get_input_direction(_input);

        if self.direction.x != 0.0 {
            if direction.y != 0.0 {
                self.direction.x = 0.0;
                self.direction.y = direction.y;
            }
        } else if self.direction.y != 0.0 {
            if direction.x != 0.0 {
                self.direction.y = 0.0;
                self.direction.x = direction.x;
            }
        }

        // Next Tick
        self.time += _dt;
        if self.time < self.delay {
            return Ok(core::Transition::None);
        }
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
            *snake = *snake + self.direction;

            snake_state = SnakeState::Moving(*snake);

            if *snake == self.frute {
                snake_state = SnakeState::New
            }

            if snake.x < 0.0 {
                match self.game_mode {
                    GameMode::ACube => {
                        snake.x = GRID_WIDTH as f32;
                        self.direction.x *= 1.0;
                    }
                    GameMode::Else => snake_state = SnakeState::Dead,
                }
            }
            if snake.y < 0.0 {
                match self.game_mode {
                    GameMode::ACube => {
                        snake.y = GRID_HEIGHT as f32;
                        self.direction.y *= 1.0;
                    }
                    GameMode::Else => snake_state = SnakeState::Dead,
                }
            }
            if snake.x > GRID_WIDTH as f32 {
                match self.game_mode {
                    GameMode::ACube => {
                        snake.x = 0.0;
                        self.direction.x *= 1.0;
                    }
                    GameMode::Else => snake_state = SnakeState::Dead,
                }
            }
            if snake.y > GRID_HEIGHT as f32 {
                match self.game_mode {
                    GameMode::ACube => {
                        snake.y = 0.0;
                        self.direction.y *= 1.0;
                    }
                    GameMode::Else => snake_state = SnakeState::Dead,
                }
            }
        }

        let world = &mut _context.get_world();
        let mut despawn = Vec::new();

        match snake_state {
            SnakeState::New => {
                self.snake.push(self.frute);
                self.refresh_frute();
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

        Ok(core::Transition::None)
    }
}

fn main() {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("Snake")))
        .with_settings(EngineSettings::WindowSize((WIDTH, HEIGHT)))
        .build()
        .unwrap()
        .run(MainState::new());
}
