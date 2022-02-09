use std::rc::Rc;

use super::{
    engine::{EngineTimerView, WindowSettings},
    game_window::{ReadOnlyRc, ReadWriteRc},
    input::Input,
    scene::{InputHandled, Scene, Transition},
    systems::{
        animated_sprite::system_update_animated_sprite, font::system_render_font_texture,
        sprite::system_render_sprite,
    },
    ui_context::UIContext,
};
use crate::{
    audio::AudioPlayer,
    core::{asset_manager::AssetManager, engine_context::EngineContext, game_context::GameContext},
    error::BreakoutResult,
    font::Font,
    render::renderer::Renderer,
};

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    context: GameContext,
    engine: EngineContext,
    ui_context: UIContext,
    asset_manager: AssetManager,
    input: Input,
    music_player: AudioPlayer,
    default_font: Rc<Font>,
}

impl GameState {
    pub fn new<S>(state: S, renderer: ReadOnlyRc<Renderer>) -> BreakoutResult<Self>
    where
        S: Scene + 'static,
    {
        let ui_context = UIContext::new()?;
        let mut engine = EngineContext::new();
        let mut context = GameContext::new(renderer.clone());
        let mut asset_manager = AssetManager::new(renderer);

        let mut state = state;
        state
            .init(&mut context, &mut asset_manager, &mut engine)
            .unwrap();

        let input = Input::new();
        let music_player = AudioPlayer::new();
        let default_font_byte = include_bytes!("../../assets/Roboto-Regular.ttf");
        let default_font = Rc::new(Font::new_from_memory(default_font_byte)?);

        Ok(Self {
            scenes: vec![Box::new(state)],
            context,
            engine,
            ui_context,
            asset_manager,
            input,
            music_player,
            default_font,
        })
    }

    pub fn take_settings(&mut self) -> Vec<WindowSettings> {
        self.engine.take_window_settings()
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> BreakoutResult<bool> {
        if self.ui_context.on_event(event) {
            return Ok(true);
        }

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
                    Transition::None => {
                        active_scene.ui(&mut self.context, &mut self.ui_context);
                    }
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

        // TODO system_update_audio
        for audio_queue in self.context.take_audio_queue() {
            let audio = self.asset_manager.get_audio(&audio_queue);
            self.music_player.play(audio);
        }
        system_update_animated_sprite(&self.context, delta);

        result
    }

    pub fn render(
        &mut self,
        renderer: ReadWriteRc<Renderer>,
        view_time: EngineTimerView,
    ) -> BreakoutResult {
        let mut renderer_borrowed_mut = renderer.borrow_mut();
        system_render_font_texture(
            &self.context,
            &mut self.asset_manager,
            &renderer_borrowed_mut,
            &mut self.default_font,
        )?;
        system_render_sprite(
            &self.context,
            &self.asset_manager,
            &mut renderer_borrowed_mut,
            &self.default_font,
        )?;

        // TODO not yet implemented
        // self.ui_context
        //     .render(&mut renderer_borrowed_mut, &view_time);
        Ok(())
    }
}
