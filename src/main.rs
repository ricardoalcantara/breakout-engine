use core::{engine::EngineBuilder, state::State};
struct MainState {}

impl MainState {
    fn new() -> Self {
        Self {}
    }
}
impl State for MainState {}

fn main() {
    env_logger::init();

    EngineBuilder::new(String::from("Hello Engine"), 800, 600)
        .build()
        .unwrap()
        .run(MainState::new());
}
