extern crate log;
extern crate pretty_env_logger;

use core::{
    components::{Sprite, Transform2D},
    engine_context::EngineContext,
    AssetManager, EngineBuilder, EngineSettings, GameContext, Input, Scene,
};

use shapes::rectangle::Rectangle;

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
    ) -> Result<(), ()> {
        let awesomeface = _asset_manager.load_sprite("assets/awesomeface.png");
        let world = &mut _context.get_world();

        world.spawn((
            Sprite {
                texture_id: Some(awesomeface),
                rect: Some(Rectangle::new(0.0, 0.0, 256.0, 256.0)),
                ..Default::default()
            },
            Transform2D {
                position: glam::vec2(144.0, 44.0),
                scale: glam::vec2(0.5, 0.5),
                rotate: 0.0,
            },
        ));

        world.spawn((
            Sprite {
                ..Default::default()
            },
            Transform2D {
                position: glam::vec2(0.0, 0.0),
                scale: glam::vec2(200.0, 200.0),
                rotate: 0.0,
            },
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
        Ok(core::Transition::None)
    }
}

fn main() {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("Breakout")))
        .with_settings(EngineSettings::WindowSize((800, 600)))
        .build()
        .unwrap()
        .run(MainState::new());
}
