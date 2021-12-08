use core::{engine::EngineBuilder, state::Scene};

struct MainState {}

impl MainState {
    fn new() -> Self {
        Self {}
    }
}
impl Scene for MainState {
    fn init(&mut self, _context: &mut core::state::Context) -> Result<(), ()> {
        let texture = _context.load_sprite("assets/awesomeface.png");
        let world = &mut _context.get_world();
        world.spawn(("sprite", 1));

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
