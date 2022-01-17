use std::{cell::RefCell, rc::Rc};

use hecs::World;

use crate::{render::window::MyWindow, shapes::rectangle::Rect};

use super::{
    asset_manager::AudioId,
    components::{Camera2D, Transform2D},
};

pub struct GameContext {
    pub(crate) clear_color: glam::Vec3,
    pub(crate) world: World,
    audio_queue: Vec<AudioId>,
    window: Rc<RefCell<MyWindow>>,
}

impl GameContext {
    pub(crate) fn new(window: Rc<RefCell<MyWindow>>) -> Self {
        Self {
            world: World::new(),
            clear_color: glam::Vec3::ZERO,
            audio_queue: Vec::new(),
            window,
        }
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn get_camera_rect(&self) -> Option<Rect> {
        if let Some((_id, (camera, transform))) = self
            .world
            .query::<(&Camera2D, &Transform2D)>()
            .iter()
            .next()
        {
            // TODO: It's wrong

            let window_size = {
                let size = self.window.borrow().window().inner_size();
                glam::uvec2(size.width, size.height)
            };

            Some(
                camera.get_view_rect(
                    self.window
                        .borrow()
                        .render_size
                        .as_ref()
                        .unwrap_or(&window_size),
                    &window_size,
                    &transform.position,
                ),
            )
        } else {
            None
        }
    }

    pub fn set_clear_color(&mut self, color: glam::Vec3) {
        self.clear_color = color;
    }

    pub fn play_audio(&mut self, audio_id: AudioId) {
        self.audio_queue.push(audio_id);
    }

    pub(crate) fn take_audio_queue(&mut self) -> Vec<AudioId> {
        self.audio_queue.drain(..).collect()
    }
}
