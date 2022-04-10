use std::cell::{RefCell, RefMut};

use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Sprite, Transform2D},
        engine::{EngineBuilder, EngineTimer, WindowSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        game_window::{GameLoopState, GameWindow},
        input::{Event, Input},
        scene::{InputHandled, Scene, Transition},
    },
    ecs::world::World,
    error::BreakoutResult,
    math,
    render::{renderer::Renderer, RenderQuad},
    shapes::rectangle::Rect,
};
use rand::Rng;
use winit::event_loop::{ControlFlow, EventLoop};

extern crate log;
extern crate pretty_env_logger;

struct GameObject {
    rect: Rect,
    color: math::Vec4,
}

fn main() {
    pretty_env_logger::init();

    let mut default_camera = math::Mat4::orthographic_rh_gl(0.0, 800.0, 600.0, 0.0, -1.0, 1.0);

    let mut world = World::new();

    let mut rng = rand::thread_rng();

    for _ in 0..20_000 {
        let rect = Rect::new(
            rng.gen_range(0.0..10000.0),
            rng.gen_range(0.0..10000.0),
            rng.gen_range(10.0..30.0),
            rng.gen_range(10.0..30.0),
        );

        let color = math::vec4(rng.gen(), rng.gen(), rng.gen(), 1.0);

        // let entity = world.new_entity();
        // world.add_component_to_entity(entity, GameObject { rect, color });
        world.spawn((GameObject { rect, color },));
    }

    let window_builder = winit::window::WindowBuilder::new();
    let game_window = GameWindow::build(window_builder);
    let mut engine_timer = EngineTimer::new();
    let mut input = Input::new();

    game_window.run(move |game_loop_state, control_flow| match game_loop_state {
        GameLoopState::Input(event) => {
            input.on_event(event);

            if let winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } = event
            {
                *control_flow = ControlFlow::Exit
            }
        }
        GameLoopState::Update => {
            let delta = engine_timer.update();

            let mut direction = math::Vec3::ZERO;

            if input.is_key_pressed(winit::event::VirtualKeyCode::Up)
                || input.is_key_pressed(winit::event::VirtualKeyCode::W)
            {
                direction.y -= 1.0;
            }
            if input.is_key_pressed(winit::event::VirtualKeyCode::Down)
                || input.is_key_pressed(winit::event::VirtualKeyCode::S)
            {
                direction.y += 1.0;
            }
            if input.is_key_pressed(winit::event::VirtualKeyCode::Left)
                || input.is_key_pressed(winit::event::VirtualKeyCode::A)
            {
                direction.x -= 1.0;
            }
            if input.is_key_pressed(winit::event::VirtualKeyCode::Right)
                || input.is_key_pressed(winit::event::VirtualKeyCode::D)
            {
                direction.x += 1.0;
            }

            if direction.length_squared() > 0.0 {
                direction = -direction.normalize();
            }

            let speed = if input.is_key_pressed(winit::event::VirtualKeyCode::Space) {
                500.0
            } else {
                250.0
            };

            default_camera =
                default_camera * math::Mat4::from_translation(direction * speed * delta);
        }
        GameLoopState::Render(renderer) => {
            let mut renderer = renderer.borrow_mut();
            renderer.begin_draw(Some(default_camera));

            for (_e, (game_object,)) in world.query::<(&GameObject,)>() {
                renderer.draw_quad(RenderQuad {
                    size: game_object.rect.size(),
                    position: game_object.rect.position(),
                    scale: glam::Vec2::ONE,
                    rotate: 0.0,
                    center_origin: false,
                    color: game_object.color,
                });
            }

            renderer.end_draw();
        }
        GameLoopState::Wait => engine_timer.wait(),
    });
}
