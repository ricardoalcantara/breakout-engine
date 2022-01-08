use super::{
    components::{Camera2D, Label},
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
    font::Font,
    render::{
        renderer::{RenderQuad, RenderText, RenderTexture, Renderer2D},
        texture::Texture,
        window::MyWindow,
    },
};

use std::{cell::RefCell, rc::Rc};

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    renderer: Rc<RefCell<dyn Renderer2D>>,
    context: GameContext,
    engine: EngineContext,
    asset_manager: AssetManager,
    input: Input,
    music_player: AudioPlayer,
    default_font: Font<Texture>,
    window_size: glam::UVec2,
}

impl GameState {
    pub fn new<S, R>(state: S, renderer: R, window: &MyWindow) -> BreakoutResult<Self>
    where
        S: Scene + 'static,
        R: Renderer2D + 'static,
    {
        let renderer = Rc::new(RefCell::new(renderer));
        let mut engine = EngineContext::new(&window);
        let mut context = GameContext::new(&window);
        let mut asset_manager = AssetManager::new(Rc::clone(&renderer));

        let mut state = state;
        state
            .init(&mut context, &mut asset_manager, &mut engine)
            .unwrap();

        let input = Input::new();
        let music_player = AudioPlayer::new();
        let default_font_byte = include_bytes!("../../assets/Roboto-Regular.ttf");
        let default_font = Font::<Texture>::new_from_memory(default_font_byte);
        let size = window.window().inner_size();
        let window_size = glam::uvec2(size.width, size.height);

        Ok(Self {
            scenes: vec![Box::new(state)],
            renderer,
            context,
            engine,
            asset_manager,
            input,
            music_player,
            default_font,
            window_size,
        })
    }

    pub fn take_settings(&mut self) -> Vec<EngineSettings> {
        self.engine.take_settings()
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = glam::uvec2(new_size.width, new_size.height);
        self.engine.window_size = glam::uvec2(new_size.width, new_size.height);
        self.context.window_size = glam::uvec2(new_size.width, new_size.height);
        self.renderer.as_ref().borrow_mut().resize(new_size);
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

    fn system_render_sprite(&mut self) {
        let world = &self.context.world;

        let mut renderer = self.renderer.borrow_mut();
        renderer.clear_color(self.context.clear_color);

        let camera_projection = if let Some((_id, (camera, transform))) =
            world.query::<(&Camera2D, &Transform2D)>().iter().next()
        {
            Some(camera.get_view_matrix(&self.window_size, &transform.position))
        } else {
            None
        };

        renderer.begin_draw(camera_projection);
        for (_id, (sprite, transform)) in world.query::<(&Sprite, &Transform2D)>().iter() {
            if !sprite.visible {
                continue;
            }
            let position = if transform.pixel_snap {
                glam::vec2(
                    transform.position.x as i32 as f32,
                    transform.position.y as i32 as f32,
                )
            } else {
                transform.position
            };
            if let Some(texture_id) = &sprite.texture_id {
                let texture = self.asset_manager.get_texture(&texture_id);
                renderer.draw_texture(RenderTexture {
                    texture: texture,
                    rect: sprite.rect,
                    position: position,
                    scale: transform.scale,
                    rotate: transform.rotate,
                    center_origin: sprite.center_origin,
                    color: sprite.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
                });
            } else {
                renderer.draw_quad(RenderQuad {
                    size: glam::Vec2::ONE,
                    position: position,
                    scale: transform.scale,
                    rotate: transform.rotate,
                    center_origin: sprite.center_origin,
                    color: sprite.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
                });
            };
        }

        for (_id, (label, _transform)) in world.query::<(&mut Label, &Transform2D)>().iter() {
            if !label.visible {
                continue;
            }

            self.default_font.build_with_size(label.size, |image| {
                renderer.generate_texture(image).unwrap()
            });

            renderer.draw_text(RenderText {
                text: &label.text,
                font: &self.default_font,
                size: label.size,
                position: _transform.position,
                scale: _transform.scale,
                color: label.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
            });
        }
        renderer.end_draw();
    }
}
