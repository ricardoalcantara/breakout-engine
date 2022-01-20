use crate::core::{
    components::{AnimatedSprite, Sprite},
    game_context::GameContext,
};

pub fn system_update_animated_sprite(context: &GameContext, delta: f32) {
    let world = &context.world;

    for (_entity, (sprite, animated_sprite)) in
        &mut world.query::<(&mut Sprite, &mut AnimatedSprite)>()
    {
        let mut update = false;

        animated_sprite.total_animation_time += delta;

        if let Some(change_to) = &animated_sprite.change_to {
            animated_sprite.current_animation = change_to.clone();
            // animated_sprite.current_frame = 0;
            // animated_sprite.total_animation_time = 0.0;
            animated_sprite.change_to = None;
            update = true;
        } else if animated_sprite
            .animations
            .contains_key(&animated_sprite.current_animation)
        {
            let animation = &animated_sprite.animations[&animated_sprite.current_animation];
            if animated_sprite.total_animation_time > animation.length {
                animated_sprite.total_animation_time = 0.0;
                animated_sprite.current_frame = 0;
                update = true;
            } else if animated_sprite.current_frame < (animation.key_frames.len() - 1) {
                if animated_sprite.total_animation_time
                    >= animation.key_frames[animated_sprite.current_frame + 1].time
                {
                    animated_sprite.current_frame += 1;
                    update = true;
                }
            }
        }

        if update {
            let frame = &animated_sprite.animations[&animated_sprite.current_animation].key_frames
                [animated_sprite.current_frame];

            if let Some(texture_id) = &frame.texture_id {
                sprite.texture_id = Some(texture_id.clone())
            }
            if let Some(sub_texture) = &frame.sub_texture {
                sprite.sub_texture = Some(sub_texture.clone())
            }

            // if let Some(effect) = &frame.effect {
            //     // Todo: Audio back plz
            //     // audio_manager.play_raw(effect);
            // }
        }
    }
}
