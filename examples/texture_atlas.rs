use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Sprite, SubTexture, Transform2D},
        engine::{EngineBuilder, WindowSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        input::{Event, Input},
        scene::{InputHandled, Scene, Transition},
    },
    error::BreakoutResult,
    math,
    shapes::rectangle::Rect,
};

extern crate log;
extern crate pretty_env_logger;

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
        let texture = _asset_manager.load_texture("assets/awesomeface.png")?;

        let world = &mut _context.get_world();
        world.spawn((
            Sprite {
                texture_id: Some(texture.clone()),
                sub_texture: Some(SubTexture::new(Rect::new(0.0, 0.0, 256.0, 256.0))),
                ..Default::default()
            },
            Transform2D::from_position(math::vec2(10.0, 10.0)),
        ));

        world.spawn((
            Sprite {
                texture_id: Some(texture.clone()),
                flip_x: true,
                flip_y: true,
                ..Default::default()
            },
            Transform2D::from_position(math::vec2(10.0, 200.0)),
        ));

        world.spawn((
            Sprite {
                texture_id: Some(texture.clone()),
                flip_x: true,
                ..Default::default()
            },
            Transform2D::from_position(math::vec2(300.0, 10.0)),
        ));

        world.spawn((
            Sprite {
                texture_id: Some(texture),
                flip_y: true,
                ..Default::default()
            },
            Transform2D::from_position(math::vec2(300.0, 200.0)),
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
        Ok(Transition::None)
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_window_settings(WindowSettings::Title(String::from("Texture Atlas")))
        .with_window_settings(WindowSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
