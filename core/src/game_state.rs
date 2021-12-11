use crate::{
    asset_manager::AssetManager,
    components::{Sprite, Transform2D},
    game_context::GameContext,
    Event, Input, InputHandled, Scene, Transition,
};
use log::info;
use render::{renderer::Renderer2D, window::MyWindow};
use winit::event::WindowEvent;

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    renderer: Box<dyn Renderer2D>,
    context: GameContext,
    asset_manager: AssetManager,
    input: Input,
}

impl GameState {
    pub fn new<S, R>(state: S, renderer: R, window: &MyWindow) -> Self
    where
        S: Scene + 'static,
        R: Renderer2D + 'static,
    {
        // Todo: EngineConfiguration
        window.window().set_cursor_visible(false);
        window.window().set_title("false");
        // window
        //     .window()
        //     .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

        let mut context = GameContext::new();
        let mut asset_manager = AssetManager::new();

        let mut state = state;
        state.init(&mut context, &mut asset_manager).unwrap();

        for (id, preloaded_texture) in asset_manager.get_preload_textures() {
            let texture = renderer.generate_texture(preloaded_texture);
            asset_manager.add_texture(id, texture);
        }

        let input = Input::new();

        Self {
            scenes: vec![Box::new(state)],
            renderer: Box::new(renderer),
            context,
            asset_manager,
            input,
        }
    }

    pub fn resize(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.as_ref().resize(new_size);
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> Result<bool, ()> {
        if let Some(on_event) = self.input.on_event(event) {
            match self.scenes.last_mut() {
                Some(active_scene) => match active_scene.input(on_event, &mut self.context)? {
                    InputHandled::Transition(transition) => {
                        match transition {
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
                        };
                        Ok(true)
                    }
                    InputHandled::Captured => Ok(true),
                    // Todo: False will let esc close the window
                    _ => Ok(false),
                },
                None => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    pub fn update(&mut self) -> Result<bool, ()> {
        let result = match self.scenes.last_mut() {
            Some(active_scene) => {
                match active_scene.update(&mut self.input, &mut self.context, 0.0)? {
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
                }
                Ok(true)
            }
            None => Ok(false),
        };
        self.input.end_frame();

        result
    }

    pub fn render(&mut self, window: &MyWindow) -> Result<(), ()> {
        let world = self.context.get_world();

        self.renderer.clean_color();

        for (_id, (sprite, transform)) in world.query_mut::<(&Sprite, &Transform2D)>() {
            let texture = self.asset_manager.get_texture(&sprite.texture_id);
            self.renderer.draw_texture(
                texture,
                transform.position,
                transform.scale,
                transform.rotate,
                glam::vec3(1.0, 1.0, 1.0),
            );
        }

        // Todo: Encapsulate ir
        window.swap_buffers();

        Ok(())
    }
}
