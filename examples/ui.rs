use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Sprite, Transform2D},
        engine::{EngineBuilder, EngineSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        scene::Scene,
        ui_context::UIContext,
    },
    error::BreakoutResult,
    gui::Constraints,
    math,
};
use log::info;

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
        _context.set_clear_color(math::vec3(0.6, 0.6, 0.6));

        // let world = &mut _context.get_world();
        // world.spawn((
        //     Sprite {
        //         color: Some(math::vec4(1.0, 0.0, 0.0, 1.0)),
        //         center_origin: true,
        //         ..Default::default()
        //     },
        //     Transform2D::from_position_rotation_scale(
        //         math::vec2(50.0, 50.0),
        //         0.0,
        //         math::vec2(50.0, 50.0),
        //     ),
        // ));

        Ok(())
    }

    fn ui(&mut self, ui: &mut UIContext) {
        ui.begin_screen(|ui_screen| {
            ui_screen.label("Hello World", math::ivec2(10, 10));
            // ui_screen.texture()
            if ui_screen.button("Click me", math::ivec2(10, 20)) {
                info!("Clicked");
            }
        });

        ui.begin_panel("My Panel", |ui_panel| {
            // ui_panel.set_x(Constraints::Pixel(20));
            // ui_panel.set_y(Constraints::Pixel(20));
            // ui_panel.set_width(Constraints::Pixel(400));
            // ui_panel.set_height(Constraints::Pixel(300));

            ui_panel.label("Hello World");
            // ui_panel.texture()
            if ui_panel.button("Click me") {
                info!("Clicked");
            }
        });
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("UI")))
        .with_settings(EngineSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
