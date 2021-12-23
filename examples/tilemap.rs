extern crate log;
extern crate pretty_env_logger;
use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Sprite, Transform2D},
        engine::{EngineBuilder, EngineSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        input::{Event, Input},
        scene::{InputHandled, Scene, Transition},
    },
    error::BreakoutResult,
    math,
};
use log::info;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TILE_SIZE: u32 = 16;
const GRID_WIDTH: u32 = (WIDTH / TILE_SIZE) - 1;
const GRID_HEIGHT: u32 = (HEIGHT / TILE_SIZE) - 1;

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
    ) -> BreakoutResult {
        let args: Vec<String> = std::env::args().collect();
        info!("{:?}", args);
        let (width, height): (u32, u32) = if args.len() > 2 {
            let width: u32 = args[1].trim().parse().unwrap_or(GRID_WIDTH);
            let height: u32 = args[2].trim().parse().unwrap_or(GRID_HEIGHT);
            (width, height)
        } else {
            (GRID_WIDTH, GRID_HEIGHT)
        };

        let world = &mut _context.get_world();

        for x in 0..width {
            for y in 0..height {
                world.spawn((
                    Sprite {
                        color: Some(math::vec4(1.0, 0.0, 0.0, 1.0)),
                        ..Default::default()
                    },
                    Transform2D {
                        position: math::vec2((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32),
                        scale: math::vec2((TILE_SIZE - 1) as f32, (TILE_SIZE - 1) as f32),
                        rotate: 0.0,
                    },
                ));
            }
        }
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
        Ok(Transition::None)
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("Tilemap")))
        .with_settings(EngineSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
