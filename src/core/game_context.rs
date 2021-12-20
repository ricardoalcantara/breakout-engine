use hecs::World;

use super::asset_manager::AudioId;

pub struct GameContext {
    pub(crate) clear_color: glam::Vec3,
    pub(crate) world: World,
    audio_queue: Vec<AudioId>,
}

impl GameContext {
    pub(crate) fn new() -> Self {
        Self {
            world: World::new(),
            clear_color: glam::Vec3::ZERO,
            audio_queue: Vec::new(),
        }
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
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
