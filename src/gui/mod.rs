use self::{button::Button, label::Label, panel::Panel, screen::Screen, texture::Texture};

mod button;
pub mod group;
mod label;
mod panel;
mod screen;
mod texture;

pub enum Constraints {
    Auto,
    Center,
    Pixel(i32),
    Relative(f32),
    Aspect(f32),
}

pub(crate) enum Elements {
    Diagnostics,
    Screen(Screen),
    Panel(Panel),
    Texture(Texture),
    Label(Label),
    Button(Button),
}
