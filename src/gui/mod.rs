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
    Label(String),
    Button(String),
}

pub(crate) enum DrawableType {
    Screen(Screen),
    Panel(Panel),
}

pub(crate) struct Label {}
pub(crate) struct Button {}
