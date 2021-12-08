use std::{
    cell::{RefCell, RefMut},
    path::Path,
    rc::Rc,
};

use hecs::World;
use render::{texture::Texture, InternalWindow, Render2D};

use crate::Events;

pub struct Context {
    world: World,
    texture: Option<Box<dyn Texture>>,
}

impl Context {
    fn new() -> Self {
        Self {
            world: World::new(),
            texture: None,
        }
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn load_sprite(&mut self, path: &str) {
        let mut texture = render::opengl::texture::OpenGLTexture::new();
        texture.generate(path);

        self.texture = Some(Box::new(texture));
    }
}

pub struct GameState {
    scenes: Vec<Box<dyn Scene>>,
    renderer: Box<dyn Render2D>,
    context: Context,
}

impl GameState {
    pub fn new<S, R>(state: S, renderer: R) -> Self
    where
        S: Scene + 'static,
        R: Render2D + 'static,
    {
        let mut context = Context::new();
        let mut state = state;
        state.init(&mut context).unwrap();

        Self {
            scenes: vec![Box::new(state)],
            renderer: Box::new(renderer),
            context,
        }
    }

    pub fn resize(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.as_ref().resize(new_size);
    }

    pub fn input(&self, event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) -> Result<(), ()> {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(&self.context, 0.0)? {
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
            },
            None => (),
        }

        Ok(())
    }

    pub fn render(&mut self, window: &InternalWindow) -> Result<(), ()> {
        let texture = { self.context.texture.as_ref().unwrap().as_ref().clone() };

        // let world = self.context.get_world();

        self.renderer.draw_texture(
            texture,
            glam::vec2(200.0, 200.0),
            glam::vec2(300.0, 400.0),
            45.0,
            glam::vec3(0.0, 1.0, 0.0),
        );

        // Todo: Encapsulate ir
        window.swap_buffers();

        Ok(())
    }
}

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
}

pub trait Scene {
    fn resize(&mut self) {}

    fn init(&mut self, _context: &mut Context) -> Result<(), ()> {
        Ok(())
    }

    fn input(&mut self, _context: &Context, _events: Events) -> Result<Transition, ()> {
        Ok(Transition::None)
    }

    fn update(&mut self, _context: &Context, _dt: f32) -> Result<Transition, ()> {
        Ok(Transition::None)
    }
}
