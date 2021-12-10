use crate::state::{GameState, Scene};
use log::error;
use render::InternalWindow;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct EngineBuilder {}

impl EngineBuilder {
    pub fn new(title: String, width: i32, height: i32) -> EngineBuilder {
        EngineBuilder {}
    }

    pub fn build(&self) -> Result<Engine, ()> {
        let (window, event_loop) = render::build_window(render::RenderAPI::OpenGL);

        // let mut state: State = State::new(&window);
        Ok(Engine { window, event_loop })
    }
}

pub struct Engine {
    window: InternalWindow,
    event_loop: EventLoop<()>,
}

impl Engine {
    pub fn run<S>(self, state: S)
    where
        S: Scene + 'static,
    {
        let window = &self.window;
        let render = render::opengl::render2d::OpenGLRender2D::new(window);
        let mut game_state = GameState::new(state, render);

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.window().id() => {
                    if !game_state.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                game_state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &&mut so w have to dereference it twice
                                game_state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    game_state.update().unwrap();
                    match game_state.render(&self.window) {
                        Ok(_) => {}
                        // Todo: Review WGPU error
                        // // Reconfigure the surface if lost
                        // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        // // The system is out of memory, we should probably quit
                        // Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => error!("{:?}", e),
                    }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    self.window.window().request_redraw();
                }
                _ => {}
            }
        });
    }
}
