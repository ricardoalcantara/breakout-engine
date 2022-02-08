#![allow(dead_code, unused)]
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use super::engine::EngineTimerView;
use crate::{error::BreakoutResult, font::Font, gui::group::Group, render::renderer::Renderer};

pub struct UIContext {
    build: HashMap<String, Group>,
    default_font: Rc<Font>,
}

impl UIContext {
    pub(crate) fn new() -> BreakoutResult<UIContext> {
        let build = HashMap::new();
        let default_font_byte = include_bytes!("../../assets/Roboto-Regular.ttf");
        let default_font = Rc::new(Font::new_from_memory(default_font_byte)?);

        Ok(UIContext {
            build,
            default_font,
        })
    }

    pub(crate) fn on_event(&mut self, _event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub(crate) fn render(&mut self, renderer: &mut RefMut<Renderer>, view_time: &EngineTimerView) {
        // TODO WGPU pending
        // self.default_font
        //     .build_with_size(25, |image| Ok(renderer.generate_texture(image)?))
        //     .unwrap();

        // TODO: Set UI Camera later
        // TODO WGPU pending
        // renderer.begin_draw(None);

        for (_, build) in &self.build {
            build.render(renderer, view_time, &self.default_font);
        }

        // TODO WGPU pending
        // renderer.end_draw();

        self.build.clear();
    }

    pub fn begin<F>(&mut self, title: &str, mut f: F)
    where
        F: FnMut(&mut Group),
    {
        let mut group = if self.build.contains_key(title) {
            if let Some(group) = self.build.remove(title) {
                group
            } else {
                panic!("That's not right")
            }
        } else {
            Group::new()
        };

        f(&mut group);
        self.build.insert(String::from(title), group);
    }
}
