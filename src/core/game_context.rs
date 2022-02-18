use super::{
    asset_manager::AudioId,
    components::{Camera2D, Transform2D},
    game_window::ReadOnlyRc,
};
use crate::{
    physics2d::physics_world::PhysicsWorld, render::renderer::Renderer, shapes::rectangle::Rect,
};
use hecs::World;
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct GameContext {
    pub(crate) clear_color: glam::Vec3,
    pub(crate) world: Rc<RefCell<World>>,
    pub(crate) physics_world: Rc<RefCell<PhysicsWorld>>,
    audio_queue: Vec<AudioId>,
    renderer: ReadOnlyRc<Renderer>,
}

impl GameContext {
    pub(crate) fn new(renderer: ReadOnlyRc<Renderer>) -> Self {
        Self {
            world: Rc::new(RefCell::new(World::new())),
            physics_world: Rc::new(RefCell::new(PhysicsWorld::new())),
            clear_color: glam::Vec3::ZERO,
            audio_queue: Vec::new(),
            renderer,
        }
    }

    pub fn get_world(&self) -> Ref<World> {
        self.world.borrow()
    }

    pub fn get_world_mut(&self) -> RefMut<World> {
        self.world.borrow_mut()
    }

    pub fn get_physics_world(&self) -> Ref<PhysicsWorld> {
        self.physics_world.borrow()
    }

    pub fn get_physics_world_mut(&self) -> RefMut<PhysicsWorld> {
        self.physics_world.borrow_mut()
    }

    pub fn get_camera_rect(&self) -> Option<Rect> {
        // TODO: not yet implemented
        todo!();
        // if let Some((_id, (camera, transform))) = self
        //     .world
        //     .query::<(&Camera2D, &Transform2D)>()
        //     .iter()
        //     .next()
        // {
        //     // let window_size = {
        //     //     let size = self.window.borrow().window().inner_size();
        //     //     glam::uvec2(size.width, size.height)
        //     // };

        //     // Some(
        //     //     camera.get_view_rect(
        //     //         self.renderer
        //     //             .borrow()
        //     //             .render_size()
        //     //             .as_ref()
        //     //             .unwrap_or(&window_size),
        //     //         &window_size,
        //     //         &transform.position,
        //     //     ),
        //     // )
        // } else {
        //     None
        // }
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
