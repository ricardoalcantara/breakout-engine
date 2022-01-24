use std::cell::RefMut;

use crate::{
    core::{asset_manager::AssetManager, components::Label, game_context::GameContext},
    error::BreakoutResult,
    font::Font,
    render::opengl::renderer2d::OpenGLRenderer2D,
};

pub fn system_render_font_texture(
    context: &GameContext,
    asset_manager: &mut AssetManager,
    renderer: &RefMut<OpenGLRenderer2D>,
    default_font: &mut Font,
) -> BreakoutResult {
    let world = &context.world;

    for (_id, label) in world.query::<&Label>().iter() {
        if let Some(font_id) = &label.font_id {
            asset_manager.get_font_with_size(&font_id, label.size, |image| {
                Ok(renderer.generate_texture(image)?)
            })?;
        } else {
            default_font
                .build_with_size(label.size, |image| Ok(renderer.generate_texture(image)?))?;
        };
    }

    Ok(())
}
