extern crate log;
extern crate pretty_env_logger;
use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Camera2D, Sprite, Transform2D},
        engine::{EngineBuilder, RenderSettings, WindowSettings},
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

struct InputTag;

struct MainState {
    rotation: f32,
}

impl MainState {
    fn new() -> Self {
        Self { rotation: 0.0 }
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

        let world = &mut _context.get_world_mut();

        for x in 0..width {
            for y in 0..height {
                world.spawn((
                    Sprite {
                        color: Some(math::vec4(1.0, 0.0, 0.0, 1.0)),
                        center_origin: true,
                        ..Default::default()
                    },
                    Transform2D::from_position_rotation_scale(
                        math::vec2((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32),
                        0.0,
                        math::vec2((TILE_SIZE - 1) as f32, (TILE_SIZE - 1) as f32),
                    ),
                ));
            }
        }

        world.spawn((InputTag, Camera2D::keep_width(0.5), Transform2D::new()));
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
        let world = &mut _context.get_world();
        self.rotation += _dt;
        if self.rotation > std::f32::consts::TAU {
            self.rotation = std::f32::consts::TAU - self.rotation;
        }
        for (_id, _transform) in &mut world.query::<&mut Transform2D>() {
            _transform.set_rotate(self.rotation);
        }
        Ok(Transition::None)
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_window_settings(WindowSettings::Title(String::from("Tilemap")))
        .with_window_settings(WindowSettings::WindowSize((800, 600)))
        .with_render_settings(RenderSettings::DisplaySize((800, 600)))
        .build()?
        .run(MainState::new())
}
