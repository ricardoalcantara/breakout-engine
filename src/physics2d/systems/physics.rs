use crate::{
    core::{components::Transform2D, game_context::GameContext},
    physics2d::components::physics::{PhysicsBody2D, PhysicsBody2DType},
};

pub fn system_update_physics(context: &GameContext, _delta: f32) {
    let world = &context.world;
    let physics_world = &context.physics_world;

    for (_entity, (physics_body_2d, transform2d)) in
        &mut world.query::<(&mut PhysicsBody2D, &mut Transform2D)>()
    {
        match &mut physics_body_2d.physics_body_type {
            PhysicsBody2DType::KinematicBody2D(kinamatic_body_2d) => {
                let move_by = kinamatic_body_2d.move_by.take();
                if let Some(move_by) = move_by {
                    transform2d.translate(move_by);
                }
            }
            PhysicsBody2DType::StaticBody2D(_) => {}
            PhysicsBody2DType::RigidBody2D(_) => todo!(),
        }
    }
}
