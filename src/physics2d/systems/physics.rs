use hecs::With;

use crate::{
    core::{components::Transform2D, game_context::GameContext},
    physics2d::components::physics::{PhysicsBody2D, PhysicsBody2DId, PhysicsBody2DType},
};

pub fn system_update_physics(context: &GameContext, delta: f32) {
    let world = context.world.borrow();
    let mut physics_world = context.physics_world.borrow_mut();

    for (_entity, (physics_body_2d, transform2d)) in
        &mut world.query::<(&mut PhysicsBody2DId, &mut Transform2D)>()
    {
        // match &mut physics_body_2d.physics_body_type {
        //     PhysicsBody2DType::KinematicBody2D(kinamatic_body_2d) => {
        //         let move_by = kinamatic_body_2d.move_by.take();
        //         if let Some(move_by) = move_by {
        //             transform2d.translate(move_by);
        //         }
        //     }
        //     PhysicsBody2DType::StaticBody2D(_) => {}
        //     PhysicsBody2DType::RigidBody2D(_) => todo!(),
        // }
    }

    physics_world.update(delta);

    // TODO start sync
    for (_entity, (transform2d, physics_body_2d_id)) in
        &mut world.query::<(&mut Transform2D, &PhysicsBody2DId)>()
    {
        if let Some(body) = physics_world.get(physics_body_2d_id) {
            transform2d.set_position(body.position);
        }
    }
    // TODO end sync (clean the rest)
}
