#![allow(dead_code, unused)]
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use super::{
    engine::EngineTimerView,
    game_window::{GameWindow, GlWindow, ReadOnlyRc},
};
use crate::{
    error::BreakoutResult,
    font::Font,
    gui::group::Group,
    render::{
        opengl::renderer2d::OpenGLRenderer2D,
        renderer::{RenderQuad, RenderText},
    },
};

pub struct UIContext {
    build: HashMap<String, Group>,
    window: ReadOnlyRc<GlWindow>,
    default_font: Font,
}

impl UIContext {
    pub(crate) fn new(window: ReadOnlyRc<GlWindow>) -> BreakoutResult<UIContext> {
        let build = HashMap::new();
        let default_font_byte = include_bytes!("../../assets/Roboto-Regular.ttf");
        let default_font = Font::new_from_memory(default_font_byte)?;

        Ok(UIContext {
            build,
            default_font,
            window,
        })
    }

    pub(crate) fn on_event(&mut self, _event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub(crate) fn render(
        &mut self,
        renderer: &mut RefMut<OpenGLRenderer2D>,
        view_time: &EngineTimerView,
    ) {
        {
            self.default_font
                .build_with_size(25, |image| Ok(renderer.generate_texture(image)?))
                .unwrap();
            renderer.begin_draw(None);
        }

        for (_, build) in &self.build {
            build.render(renderer, view_time, &self.default_font);
        }

        renderer.end_draw();

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
