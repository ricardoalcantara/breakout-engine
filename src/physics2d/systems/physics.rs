use crate::{
    core::{components::Transform2D, game_context::GameContext},
    physics2d::components::physics::PhysicsBody2DId,
};

pub fn system_update_physics(context: &GameContext, delta: f32) {
    let world = context.world.borrow();
    let mut physics_world = context.physics_world.borrow_mut();

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
