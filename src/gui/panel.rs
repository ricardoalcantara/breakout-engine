use std::{cell::RefCell, rc::Rc};

use crate::{
    font::Font,
    render::renderer::{RenderQuad, RenderText, Renderer2D},
};

use super::{Constraints, Elements};

pub struct Panel {
    x: Constraints,
    y: Constraints,
    width: Constraints,
    height: Constraints,
    elements: Vec<Elements>,
}

impl Panel {
    pub(crate) fn new() -> Panel {
        Panel {
            x: Constraints::Auto,
            y: Constraints::Auto,
            width: Constraints::Auto,
            height: Constraints::Auto,
            elements: Vec::new(),
        }
    }
    pub fn set_x(&mut self, constraint: Constraints) {
        self.x = constraint;
    }
    pub fn set_y(&mut self, _constraints: Constraints) {
        self.y = _constraints;
    }
    pub fn set_width(&mut self, _constraints: Constraints) {
        self.width = _constraints;
    }
    pub fn set_height(&mut self, _constraints: Constraints) {
        self.height = _constraints;
    }

    pub fn label(&mut self, _value: &str) {
        self.elements.push(Elements::Label(_value.to_string()));
    }
    pub fn button(&mut self, _value: &str) -> bool {
        self.elements.push(Elements::Button(_value.to_string()));
        false
    }

    pub(crate) fn update(&self, panel: &Panel) {}
    pub(crate) fn render(&self, renderer: Rc<RefCell<dyn Renderer2D>>, font: &Font) {
        let mut r = renderer.borrow_mut();
        let element_height = 30;

        let x = match self.x {
            Constraints::Pixel(x) => x,
            Constraints::Auto => 0,
            _ => 0,
        };
        let y = match self.y {
            Constraints::Pixel(y) => y,
            Constraints::Auto => 0,
            _ => 0,
        };
        let width = match self.width {
            Constraints::Pixel(width) => width,
            Constraints::Auto => 200,
            _ => 0,
        };
        let height = match self.height {
            Constraints::Pixel(height) => height,
            Constraints::Auto => (element_height + 10) * self.elements.len() as i32 + 10,
            _ => 0,
        };

        r.draw_quad(RenderQuad {
            size: glam::ivec2(width, height).as_vec2(),
            position: glam::ivec2(x, y).as_vec2(),
            scale: glam::Vec2::ONE,
            rotate: 0.0,
            center_origin: false,
            color: glam::vec4(0.01, 0.01, 0.01, 0.8),
        });
        let element_x = x + 10;
        let mut element_y = y + 10;
        for element in &self.elements {
            match element {
                Elements::Label(text) => {
                    r.draw_text(RenderText {
                        text,
                        font,
                        size: 25,
                        position: glam::ivec2(element_x, element_y).as_vec2(),
                        scale: glam::Vec2::ONE,
                        color: glam::vec4(1.0, 1.0, 1.0, 1.0),
                    });
                    element_y += element_height + 10;
                }
                Elements::Button(text) => {
                    r.draw_quad(RenderQuad {
                        size: glam::ivec2(width - 25, element_height).as_vec2(),
                        position: glam::ivec2(element_x, element_y).as_vec2(),
                        scale: glam::Vec2::ONE,
                        rotate: 0.0,
                        center_origin: false,
                        color: glam::vec4(0.01, 0.01, 0.01, 1.0),
                    });
                    r.draw_text(RenderText {
                        text,
                        font,
                        size: 25,
                        position: glam::ivec2(element_x + 5, element_y + 5).as_vec2(),
                        scale: glam::Vec2::ONE,
                        color: glam::vec4(1.0, 1.0, 1.0, 1.0),
                    });
                    element_y += element_height + 10;
                }
            }
        }
    }
}
