use super::engine::EngineSettings;
use crate::render::window::MyWindow;

pub struct EngineContext {
    #[warn(dead_code)]
    engine_settings: Vec<EngineSettings>,
    is_fullscreen: bool,
    pub(crate) window_size: glam::UVec2,
}

impl EngineContext {
    pub fn new(window: &MyWindow) -> EngineContext {
        EngineContext {
            engine_settings: Vec::new(),
            is_fullscreen: window.window().fullscreen().is_some(),
            window_size: {
                let size = window.window().inner_size();
                glam::uvec2(size.width, size.height)
            },
        }
    }

    pub fn update_settings(&mut self, engine_settings: EngineSettings) {
        if let EngineSettings::Fullscreen(set) = engine_settings {
            self.is_fullscreen = set;
        }
        self.engine_settings.push(engine_settings);
    }

    pub fn take_settings(&mut self) -> Vec<EngineSettings> {
        self.engine_settings.drain(..).collect()
    }

    pub fn fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    pub fn window_size(&self) -> glam::UVec2 {
        self.window_size
    }
}
