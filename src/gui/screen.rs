use std::{cell::RefCell, rc::Rc};

use crate::render::renderer::Renderer2D;

use super::{Button, ButtonType, Elements, Label};

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
        let label = Label::new(_value.to_string(), glam::vec4(1.0, 1.0, 1.0, 1.0));
        self.elements.push(Elements::Label(label));
    }
    pub fn button(&mut self, _value: &str, position: glam::IVec2) -> bool {
        let label = Label::new(_value.to_string(), glam::vec4(1.0, 1.0, 1.0, 1.0));
        let button = Button::new(ButtonType::Text(label), glam::vec4(0.01, 0.01, 0.01, 1.0));
        self.elements.push(Elements::Button(button));
        false
    }

    pub(crate) fn update(&self, screen: &Screen) {}
    pub(crate) fn render(&self, renderer: &RefCell<dyn Renderer2D>) {}
}
