use std::{cell::RefCell, rc::Rc};

use crate::{
    core::{asset_manager::AssetManager, components::Label, game_context::GameContext},
    error::BreakoutResult,
    font::Font,
    render::renderer::Renderer2D,
};

pub fn system_font_update(
    context: &GameContext,
    asset_manager: &mut AssetManager,
    renderer: Rc<RefCell<dyn Renderer2D>>,
    default_font: &mut Font,
) -> BreakoutResult {
    let world = &context.world;

    for (_id, label) in world.query::<&Label>().iter() {
        if let Some(font_id) = &label.font_id {
            asset_manager.get_font_with_size(&font_id, label.size, |image| {
                Ok(renderer.borrow().generate_texture(image)?)
            })?;
        } else {
            default_font.build_with_size(label.size, |image| {
                Ok(renderer.borrow().generate_texture(image)?)
            })?;
        };
    }

    Ok(())
}
