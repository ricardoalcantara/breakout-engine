use std::{cell::RefCell, rc::Rc};

use crate::{
    core::asset_manager::TextureId,
    font::Font,
    render::renderer::{RenderQuad, RenderText, Renderer2D},
    shapes::rectangle::Rect,
};

pub mod group;

pub enum Constraints {
    Auto,
    Center,
    Pixel(i32),
    Relative(f32),
    Aspect(f32),
}

pub(crate) enum Elements {
    Screen(Screen),
    Panel(Panel),
    Texture(Texture),
    Label(Label),
    Button(Button),
}

pub(crate) struct Screen {
    x: Constraints,
    y: Constraints,
}

impl Screen {
    pub(crate) fn new(x: Constraints, y: Constraints) -> Screen {
        Screen { x, y }
    }

    pub(crate) fn position(&self) -> glam::Vec2 {
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

        glam::vec2(x as f32, y as f32)
    }
}

pub(crate) struct Panel {
    x: Constraints,
    y: Constraints,
    width: Constraints,
    height: Constraints,
}

impl Panel {
    pub(crate) fn new(
        x: Constraints,
        y: Constraints,
        width: Constraints,
        height: Constraints,
    ) -> Panel {
        Panel {
            x,
            y,
            width,
            height,
        }
    }

    pub(crate) fn position(&self) -> glam::Vec2 {
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

        glam::vec2(x as f32, y as f32)
    }

    pub(crate) fn size(&self, spacing: i32, elements_count: i32) -> glam::Vec2 {
        let width = match self.width {
            Constraints::Pixel(width) => width,
            Constraints::Auto => 400,
            _ => 0,
        };
        let height = match self.height {
            Constraints::Pixel(height) => height,
            Constraints::Auto => (spacing + 10) * elements_count + 10,
            _ => 0,
        };

        glam::vec2(width as f32, height as f32)
    }

    pub(crate) fn draw(
        &self,
        renderer: &RefCell<dyn Renderer2D>,
        spacing: i32,
        elements_count: i32,
        // font: &Font,
    ) {
        let position = self.position();
        let size = self.size(spacing, elements_count);
        renderer.borrow_mut().draw_quad(RenderQuad {
            size,
            position,
            scale: glam::Vec2::ONE,
            rotate: 0.0,
            center_origin: false,
            color: glam::vec4(0.01, 0.01, 0.01, 0.8),
        });
    }
}

pub(crate) struct Texture {
    texture_id: TextureId,
}

impl Texture {
    pub(crate) fn new(texture_id: TextureId) -> Texture {
        Texture { texture_id }
    }
}

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

    pub(crate) fn draw(&self, renderer: &RefCell<dyn Renderer2D>, rect: Rect, font: &Font) {
        let size = font.measure(&self.text, 25) / 2.0;
        let rect_size: glam::Vec2 = rect.size().into();
        let rect_size = rect_size / 2.0;
        let position: glam::Vec2 = rect.position().into();
        let position = match self.orientation {
            Orientation::Center => position + rect_size - size,
            Orientation::Left => position,
            Orientation::Right => todo!(),
        };
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
