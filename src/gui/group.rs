use std::cell::RefCell;

use crate::{
    core::engine::EngineTimerView,
    font::Font,
    render::renderer::{RenderQuad, RenderText, Renderer2D},
    shapes::rectangle::Rect,
};

use super::{
    button::ButtonType,
    label::{Label, Orientation},
    Button, Constraints, Elements, Panel, Screen,
};

pub struct Group {
    x: Constraints,
    y: Constraints,
    width: Constraints,
    height: Constraints,
    elements: Vec<Elements>,
}

impl Group {
    pub(crate) fn new() -> Group {
        Group {
            x: Constraints::Auto,
            y: Constraints::Auto,
            width: Constraints::Auto,
            height: Constraints::Auto,
            elements: Vec::new(),
        }
    }

    pub fn screen(&mut self, x: Constraints, y: Constraints) {
        let screen = Screen::new(x, y);
        self.elements.push(Elements::Screen(screen));
    }

    pub fn panel(
        &mut self,
        x: Constraints,
        y: Constraints,
        width: Constraints,
        height: Constraints,
    ) {
        let panel = Panel::new(x, y, width, height);
        self.elements.push(Elements::Panel(panel));
    }

    pub fn print_diagnostics(&mut self) {
        self.elements.push(Elements::Diagnostics);
    }

    pub fn label(&mut self, value: &str) {
        let label = Label::new(
            value.to_string(),
            glam::vec4(1.0, 1.0, 1.0, 1.0),
            Orientation::Left,
        );
        self.elements.push(Elements::Label(label));
    }

    pub fn button(&mut self, value: &str) -> bool {
        let label = Label::new(
            value.to_string(),
            glam::vec4(1.0, 1.0, 1.0, 1.0),
            Orientation::Center,
        );
        let button = Button::new(ButtonType::Text(label), glam::vec4(0.01, 0.01, 0.01, 1.0));
        self.elements.push(Elements::Button(button));
        false
    }

    pub(crate) fn render(
        &self,
        renderer: &RefCell<dyn Renderer2D>,
        view_time: &EngineTimerView,
        font: &Font,
    ) {
        let spacing = 30;
        let padding = 10;
        let elements_count = self.elements.len() as i32;
        let (position, size) = match &self.elements.first() {
            Some(element) => match &element {
                Elements::Panel(panel) => {
                    panel.draw(renderer, spacing, elements_count);
                    (panel.position(), panel.size(spacing, elements_count))
                }
                Elements::Screen(screen) => (screen.position(), glam::Vec2::ZERO),
                _ => (glam::vec2(10.0, 10.0), glam::Vec2::ZERO),
            },
            None => (glam::vec2(10.0, 10.0), glam::Vec2::ZERO),
        };

        let mut element_position = position + glam::vec2(padding as f32, 0.0);
        for element in &self.elements {
            match &element {
                Elements::Diagnostics => {
                    Label::new(
                        String::from(format!("Fps: {}", view_time.fps)),
                        glam::vec4(1.0, 1.0, 1.0, 1.0),
                        Orientation::Left,
                    )
                    .draw(
                        renderer,
                        Rect::from_position_size(
                            element_position.into(),
                            glam::vec2(size.x - padding as f32 * 2.0, spacing as f32).into(),
                        ),
                        font,
                    );
                    element_position.y += spacing as f32 + 10.0;
                    Label::new(
                        String::from(format!("Frame time: {}", view_time.frame_time_avg)),
                        glam::vec4(1.0, 1.0, 1.0, 1.0),
                        Orientation::Left,
                    )
                    .draw(
                        renderer,
                        Rect::from_position_size(
                            element_position.into(),
                            glam::vec2(size.x - padding as f32 * 2.0, spacing as f32).into(),
                        ),
                        font,
                    );
                    element_position.y += spacing as f32 + 10.0;
                }
                Elements::Label(label) => {
                    label.draw(
                        renderer,
                        Rect::from_position_size(
                            element_position.into(),
                            glam::vec2(size.x - padding as f32 * 2.0, spacing as f32).into(),
                        ),
                        font,
                    );
                    element_position.y += spacing as f32 + 10.0;
                }
                Elements::Button(button) => {
                    button.draw(
                        renderer,
                        Rect::from_position_size(
                            element_position.into(),
                            glam::vec2(size.x - padding as f32 * 2.0, spacing as f32 + 10.0).into(),
                        ),
                        font,
                    );
                    element_position.y += spacing as f32 + 20.0;
                }
                Elements::Texture(_texture) => todo!(),
                Elements::Panel(_) | Elements::Screen(_) => {}
            }
        }
    }
}
