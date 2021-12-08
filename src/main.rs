use core::{components::Transform2D, AssetManager, EngineBuilder, GameContext, Scene};

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
    ) -> Result<(), ()> {
        let texture = _asset_manager.load_sprite("assets/awesomeface.png");

        let world = &mut _context.get_world();
        world.spawn((
            "sprite",
            Transform2D {
                position: glam::vec2(0.0, 0.0),
                scale: glam::vec2(300.0, 400.0),
                rotate: 0.0,
            },
        ));
        world.spawn((
            "sprite",
            Transform2D {
                position: glam::vec2(600.0, 100.0),
                scale: glam::vec2(300.0, 400.0),
                rotate: 45.0,
            },
        ));
        world.spawn((
            "sprite",
            Transform2D {
                position: glam::vec2(250.0, 400.0),
                scale: glam::vec2(150.0, 200.0),
                rotate: 0.0,
            },
        ));

        Ok(())
    }
}

fn main() {
    env_logger::init();

    EngineBuilder::new(String::from("Hello Engine"), 800, 600)
        .build()
        .unwrap()
        .run(MainState::new());
}
