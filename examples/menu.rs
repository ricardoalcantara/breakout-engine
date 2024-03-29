use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Label, Sprite, Transform2D},
        engine::{EngineBuilder, WindowSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        input::{Event, Input},
        scene::{InputHandled, Scene, Transition},
    },
    error::BreakoutResult,
    math,
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
        let font = _asset_manager.load_font("assets/Roboto-Regular.ttf")?;

        let world = &mut _context.get_world_mut();

        world.spawn((
            Label::new_with_font(String::from("First Word"), font, 20),
            Transform2D::default(),
        ));

        world.spawn((
            Sprite {
                color: Some(math::vec4(0.0, 1.0, 0.0, 0.5)),
                ..Default::default()
            },
            Transform2D::from_position_rotation_scale(
                math::vec2(10.0, 60.0),
                0.0,
                math::vec2(10.0, 10.0),
            ),
        ));

        world.spawn((
            Label::new(String::from("Hello World"), 60),
            Transform2D::from_position(math::vec2(10.0, 60.0)),
        ));

        world.spawn((
            Label::new(String::from("Breakout\nEngine"), 48),
            Transform2D::from_position(math::vec2(10.0, 160.0)),
        ));

        world.spawn((
            Label {
                ..Default::default()
            },
            Transform2D::from_position(math::vec2(0.0, 60.0)),
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
        .with_window_settings(WindowSettings::Title(String::from("Menu")))
        .with_window_settings(WindowSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
