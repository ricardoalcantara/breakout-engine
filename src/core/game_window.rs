use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::render::{opengl::renderer2d::OpenGLRenderer2D, renderer::Renderer2D};
use glam::{UVec2, Vec2};
use glutin::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextWrapper, GlRequest, PossiblyCurrent,
};
use log::error;

pub struct ReadOnlyRc<T>(Rc<RefCell<T>>);
pub struct ReadWriteRc<T>(Rc<RefCell<T>>);
impl<T> ReadOnlyRc<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn clone(from: &Self) -> Self {
        ReadOnlyRc(Rc::clone(&from.0))
    }
}

impl<T> Clone for ReadOnlyRc<T> {
    fn clone(&self) -> Self {
        Self::clone(&self)
    }
}

impl<T> ReadWriteRc<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

pub enum GameLoopState<'a> {
    Input(&'a WindowEvent<'a>),
    Update,
    Render(ReadWriteRc<OpenGLRenderer2D>),
    Wait,
}

pub type GlWindow = ContextWrapper<PossiblyCurrent, glutin::window::Window>;

pub struct GameWindow {
    event_loop: Option<EventLoop<()>>,
    window: Rc<RefCell<GlWindow>>,
    renderer: Rc<RefCell<OpenGLRenderer2D>>,
    pub render_size: Option<UVec2>,
}

impl GameWindow {
    pub fn new() -> GameWindow {
        let window_builder = WindowBuilder::new().with_title("A fantastic window!");
        GameWindow::build(window_builder)
    }

    pub fn build(window_builder: WindowBuilder) -> GameWindow {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = glutin::ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        let window = Rc::new(RefCell::new(unsafe { window.make_current() }.unwrap()));
        let renderer = Rc::new(RefCell::new(
            OpenGLRenderer2D::new(&window.borrow()).unwrap(),
        ));

        GameWindow {
            window,
            event_loop: Some(event_loop),
            renderer,
            render_size: None,
        }
    }

    pub fn window(&self) -> ReadOnlyRc<GlWindow> {
        ReadOnlyRc(Rc::clone(&self.window))
    }

    pub fn window_mut(&self) -> ReadWriteRc<GlWindow> {
        ReadWriteRc(Rc::clone(&self.window))
    }

    pub fn renderer(&self) -> ReadOnlyRc<OpenGLRenderer2D> {
        ReadOnlyRc(Rc::clone(&self.renderer))
    }

    pub fn renderer_mut(&self) -> ReadWriteRc<OpenGLRenderer2D> {
        ReadWriteRc(Rc::clone(&self.renderer))
    }

    pub fn run<F>(mut self, mut game_loop: F)
    where
        F: FnMut(GameLoopState, &mut ControlFlow) + 'static,
    {
        let window = self.window.clone();
        let renderer = self.renderer.clone();
        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, .. } => {
                    // if window_id == self.window.window().id() =>
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            renderer.borrow_mut().resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            renderer.borrow_mut().resize(**new_inner_size);
                        }
                        _ => game_loop(GameLoopState::Input(event), control_flow),
                    }
                }
                Event::MainEventsCleared => {
                    game_loop(GameLoopState::Update, control_flow);
                    game_loop(GameLoopState::Render(self.renderer_mut()), control_flow);
                    window.borrow_mut().swap_buffers().unwrap()
                }
                Event::RedrawRequested(_) => {
                    // windows_id is not required for the engine
                    window.borrow_mut().window().request_redraw();
                }
                Event::RedrawEventsCleared => {
                    game_loop(GameLoopState::Wait, control_flow);
                    *control_flow = ControlFlow::Poll;
                }
                _ => {}
            }
        });
    }
}
