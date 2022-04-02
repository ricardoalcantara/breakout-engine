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
    error::BreakoutResult,
    math,
    render::{renderer::Renderer, RenderQuad},
    shapes::rectangle::Rect,
};
use rand::Rng;
use winit::event_loop::{ControlFlow, EventLoop};

extern crate log;
extern crate pretty_env_logger;

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }
    fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            // The `downcast_mut` type here is changed to `RefCell<Vec<Option<ComponentType>>`
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                // add a `get_mut` here. Once again `get_mut` bypasses
                // `RefCell`'s runtime checks if accessing through a `&mut` reference.
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);

        // Here we create a `RefCell` before inserting into `component_vecs`
        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                // Here we use `borrow_mut`.
                // If this `RefCell` is already borrowed from this will panic.
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    //https://stackoverflow.com/a/56700760/8378479
    fn spawn(&mut self, _b: impl ComponentBundle) -> usize {
        0
    }

    fn query<T: QueryParameters>(&mut self) {
        0
    }
}

trait ComponentBundle {}

impl<A> ComponentBundle for (A,) {}
impl<A, B> ComponentBundle for (A, B) {}

trait QueryParameters {}
impl<A> QueryParameters for (A,) {}
impl<A, B> QueryParameters for (A, B) {}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

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
        world.spawn((GameObject { rect, color }, true));
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

            world.query::<(&GameObject,)>();
            let data = world.borrow_component_vec::<GameObject>().unwrap();
            for game_object in data.iter().filter_map(|f| f.as_ref()) {
                // for item in quad_tree.search(&Rect::new(0.0, 0.0, 800.0, 600.0)) {
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
