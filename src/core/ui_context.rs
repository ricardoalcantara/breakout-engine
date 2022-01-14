#![allow(dead_code, unused)]
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::BreakoutResult,
    font::Font,
    gui::{panel::Panel, screen::Screen, DrawableType},
    render::{
        renderer::{RenderQuad, RenderText, Renderer2D},
        window::MyWindow,
    },
};

pub struct UIContext {
    build: HashMap<String, DrawableType>,
    // state: HashMap<String, DrawableType>,
    pub(crate) window_size: glam::UVec2,
    default_font: Font,
}

impl UIContext {
    pub(crate) fn new(window: &MyWindow) -> BreakoutResult<UIContext> {
        let build = HashMap::new();
        // let state = HashMap::new();
        let default_font_byte = include_bytes!("../../assets/Roboto-Regular.ttf");
        let default_font = Font::new_from_memory(default_font_byte)?;

        Ok(UIContext {
            build,
            // state,
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
        // self.end_frame();
        {
            let mut r = _renderer.borrow_mut();
            self.default_font
                .build_with_size(25, |image| Ok(r.generate_texture(image)?))
                .unwrap();
            r.begin_draw(None);
        }

        for (_, state) in &self.build {
            match &state {
                DrawableType::Panel(panel) => panel.render(_renderer, &self.default_font),
                DrawableType::Screen(screen) => screen.render(_renderer),
            }
        }

        {
            let mut r = _renderer.borrow_mut();
            r.end_draw();
        }

        self.build.clear();
    }

    pub fn begin_panel<F>(&mut self, title: &str, mut f: F)
    where
        F: FnMut(&mut Panel),
    {
        let mut panel = if self.build.contains_key(title) {
            if let Some(screen) = self.build.remove(title) {
                if let DrawableType::Panel(screen) = screen {
                    screen
                } else {
                    panic!("That's not right")
                }
            } else {
                panic!("That's not right")
            }
        } else {
            Panel::new()
        };

        f(&mut panel);
        self.build
            .insert(String::from(title), DrawableType::Panel(panel));
    }

    pub fn begin_screen<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Screen),
    {
        let mut screen = if self.build.contains_key("screen") {
            if let Some(screen) = self.build.remove("screen") {
                if let DrawableType::Screen(screen) = screen {
                    screen
                } else {
                    panic!("That's not right")
                }
            } else {
                panic!("That's not right")
            }
        } else {
            Screen::new()
        };

        f(&mut screen);
        self.build
            .insert(String::from("screen"), DrawableType::Screen(screen));
    }

    // fn end_frame(&mut self) {
    //     let mut old_state = HashMap::new();
    //     for (key, value) in self.state.drain() {
    //         if self.build.contains_key(&key) {
    //             old_state.insert(key, value);
    //         }
    //     }

    //     for (key, value) in self.build.drain() {
    //         if old_state.contains_key(&key) {
    //             match &value {
    //                 DrawableType::Panel(panel) => {
    //                     if let DrawableType::Panel(old_panel) = &old_state[&key] {
    //                         panel.update(old_panel);
    //                     }
    //                 }
    //                 DrawableType::Screen(screen) => {
    //                     if let DrawableType::Screen(old_screen) = &old_state[&key] {
    //                         screen.update(old_screen);
    //                     }
    //                 }
    //             };
    //         }

    //         self.state.insert(key, value);
    //     }
    // }
}
