use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Label, Sprite, Transform2D},
        engine::{EngineBuilder, EngineSettings},
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
        // let font = _asset_manager.load_font("assets/Roboto-Regular.ttf")?;

        let world = &mut _context.get_world();

        // world.spawn((
        //     Label::new_with_font(String::from("Hello world"), font, 48.0),
        //     Transform2D::default(),
        // ));

        world.spawn((
            Sprite {
                color: Some(math::vec4(0.0, 1.0, 0.0, 0.5)),
                ..Default::default()
            },
            Transform2D {
                position: math::vec2(10.0, 60.0),
                scale: math::vec2(10.0, 10.0),
                ..Default::default()
            },
        ));

        world.spawn((
            Label::new(String::from("Hello World"), 48.0),
            Transform2D {
                position: math::vec2(10.0, 60.0),
                ..Default::default()
            },
        ));

        // world.spawn((
        //     Label {
        //         ..Default::default()
        //     },
        //     Transform2D {
        //         position: math::vec2(0.0, 60.0),
        //         ..Default::default()
        //     },
        // ));

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
        .with_settings(EngineSettings::Title(String::from("Menu")))
        .with_settings(EngineSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
