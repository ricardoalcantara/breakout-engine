use crate::{
    asset_manager::AssetManager, components::Transform2D, game_context::GameContext, Events,
};
use render::{InternalWindow, Render2D};

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    renderer: Box<dyn Render2D>,
    context: GameContext,
    asset_manager: AssetManager,
}

impl GameState {
    pub fn new<S, R>(state: S, renderer: R) -> Self
    where
        S: Scene + 'static,
        R: Render2D + 'static,
    {
        let mut context = GameContext::new();
        let mut asset_manager = AssetManager::new();

        let mut state = state;
        state.init(&mut context, &mut asset_manager).unwrap();

        Self {
            scenes: vec![Box::new(state)],
            renderer: Box::new(renderer),
            context,
            asset_manager,
        }
    }

    pub fn resize(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.as_ref().resize(new_size);
    }

    pub fn input(&self, event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) -> Result<(), ()> {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(&self.context, 0.0)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                Transition::Replace(s) => {
                    self.scenes.pop();
                    self.scenes.push(s);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None => (),
        }

        Ok(())
    }

    pub fn render(&mut self, window: &InternalWindow) -> Result<(), ()> {
        let texture = self.asset_manager.texture.as_ref().unwrap();

        let world = self.context.get_world();

        for (_id, transform) in world.query_mut::<&Transform2D>() {
            self.renderer.draw_texture(
                texture,
                transform.position,
                transform.scale,
                transform.rotate,
                glam::vec3(0.0, 1.0, 0.0),
            );
        }

        // self.renderer.draw_texture(
        //     texture,
        //     glam::vec2(200.0, 200.0),
        //     glam::vec2(300.0, 400.0),
        //     45.0,
        //     glam::vec3(0.0, 1.0, 0.0),
        // );

        // Todo: Encapsulate ir
        window.swap_buffers();

        Ok(())
    }
}

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
}

pub trait Scene {
    fn resize(&mut self) {}

    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
    ) -> Result<(), ()> {
        Ok(())
    }

    fn input(&mut self, _context: &GameContext, _events: Events) -> Result<Transition, ()> {
        Ok(Transition::None)
    }

    fn update(&mut self, _context: &GameContext, _dt: f32) -> Result<Transition, ()> {
        Ok(Transition::None)
    }
}
