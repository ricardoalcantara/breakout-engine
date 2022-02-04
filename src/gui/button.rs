use super::label::Label;
use crate::{
    font::Font,
    render::{renderer::Renderer, RenderQuad},
    shapes::rectangle::Rect,
};
use std::{cell::RefMut, rc::Rc};

pub(crate) enum ButtonType {
    Text(Label),
}

pub(crate) struct Button {
    button_type: ButtonType,
    pub(crate) normal_color: glam::Vec4,
    pub(crate) selected_color: glam::Vec4,
    pub(crate) disabled_color: glam::Vec4,
}

impl Button {
    pub(crate) fn new(button_type: ButtonType, color: glam::Vec4) -> Button {
        Button {
            button_type,
            normal_color: color,
            selected_color: color + glam::vec4(0.05, 0.05, 0.05, 0.0),
            disabled_color: color - glam::vec4(0.05, 0.05, 0.05, 0.0),
        }
    }

    pub(crate) fn draw(&self, renderer: &mut RefMut<Renderer>, rect: Rect, font: &Rc<Font>) {
        renderer.draw_quad(RenderQuad {
            size: rect.size().into(),
            position: rect.position().into(),
            scale: glam::Vec2::ONE,
            rotate: 0.0,
            center_origin: false,
            color: self.normal_color,
        });

        match &self.button_type {
            ButtonType::Text(label) => label.draw(renderer, rect, font),
        }
    }
}
