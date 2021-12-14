use render::window::MyWindow;

use crate::EngineSettings;

pub struct EngineContext {
    #[warn(dead_code)]
    engine_settings: Vec<EngineSettings>,
    is_fullscreen: bool,
}

impl EngineContext {
    pub fn new(window: &MyWindow) -> EngineContext {
        EngineContext {
            engine_settings: Vec::new(),
            is_fullscreen: window.window().fullscreen().is_some(),
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
}
