#![allow(dead_code)]
#![allow(unused)]
use glutin::{ContextWrapper, PossiblyCurrent};
use state::State;
use winit::{event::*, event_loop::ControlFlow};

mod helper;
mod shader;
mod sprite_renderer;
mod state;
mod texture;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    let window: ContextWrapper<PossiblyCurrent, glutin::window::Window> =
        unsafe { window.make_current() }.unwrap();

    let mut state: State = State::new(&window);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, .. } => {
                if !state.input(event) {
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
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {
                        window.swap_buffers().unwrap();
                    }
                    // // Reconfigure the surface if lost
                    // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // // The system is out of memory, we should probably quit
                    // Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::RedrawEventsCleared => {}
            _ => {}
        }
    });
}
