use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{
    core::{
        asset_manager::AssetManager,
        components::{Camera2D, Label, Sprite, Transform2D},
        game_context::GameContext,
        game_window::GameWindow,
    },
    error::BreakoutResult,
    font::Font,
    render::{
        renderer::{RenderText, RenderVertices, Renderer2D},
        vertex::{
            TEXTURE_COORDS, TEXTURE_COORDS_FLIPPED_X, TEXTURE_COORDS_FLIPPED_X_Y,
            TEXTURE_COORDS_FLIPPED_Y,
        },
    },
};

pub fn system_render_sprite<R>(
    context: &GameContext,
    asset_manager: &AssetManager,
    renderer: &mut R,
    window: Ref<GameWindow>,
    default_font: &Font,
) -> BreakoutResult
where
    R: Renderer2D,
{
    let world = &context.world;

    renderer.clear_color(context.clear_color);

    let camera_projection = if let Some((_id, (camera, transform))) =
        world.query::<(&Camera2D, &Transform2D)>().iter().next()
    {
        let window_size = {
            let size = window.window.window().inner_size();
            glam::uvec2(size.width, size.height)
        };

        Some(camera.get_view_matrix(
            window.render_size.as_ref().unwrap_or(&window_size),
            &window_size,
            &transform.position,
        ))
    } else {
        None
    };

    renderer.begin_draw(camera_projection);
    for (_id, (sprite, transform)) in world.query::<(&mut Sprite, &mut Transform2D)>().iter() {
        if !sprite.visible {
            continue;
        }
        if let Some(texture_id) = &sprite.texture_id {
            let texture = asset_manager.get_texture(&texture_id);
            if transform.dirt {
                sprite.update_vertices(
                    transform.position,
                    transform.rotate,
                    transform.scale,
                    texture.size().as_vec2(),
                );
                transform.dirt = false;
            }

            if let Some(sub_texture) = &mut sprite.sub_texture {
                if sub_texture.texture_coords.is_none() {
                    sub_texture.texture_size =
                        glam::vec2(texture.width as f32, texture.height as f32);
                    sub_texture.update_texture_coords()
                }
            }

            let texture_coords = if let Some(sub_texture) = &sprite.sub_texture {
                sub_texture.texture_coords.as_ref().unwrap()
            } else {
                match (sprite.flip_x, sprite.flip_y) {
                    (true, false) => &TEXTURE_COORDS_FLIPPED_X,
                    (false, true) => &TEXTURE_COORDS_FLIPPED_Y,
                    (true, true) => &TEXTURE_COORDS_FLIPPED_X_Y,
                    _ => &TEXTURE_COORDS, // (false, false)
                }
            };

            renderer.draw_vertices(RenderVertices {
                texture: Some(texture),
                vertices: sprite.get_vertices(),
                color: sprite.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
                texture_coords: texture_coords,
            });
        } else {
            if transform.dirt {
                sprite.update_vertices(
                    transform.position,
                    transform.rotate,
                    transform.scale,
                    glam::Vec2::ONE,
                );
                transform.dirt = false;
            }
            renderer.draw_vertices(RenderVertices {
                texture: None,
                vertices: sprite.get_vertices(),
                color: sprite.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
                texture_coords: &TEXTURE_COORDS,
            });
        };
    }

    for (_id, (label, _transform)) in world.query::<(&mut Label, &Transform2D)>().iter() {
        if !label.visible {
            continue;
        }

        let font = if let Some(font_id) = &label.font_id {
            asset_manager.get_font(&font_id)
        } else {
            default_font
        };

        renderer.draw_text(RenderText {
            text: &label.text,
            font,
            size: label.size,
            position: _transform.position,
            scale: _transform.scale,
            color: label.color.unwrap_or(glam::vec4(1.0, 1.0, 1.0, 1.0)),
        });
    }
    renderer.end_draw();

    Ok(())
}
