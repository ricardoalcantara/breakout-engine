use crate::shapes::rectangle::Rect;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct PhysicsBody2DId(pub(crate) u32);

pub struct Collision {
    pub shapes: Vec<Shapes>,
}

impl Collision {
    pub fn from_rect(rect: Rect) -> Collision {
        Collision {
            shapes: vec![Shapes::Rect(rect)],
        }
    }
}

pub enum Shapes {
    Rect(Rect),
}

pub enum PhysicsBody2DType {
    StaticBody2D(StaticBody2D),
    RigidBody2D(RigidBody2D),
    KinematicBody2D(KinematicBody2D),
}

impl PhysicsBody2DType {
    pub fn kinematic_body_2d() -> PhysicsBody2DType {
        PhysicsBody2DType::KinematicBody2D(KinematicBody2D { move_by: None })
    }

    pub fn static_body_2d() -> PhysicsBody2DType {
        PhysicsBody2DType::StaticBody2D(StaticBody2D {})
    }
}

pub struct PhysicsBody2D {
    pub physics_body_type: PhysicsBody2DType,
    pub collision: Collision,
    pub position: glam::Vec2,
}

impl PhysicsBody2D {
    pub fn is_colliding(&self, other: &PhysicsBody2D, move_by: &glam::Vec2) -> (bool, bool) {
        let mut is_colliding_x = false;
        let mut is_colliding_y = false;

        for shape_a in &self.collision.shapes {
            let Shapes::Rect(rect_a) = shape_a;
            let rect_a = rect_a.translated(self.position.into());
            let rect_a_x = rect_a.translated(glam::vec2(move_by.x, 0.0).into());
            let rect_a_y = rect_a.translated(glam::vec2(0.0, move_by.y).into());

            for shape_b in &other.collision.shapes {
                let Shapes::Rect(rect_b) = shape_b;
                let rect_b = rect_b.translated(other.position.into());

                is_colliding_x = is_colliding_x || rect_a_x.intersects(&rect_b);
                is_colliding_y = is_colliding_y || rect_a_y.intersects(&rect_b);
            }
        }

        return (is_colliding_x, is_colliding_y);
    }
}

// Area2D provide detection and influence
pub struct Area2D {}

// A static body is one that is not moved by the physics engine.
pub struct StaticBody2D {}

// This implements simulated 2D physics. You do not control a RigidBody2D directly
pub struct RigidBody2D {}

// A body that provides collision detection, but no physics
pub struct KinematicBody2D {
    pub move_by: Option<glam::Vec2>,
}
