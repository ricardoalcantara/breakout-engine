#![allow(dead_code, unused)]
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::BreakoutResult,
    font::Font,
    gui::group::Group,
    render::{
        renderer::{RenderQuad, RenderText, Renderer2D},
        window::MyWindow,
    },
};

pub struct UIContext {
    build: HashMap<String, Group>,
    pub(crate) window_size: glam::UVec2,
    default_font: Font,
}

impl UIContext {
    pub(crate) fn new(window: &MyWindow) -> BreakoutResult<UIContext> {
        let build = HashMap::new();
        let default_font_byte = include_bytes!("../../assets/Roboto-Regular.ttf");
        let default_font = Font::new_from_memory(default_font_byte)?;

        Ok(UIContext {
            build,
            default_font,
            window_size: {
                let size = window.window().inner_size();
                glam::uvec2(size.width, size.height)
            },
        })
    }

    pub(crate) fn on_event(&mut self, _event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub(crate) fn render(&mut self, _renderer: &RefCell<dyn Renderer2D>) {
        {
            let mut r = _renderer.borrow_mut();
            self.default_font
                .build_with_size(25, |image| Ok(r.generate_texture(image)?))
                .unwrap();
            r.begin_draw(None);
        }

        for (_, build) in &self.build {
            build.render(_renderer, &self.default_font);
        }

        {
            let mut r = _renderer.borrow_mut();
            r.end_draw();
        }

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
