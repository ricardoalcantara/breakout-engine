use crate::{game_state::GameState, Scene};
use log::{error, info};
use render::window::MyWindow;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

pub struct EngineBuilder {
    title: String,
    width: u32,
    height: u32,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            title: String::from("My Engine"),
            width: 800,
            height: 600,
        }
    }
}

impl EngineBuilder {
    pub fn new() -> EngineBuilder {
        EngineBuilder::default()
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build(self) -> Result<Engine, ()> {
        let window_builder = winit::window::WindowBuilder::new()
            .with_title(self.title)
            .with_inner_size(PhysicalSize::new(self.width, self.height));
        let my_window = render::build_window(window_builder, render::renderer::RenderAPI::OpenGL);
        Ok(Engine { window: my_window })
    }
}

pub struct Engine {
    window: MyWindow,
}

impl Engine {
    pub fn run<S>(mut self, state: S)
    where
        S: Scene + 'static,
    {
        let render = self.window.create_renderer_2d();
        let event_loop = self.window.event_loop.take().unwrap();
        let mut game_state = GameState::new(state, render, &self.window);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.window().id() => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            game_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            game_state.resize(**new_inner_size);
                        }
                        _ => match game_state.input(event) {
                            Ok(handled) => {
                                if !handled {
                                    if let WindowEvent::KeyboardInput {
                                        input:
                                            KeyboardInput {
                                                state: ElementState::Pressed,
                                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                                ..
                                            },
                                        ..
                                    } = event
                                    {
                                        *control_flow = ControlFlow::Exit
                                    }
                                }
                            }
                            Err(e) => error!("Input Broken: {:?}", e),
                        },
                    }
                }
                Event::RedrawRequested(_) => {
                    if let Ok(updated) = game_state.update() {
                        if !updated {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                    match game_state.render(&self.window) {
                        Ok(_) => {}
                        // Todo: Review WGPU error
                        // // Reconfigure the surface if lost
                        // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        // // The system is out of memory, we should probably quit
                        // Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => error!("Render Broken {:?}", e),
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
