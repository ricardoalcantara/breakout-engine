use std::{cell::RefCell, rc::Rc};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::renderer::Renderer;

pub enum GameLoopState<'a> {
    Input(&'a winit::event::WindowEvent<'a>),
    Update,
    Render(Rc<RefCell<Renderer>>),
}

pub struct GameWindow {
    event_loop: EventLoop<()>,
    window: Window,
    renderer: Rc<RefCell<Renderer>>,
}

impl GameWindow {
    pub fn new() -> GameWindow {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = Rc::new(RefCell::new(pollster::block_on(Renderer::new(&window))));
        GameWindow {
            event_loop,
            window,
            renderer,
        }
    }

    pub fn run<F>(self, mut game_loop: F)
    where
        F: FnMut(GameLoopState, &mut ControlFlow) + 'static,
    {
        let event_loop = self.event_loop;
        let window = self.window;

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.renderer.borrow_mut().resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            self.renderer.borrow_mut().resize(**new_inner_size);
                        }
                        _ => game_loop(GameLoopState::Input(event), control_flow),
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    game_loop(GameLoopState::Update, control_flow);
                    game_loop(GameLoopState::Render(self.renderer.clone()), control_flow);
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
