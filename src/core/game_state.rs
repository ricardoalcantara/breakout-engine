use super::{
    engine::EngineSettings,
    input::Input,
    scene::{InputHandled, Scene, Transition},
};
use crate::core::{
    asset_manager::AssetManager,
    components::{Sprite, Transform2D},
    engine_context::EngineContext,
    game_context::GameContext,
};
use audio::audio_player::AudioPlayer;
use hecs::World;
use render::{renderer::Renderer2D, window::MyWindow};

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    renderer: Box<dyn Renderer2D>,
    context: GameContext,
    engine: EngineContext,
    asset_manager: AssetManager,
    input: Input,
    music_player: AudioPlayer,
}

impl GameState {
    pub fn new<S, R>(state: S, renderer: R, window: &MyWindow) -> Self
    where
        S: Scene + 'static,
        R: Renderer2D + 'static,
    {
        let mut engine = EngineContext::new(&window);
        let mut context = GameContext::new();
        let mut asset_manager = AssetManager::new();

        let mut state = state;
        state
            .init(&mut context, &mut asset_manager, &mut engine)
            .unwrap();

        for (id, preloaded_texture) in asset_manager.take_preload_textures() {
            let texture = renderer.generate_texture(preloaded_texture);
            asset_manager.add_texture(id, texture);
        }

        let input = Input::new();
        let music_player = AudioPlayer::new();
        Self {
            scenes: vec![Box::new(state)],
            renderer: Box::new(renderer),
            context,
            engine,
            asset_manager,
            input,
            music_player,
        }
    }

    pub fn take_settings(&mut self) -> Vec<EngineSettings> {
        self.engine.take_settings()
    }

    pub fn resize(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.as_ref().resize(new_size);
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> Result<bool, ()> {
        if let Some(on_event) = self.input.on_event(event) {
            match self.scenes.last_mut() {
                Some(active_scene) => {
                    match active_scene.input(on_event, &mut self.context, &mut self.engine)? {
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
                    }
                }
                None => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    pub fn update(&mut self, delta: f32) -> Result<bool, ()> {
        let result = match self.scenes.last_mut() {
            Some(active_scene) => {
                match active_scene.update(
                    delta,
                    &mut self.input,
                    &mut self.context,
                    &mut self.engine,
                )? {
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

        for audio_id in self.context.take_audio_queue() {
            let audio = self.asset_manager.get_audio(&audio_id);
            self.music_player.play(audio)
        }

        result
    }

    pub fn render(&mut self, window: &MyWindow) -> Result<(), ()> {
        let world = &mut self.context.world;

        self.renderer.clear_color(self.context.clear_color);

        system_render_sprite(world, self.renderer.as_mut(), &mut self.asset_manager);
        // Todo: Encapsulate ir
        window.swap_buffers();

        Ok(())
    }
}

fn system_render_sprite(
    world: &mut World,
    renderer: &mut dyn Renderer2D,
    asset_manager: &AssetManager,
) {
    for (_id, (sprite, transform)) in world.query_mut::<(&Sprite, &Transform2D)>() {
        // Todo: Generate default texture
        let texture = if let Some(texture_id) = &sprite.texture_id {
            Some(asset_manager.get_texture(&texture_id))
        } else {
            None
        };
        renderer.draw_texture(
            texture,
            sprite.rect,
            transform.position,
            transform.scale,
            transform.rotate,
            sprite.color.unwrap_or(glam::vec3(1.0, 1.0, 1.0)),
        );
    }
}
