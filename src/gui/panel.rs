use std::{cell::RefCell, rc::Rc};

use crate::{
    font::Font,
    render::renderer::{RenderQuad, RenderText, Renderer2D},
    shapes::rectangle::Rect,
};

use super::{Button, ButtonType, Constraints, Elements, Label};

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
        let label = Label::new(_value.to_string(), glam::vec4(1.0, 1.0, 1.0, 1.0));
        self.elements.push(Elements::Label(label));
    }
    pub fn button(&mut self, _value: &str) -> bool {
        let label = Label::new(_value.to_string(), glam::vec4(1.0, 1.0, 1.0, 1.0));
        let button = Button::new(ButtonType::Text(label), glam::vec4(0.01, 0.01, 0.01, 1.0));
        self.elements.push(Elements::Button(button));
        false
    }

    pub(crate) fn update(&self, panel: &Panel) {}
    pub(crate) fn render(&self, renderer: &RefCell<dyn Renderer2D>, font: &Font) {
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

        renderer.borrow_mut().draw_quad(RenderQuad {
            size: glam::ivec2(width, height).as_vec2(),
            position: glam::ivec2(x, y).as_vec2(),
            scale: glam::Vec2::ONE,
            rotate: 0.0,
            center_origin: false,
            color: glam::vec4(0.01, 0.01, 0.01, 0.8),
        });

        let mut element_position = glam::vec2(x as f32 + 10.0, y as f32 + 10.0);
        for element in &self.elements {
            match &element {
                Elements::Label(label) => {
                    label.draw(
                        renderer,
                        Rect::from_position_size(
                            element_position.into(),
                            glam::vec2(width as f32 - 25.0, element_height as f32).into(),
                        ),
                        font,
                    );
                    element_position.y += element_height as f32 + 10.0;
                }
                Elements::Button(button) => {
                    button.draw(
                        renderer,
                        Rect::from_position_size(
                            element_position.into(),
                            glam::vec2(width as f32 - 25.0, element_height as f32 + 10.0).into(),
                        ),
                        font,
                    );
                    element_position.y += element_height as f32 + 20.0;
                }
                &Elements::Texture(_texture) => todo!(),
            }
        }
    }
}
