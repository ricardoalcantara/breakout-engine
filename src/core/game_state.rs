use super::{
    components::Label,
    engine::EngineSettings,
    input::Input,
    scene::{InputHandled, Scene, Transition},
};
use crate::{
    audio::AudioPlayer,
    core::{
        asset_manager::AssetManager,
        components::{Sprite, Transform2D},
        engine_context::EngineContext,
        game_context::GameContext,
    },
    error::BreakoutResult,
    render::{font::Font, renderer::Renderer2D, window::MyWindow},
};
use hecs::World;
use image::GenericImageView;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    renderer: Rc<RefCell<dyn Renderer2D>>,
    context: GameContext,
    engine: EngineContext,
    asset_manager: AssetManager,
    input: Input,
    music_player: AudioPlayer,
    default_font: Font,
}

impl GameState {
    pub fn new<S, R>(state: S, renderer: R, window: &MyWindow) -> BreakoutResult<Self>
    where
        S: Scene + 'static,
        R: Renderer2D + 'static,
    {
        let renderer = Rc::new(RefCell::new(renderer));
        let mut engine = EngineContext::new(&window);
        let mut context = GameContext::new();
        let mut asset_manager = AssetManager::new(Rc::clone(&renderer));

        let mut state = state;
        state
            .init(&mut context, &mut asset_manager, &mut engine)
            .unwrap();

        let input = Input::new();
        let music_player = AudioPlayer::new();
        let default_font_byte = include_bytes!("../../assets/Roboto-Regular.ttf");

        let default_font = Font::new_from_bytes(default_font_byte)?;
        Ok(Self {
            scenes: vec![Box::new(state)],
            renderer,
            context,
            engine,
            asset_manager,
            input,
            music_player,
            default_font,
        })
    }

    pub fn take_settings(&mut self) -> Vec<EngineSettings> {
        self.engine.take_settings()
    }

    pub fn resize(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.as_ref().borrow().resize(new_size);
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> BreakoutResult<bool> {
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
                        // TODO: False will let esc close the window
                        _ => Ok(false),
                    }
                }
                None => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    pub fn update(&mut self, delta: f32) -> BreakoutResult<bool> {
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

        for audio_queue in self.context.take_audio_queue() {
            let audio = self.asset_manager.get_audio(&audio_queue);
            self.music_player.play(audio);
        }

        result
    }

    pub fn render(&mut self, window: &MyWindow) -> BreakoutResult {
        self.system_render_sprite();

        window.swap_buffers();
        Ok(())
    }

    fn system_render_sprite(&self) {
        let world = &self.context.world;

        let mut renderer = self.renderer.borrow_mut();
        renderer.clear_color(self.context.clear_color);

        renderer.begin_draw();
        for (_id, (sprite, transform)) in world.query::<(&Sprite, &Transform2D)>().iter() {
            if let Some(texture_id) = &sprite.texture_id {
                let texture = self.asset_manager.get_texture(&texture_id);
                renderer.draw_texture(
                    texture,
                    sprite.rect,
                    transform.position,
                    transform.scale,
                    transform.rotate,
                    sprite.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
                );
            } else {
                renderer.draw_quad(
                    glam::Vec2::ONE,
                    transform.position,
                    transform.scale,
                    transform.rotate,
                    sprite.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
                );
            };
        }

        for (_id, (label, transform)) in world.query::<(&mut Label, &Transform2D)>().iter() {
            if label.texture.is_none() {
                let font = if let Some(font_id) = &label.font_id {
                    self.asset_manager.get_font(font_id)
                } else {
                    &self.default_font
                };
                let image = font.get_texture_from_text(&label.text, label.size);
                let (width, height) = image.dimensions();
                let texture = renderer.generate_texture(image).unwrap();

                // TODO: Load it before this stage, end user will only get width and height after the first render
                label.texture = Some(texture);
                label.width = width as f32;
                label.height = height as f32;
            }

            renderer.draw_texture(
                // TODO: Error Prone
                label.texture.as_ref().unwrap(),
                None,
                transform.position,
                transform.scale,
                transform.rotate,
                label.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
            );
        }
        renderer.end_draw();
    }
}
