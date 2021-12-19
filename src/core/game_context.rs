use audio::audio_player::AudioPlayer;
use hecs::World;

use super::asset_manager::AudioId;

pub(crate) enum AudioQueuePlayer {
    Once(AudioId),
    Loop(AudioId),
}

pub struct GameContext {
    pub(crate) clear_color: glam::Vec3,
    pub(crate) world: World,
    audio_queue: Vec<AudioQueuePlayer>,
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

    pub fn play_audio(&mut self, audio_id: AudioId, repeat_infinite: bool) {
        self.audio_queue.push(if repeat_infinite {
            AudioQueuePlayer::Loop(audio_id)
        } else {
            AudioQueuePlayer::Once(audio_id)
        });
    }

    pub(crate) fn take_audio_queue(&mut self) -> Vec<AudioQueuePlayer> {
        self.audio_queue.drain(..).collect()
    }
}
