use glutin::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextWrapper, GlRequest, PossiblyCurrent,
};
use log::error;

use crate::render::{opengl::renderer2d::OpenGLRenderer2D, renderer::Renderer2D};

pub enum GameLoopState<'a> {
    Input(&'a WindowEvent<'a>),
    Update,
    Render(&'a mut OpenGLRenderer2D),
    Wait,
}

pub struct GameWindow {
    event_loop: EventLoop<()>,
    window: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
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

        let window = unsafe { window.make_current() }.unwrap();

        GameWindow { window, event_loop }
    }

    pub fn run<F>(self, mut game_loop: F)
    where
        F: FnMut(GameLoopState) + 'static,
    {
        let event_loop = self.event_loop;
        let window = self.window;

        let mut renderer = OpenGLRenderer2D::new(&window).unwrap();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, .. } => {
                    // if window_id == self.window.window().id() =>
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            renderer.resize(**new_inner_size);
                        }
                        _ => game_loop(GameLoopState::Input(event)),
                    }
                }
                Event::MainEventsCleared => {
                    game_loop(GameLoopState::Update);
                    game_loop(GameLoopState::Render(&mut renderer));
                }
                Event::RedrawRequested(_) => {
                    // windows_id is not required for the engine
                    window.swap_buffers().unwrap();
                }
                Event::RedrawEventsCleared => {
                    game_loop(GameLoopState::Wait);
                    *control_flow = ControlFlow::Poll;
                }
                _ => {}
            }
        });
    }
}
