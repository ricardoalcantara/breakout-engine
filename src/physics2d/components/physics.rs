pub enum Collision {
    Rect,
}

pub enum PhysicsBody2DType {
    StaticBody2D(StaticBody2D),
    RigidBody2D(RigidBody2D),
    KinematicBody2D(KinematicBody2D),
}

pub struct PhysicsBody2D {
    pub physics_body_type: PhysicsBody2DType,
    pub collisions: Vec<Collision>,
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
