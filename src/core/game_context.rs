use hecs::World;

pub struct GameContext {
    world: World,
}

impl GameContext {
    pub(crate) fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}
