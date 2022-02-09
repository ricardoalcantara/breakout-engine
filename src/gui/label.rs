use crate::{
    font::Font,
    render::{renderer::Renderer, RenderQuad, RenderText},
    shapes::rectangle::Rect,
};
use std::{cell::RefMut, rc::Rc};

pub(crate) enum Orientation {
    Center,
    Left,
    Right,
}

pub(crate) struct Label {
    pub(crate) text: String,
    pub(crate) color: glam::Vec4,
    pub(crate) orientation: Orientation,
}

impl Label {
    pub(crate) fn new(text: String, color: glam::Vec4, orientation: Orientation) -> Label {
        Label {
            text,
            color,
            orientation,
        }
    }

    pub(crate) fn draw(&self, renderer: &mut RefMut<Renderer>, rect: Rect, font: &Rc<Font>) {
        let size = font.measure(&self.text, 25) / 2.0;
        let rect_size: glam::Vec2 = rect.size().into();
        let rect_size = rect_size / 2.0;
        let position: glam::Vec2 = rect.position().into();
        let position = match self.orientation {
            Orientation::Center => position + rect_size - size,
            Orientation::Left => position,
            Orientation::Right => todo!(),
        };
        renderer.draw_text(RenderText {
            text: &self.text,
            font: font.clone(),
            size: 25,
            position,
            scale: glam::Vec2::ONE,
            color: glam::vec4(1.0, 1.0, 1.0, 1.0),
        });
    }
}
