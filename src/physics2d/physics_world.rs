use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use super::components::physics::{PhysicsBody2D, PhysicsBody2DId, PhysicsBody2DType};

pub struct PhysicsWorld {
    bodies: HashMap<PhysicsBody2DId, RefCell<PhysicsBody2D>>,
    next_id: u32,
}

impl PhysicsWorld {
    pub fn new() -> PhysicsWorld {
        PhysicsWorld {
            bodies: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, physics_body_2d: PhysicsBody2D) -> PhysicsBody2DId {
        let physics_body_2d_id = PhysicsBody2DId(self.next_id);
        self.next_id += 1;

        self.bodies
            .insert(physics_body_2d_id.clone(), RefCell::new(physics_body_2d));

        physics_body_2d_id
    }

    pub fn update(&mut self, _delta: f32) {
        for (_, body) in &self.bodies {
            let mut body_a = body.borrow_mut();

            match &mut body_a.physics_body_type {
                PhysicsBody2DType::KinematicBody2D(k) => {
                    let move_by = k.move_by.take();
                    let mut is_colliding_x = false;
                    let mut is_colliding_y = false;

                    if let Some(move_by) = move_by {
                        if move_by.length_squared() > 0.0 {
                            for (_, other) in &self.bodies {
                                if std::ptr::eq(body, other) {
                                    continue;
                                }
                                let body_b = other.borrow();

                                let is_colliding = body_a.is_colliding(&body_b, &move_by);

                                is_colliding_x = is_colliding_x || is_colliding.0;
                                is_colliding_y = is_colliding_y || is_colliding.1;

                                if is_colliding_x && is_colliding_y {
                                    break;
                                }
                            }

                            match (is_colliding_x, is_colliding_y) {
                                (false, false) => body_a.position += move_by,
                                (true, false) => body_a.position += glam::vec2(0.0, move_by.y),
                                (false, true) => body_a.position += glam::vec2(move_by.x, 0.0),
                                _ => (),
                            }
                        }
                    }
                }
                PhysicsBody2DType::StaticBody2D(_) => {}
                PhysicsBody2DType::RigidBody2D(_) => todo!(),
            }
        }
    }

    pub fn get(&self, id: &PhysicsBody2DId) -> Option<Ref<PhysicsBody2D>> {
        if let Some(body) = self.bodies.get(id) {
            Some(body.borrow())
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: &PhysicsBody2DId) -> Option<RefMut<PhysicsBody2D>> {
        if let Some(body) = self.bodies.get_mut(id) {
            Some(body.borrow_mut())
        } else {
            None
        }
    }
}
