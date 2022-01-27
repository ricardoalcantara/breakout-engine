use game_window::{GameLoopState, GameWindow};

use winit::{event::*, event_loop::ControlFlow};

mod game_window;
mod render2d_pipeline;
mod renderer;
mod texture;
mod uniform;
mod vertex;

fn main() {
    env_logger::init();
    let game_window = GameWindow::new();

    game_window.run(move |game_state, control_flow| match game_state {
        GameLoopState::Input(event) => match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        GameLoopState::Update => {}
        GameLoopState::Render(renderer) => {
            match renderer.borrow_mut().render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => renderer.borrow_mut().reconfigure(),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
    })
}
