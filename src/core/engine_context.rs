use super::engine::{RenderSettings, WindowSettings};

pub struct EngineContext {
    engine_settings: Vec<WindowSettings>,
    render_settings: Vec<RenderSettings>,
}

impl EngineContext {
    pub fn new() -> EngineContext {
        EngineContext {
            engine_settings: Vec::new(),
            render_settings: Vec::new(),
        }
    }

    pub fn update_window_settings(&mut self, engine_settings: WindowSettings) {
        self.engine_settings.push(engine_settings);
    }

    pub fn update_render_settings(&mut self, render_settings: RenderSettings) {
        self.render_settings.push(render_settings);
    }

    pub fn take_window_settings(&mut self) -> Vec<WindowSettings> {
        self.engine_settings.drain(..).collect()
    }

    pub fn fullscreen(&self) -> bool {
        todo!()
        // TODO not yet implemented
        // self.window.borrow().window().fullscreen().is_some()
    }

    pub fn window_size(&self) -> glam::UVec2 {
        todo!()
        // TODO not yet implemented
        // let size = self.window.borrow().window().inner_size();
        // glam::uvec2(size.width, size.height)
    }
}
