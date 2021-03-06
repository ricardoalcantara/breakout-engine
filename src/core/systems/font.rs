use std::{cell::RefMut, rc::Rc};

use crate::{
    core::{asset_manager::AssetManager, components::Label, game_context::GameContext},
    error::BreakoutResult,
    font::Font,
    render::{renderer::Renderer, texture::Texture},
};

pub fn system_render_font_texture(
    context: &GameContext,
    asset_manager: &mut AssetManager,
    renderer: &RefMut<Renderer>,
    default_font: &mut Rc<Font>,
) -> BreakoutResult {
    let world = context.world.borrow();

    for (_id, label) in world.query::<&Label>().iter() {
        if let Some(font_id) = &label.font_id {
            asset_manager.get_font_with_size(&font_id, label.size, |image| {
                Texture::from_dynamic_image(image, renderer.device(), renderer.queue())
            })?;
        } else {
            (*Rc::get_mut(&mut *default_font).unwrap()).build_with_size(label.size, |image| {
                Texture::from_dynamic_image(image, renderer.device(), renderer.queue())
            })?;
        };
    }

    Ok(())
}
