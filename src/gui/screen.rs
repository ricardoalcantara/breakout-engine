use std::{cell::RefCell, rc::Rc};

use crate::render::renderer::Renderer2D;

use super::Elements;

#[derive(Default)]
pub struct Screen {
    elements: Vec<Elements>,
}

impl Screen {
    pub(crate) fn new() -> Screen {
        Screen {
            elements: Vec::new(),
        }
    }

    pub fn label(&mut self, _value: &str, position: glam::IVec2) {
        self.elements.push(Elements::Label(_value.to_string()));
    }
    pub fn button(&mut self, _value: &str, position: glam::IVec2) -> bool {
        self.elements.push(Elements::Button(_value.to_string()));
        false
    }

    pub(crate) fn update(&self, screen: &Screen) {}
    pub(crate) fn render(&self, renderer: Rc<RefCell<dyn Renderer2D>>) {
        // let mut r = renderer.borrow_mut();

        // r.draw_quad(RenderQuad {
        //     size: glam::vec2(20.0, 20.0),
        //     position: glam::vec2(100.0, 100.0),
        //     scale: glam::Vec2::ONE,
        //     rotate: 0.0,
        //     center_origin: false,
        //     color: glam::vec4(1.0, 1.0, 1.0, 1.0),
        // });
    }
}
