use breakout_engine::{
    core::{
        asset_manager::AssetManager,
        components::{Sprite, Transform2D},
        engine::{EngineBuilder, WindowSettings},
        engine_context::EngineContext,
        game_context::GameContext,
        input::{Event, Input},
        scene::{InputHandled, Scene, Transition},
    },
    error::BreakoutResult,
    math,
    shapes::rectangle::Rect,
};
use rand::Rng;

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

struct GameObject {}

struct MainState {
    quad: StaticQuadTree<GameObject>,
}

impl MainState {
    fn new() -> Self {
        Self {
            quad: StaticQuadTree::new(Rect::new(0.0, 0.0, 1000.0, 1000.0)),
        }
    }
}
impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
        _engine: &mut EngineContext,
    ) -> BreakoutResult {
        let mut rng = rand::thread_rng();
        let mut world = _context.get_world_mut();

        for _ in 0..1_000 {
            let go = GameObject {};
            let rect = Rect::new(
                rng.gen_range(0.0..1000.0),
                rng.gen_range(0.0..1000.0),
                rng.gen_range(0.0..100.0),
                rng.gen_range(0.0..100.0),
            );

            world.spawn((
                rect.clone(),
                Sprite {
                    color: Some(math::vec4(rng.gen(), rng.gen(), rng.gen(), 1.0)),
                    ..Default::default()
                },
                Transform2D::from_position_rotation_scale(rect.position(), 0.0, rect.size()),
            ));

            self.quad.insert(go, rect)
        }

        Ok(())
    }

    fn input(
        &mut self,
        _event: Event,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<InputHandled> {
        Ok(InputHandled::None)
    }

    fn update(
        &mut self,
        _dt: f32,
        _input: &mut Input,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<Transition> {
        Ok(Transition::None)
    }
}

fn main() -> BreakoutResult {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_window_settings(WindowSettings::Title(String::from("QuadTree")))
        .with_window_settings(WindowSettings::WindowSize((800, 600)))
        .build()?
        .run(MainState::new())
}
