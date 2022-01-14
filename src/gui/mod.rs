use std::{cell::RefCell, rc::Rc};

use crate::{
    core::asset_manager::TextureId,
    font::Font,
    render::renderer::{RenderQuad, RenderText, Renderer2D},
    shapes::rectangle::Rect,
};

use self::{panel::Panel, screen::Screen};

pub mod panel;
pub mod screen;

pub enum Constraints {
    Auto,
    Center,
    Pixel(i32),
    Relative(f32),
    Aspect(f32),
}

pub(crate) enum Elements {
    Texture(Texture),
    Label(Label),
    Button(Button),
}

pub(crate) enum DrawableType {
    Screen(Screen),
    Panel(Panel),
}

pub(crate) struct Texture {
    texture_id: TextureId,
}

impl Texture {
    pub(crate) fn new(texture_id: TextureId) -> Texture {
        Texture { texture_id }
    }
}

pub(crate) struct Label {
    pub(crate) text: String,
    pub(crate) color: glam::Vec4,
}

impl Label {
    pub(crate) fn new(text: String, color: glam::Vec4) -> Label {
        Label { text, color }
    }

    pub(crate) fn draw(&self, renderer: &RefCell<dyn Renderer2D>, rect: Rect, font: &Font) {
        let size = font.measure(&self.text, 25) / 2.0;
        let rect_size: glam::Vec2 = rect.size().into();
        let rect_size = rect_size / 2.0;
        let position: glam::Vec2 = rect.position().into();
        let position = position + rect_size - size;
        renderer.borrow_mut().draw_text(RenderText {
            text: &self.text,
            font,
            size: 25,
            position,
            scale: glam::Vec2::ONE,
            color: glam::vec4(1.0, 1.0, 1.0, 1.0),
        });
    }
}

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

    pub(crate) fn draw(&self, renderer: &RefCell<dyn Renderer2D>, rect: Rect, font: &Font) {
        renderer.borrow_mut().draw_quad(RenderQuad {
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
