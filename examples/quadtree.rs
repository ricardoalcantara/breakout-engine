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

const MAX_DEPTH: u32 = 8;

struct StaticQuadTree<T> {
    depth: u32,
    size: Rect,
    child: [Rect; 4],
    sub_quad: [Option<Box<StaticQuadTree<T>>>; 4],
    items: Option<Vec<(Rect, T)>>,
}

impl<T> StaticQuadTree<T> {
    fn new_with_depth(size: Rect, depth: u32) -> StaticQuadTree<T> {
        let mut static_quad_tree = StaticQuadTree::new(size);
        static_quad_tree.depth = depth;
        static_quad_tree
    }

    fn new(size: Rect) -> StaticQuadTree<T> {
        let mut static_quad_tree = StaticQuadTree {
            depth: 0,
            size,
            child: [Rect::default(); 4],
            sub_quad: [None, None, None, None],
            items: Some(vec![]),
        };

        static_quad_tree.resize(size);
        static_quad_tree
    }

    fn resize(&mut self, size: Rect) {
        self.clear();
        self.size = size;

        let position = self.size.position();
        let size: math::Vec2 = self.size.size() / 2.0;
        self.child = [
            Rect::from_position_size(position, size),
            Rect::from_position_size(math::vec2(position.x + size.x, position.y), size),
            Rect::from_position_size(math::vec2(position.x, position.y + size.y), size),
            Rect::from_position_size(position + size, size),
        ];
    }

    fn clear(&mut self) {
        if let Some(items) = &mut self.items {
            items.clear();
        }

        for sub_quad in &mut self.sub_quad {
            *sub_quad = None;
        }
    }

    fn size(&self) -> usize {
        let mut count = if let Some(items) = &self.items {
            items.len()
        } else {
            0
        };

        for sub_quad in &self.sub_quad {
            if let Some(sub_quad) = sub_quad {
                count += sub_quad.size();
            }
        }

        count
    }

    fn insert(&mut self, item: T, item_size: Rect) {
        for (i, child) in self.child.iter().enumerate() {
            if child.contains(&item_size) {
                if self.depth + 1 < MAX_DEPTH {
                    if self.sub_quad[i].is_none() {
                        self.sub_quad[i] = Some(Box::new(StaticQuadTree::new_with_depth(
                            child.clone(),
                            self.depth + 1,
                        )))
                    }

                    if let Some(sub_quad) = &mut self.sub_quad[i] {
                        sub_quad.insert(item, item_size);
                        return;
                    }
                }
            }
        }

        let it = (item_size, item);
        if let Some(items) = &mut self.items {
            items.push(it);
        } else {
            self.items = Some(vec![it])
        }
    }

    fn search(&self, area: &Rect) -> Vec<&T> {
        let mut list_items = Vec::new();

        if let Some(items) = &self.items {
            for (rect, item) in items {
                if area.intersects(rect) {
                    list_items.push(item)
                }
            }
        }

        for (i, sub_quad) in self.sub_quad.iter().enumerate() {
            if let Some(sub_quad) = sub_quad {
                if area.contains(&self.child[i]) {
                    list_items.extend_from_slice(&sub_quad.items());
                } else if self.child[i].intersects(area) {
                    list_items.extend_from_slice(&sub_quad.search(area));
                }
            }
        }

        list_items
    }

    fn items(&self) -> Vec<&T> {
        let mut list_items = Vec::new();

        if let Some(items) = &self.items {
            for (_, item) in items {
                list_items.push(item)
            }
        }

        for sub_quad in &self.sub_quad {
            if let Some(sub_quad) = sub_quad {
                list_items.extend_from_slice(&sub_quad.items());
            }
        }

        list_items
    }
}

struct GameObject {
    rect: Rect,
    color: math::Vec4,
}

fn main() {
    pretty_env_logger::init();

    let mut default_camera = math::Mat4::orthographic_rh_gl(0.0, 800.0, 600.0, 0.0, -1.0, 1.0);

    let mut quad_tree: StaticQuadTree<GameObject> =
        StaticQuadTree::new(Rect::new(0.0, 0.0, 800.0, 600.0));

    let mut rng = rand::thread_rng();

    for _ in 0..20_000 {
        let rect = Rect::new(
            rng.gen_range(0.0..10000.0),
            rng.gen_range(0.0..10000.0),
            rng.gen_range(10.0..30.0),
            rng.gen_range(10.0..30.0),
        );

        let color = math::vec4(rng.gen(), rng.gen(), rng.gen(), 1.0);

        quad_tree.insert(GameObject { rect, color }, rect)
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

            for item in quad_tree.items() {
                // for item in quad_tree.search(&Rect::new(0.0, 0.0, 800.0, 600.0)) {
                renderer.draw_quad(RenderQuad {
                    size: item.rect.size(),
                    position: item.rect.position(),
                    scale: glam::Vec2::ONE,
                    rotate: 0.0,
                    center_origin: false,
                    color: item.color,
                });
            }
            renderer.end_draw();
        }
        GameLoopState::Wait => engine_timer.wait(),
    });
}
