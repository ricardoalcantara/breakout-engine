// pub use winint::event::;
use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode, WindowEvent};

pub use winit::event::MouseButton;
pub type KeyCode = VirtualKeyCode;

#[derive(Debug)]
pub enum Event {
    KeyboardInput {
        key_code: KeyCode,
        is_pressed: bool,
    },
    MouseInput {
        button: MouseButton,
        is_pressed: bool,
    },
    MouseWheel(f32),
    CursorMoved(glam::Vec2),
    ModifiersChanged,
}

pub struct Input {
    key_presseds: Vec<KeyCode>,
    key_releaseds: Vec<KeyCode>,
    mouse_presseds: Vec<MouseButton>,
    mouse_releaseds: Vec<MouseButton>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            key_presseds: Vec::with_capacity(10),
            key_releaseds: Vec::with_capacity(10),
            mouse_presseds: Vec::with_capacity(3),
            mouse_releaseds: Vec::with_capacity(3),
        }
    }

    pub(crate) fn on_event(&mut self, event: &winit::event::WindowEvent) -> Option<Event> {
        let on_event = match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key_code) = input.virtual_keycode {
                    let is_pressed = match input.state {
                        ElementState::Pressed => {
                            if !self.key_presseds.contains(&key_code) {
                                self.key_presseds.push(key_code.clone());
                            }
                            true
                        }
                        ElementState::Released => {
                            if self.key_presseds.contains(&key_code) {
                                let index = self
                                    .key_presseds
                                    .iter()
                                    .position(|x| *x == key_code)
                                    .unwrap();
                                self.key_presseds.remove(index);
                            }
                            if !self.key_releaseds.contains(&key_code) {
                                self.key_releaseds.push(key_code.clone());
                            }
                            false
                        }
                    };
                    Some(Event::KeyboardInput {
                        key_code,
                        is_pressed,
                    })
                } else {
                    None
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = match state {
                    ElementState::Pressed => {
                        if !self.mouse_presseds.contains(button) {
                            self.mouse_presseds.push(button.clone());
                        }
                        true
                    }
                    ElementState::Released => {
                        if self.mouse_presseds.contains(button) {
                            let index = self
                                .mouse_presseds
                                .iter()
                                .position(|x| *x == *button)
                                .unwrap();
                            self.mouse_presseds.remove(index);
                        }
                        if !self.mouse_releaseds.contains(button) {
                            self.mouse_releaseds.push(button.clone());
                        }
                        false
                    }
                };
                Some(Event::MouseInput {
                    button: button.clone(),
                    is_pressed,
                })
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y.clone(),
                    MouseScrollDelta::PixelDelta(d) => d.y.clone() as f32,
                };
                Some(Event::MouseWheel(d))
            }
            WindowEvent::CursorMoved { position, .. } => Some(Event::CursorMoved(glam::vec2(
                position.x as f32,
                position.y as f32,
            ))),
            WindowEvent::ModifiersChanged(_modifiers_state) => Some(Event::ModifiersChanged),
            _ => None,
        };

        on_event
    }

    // pub fn is_action_pressed(&self, action: &str) -> bool {
    //     false
    // }

    // pub fn is_action_released(&self, action: &str) -> bool {
    //     false
    // }

    // pub fn is_action_just_pressed(&self, action: &str) -> bool {
    //     false
    // }

    // pub fn is_action_just_released(&self, action: &str) -> bool {
    //     false
    // }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_presseds.contains(&button)
    }

    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_releaseds.contains(&button)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.key_presseds.contains(&key)
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.key_releaseds.contains(&key)
    }

    pub(crate) fn end_frame(&mut self) {
        self.key_releaseds.clear();
        self.mouse_releaseds.clear();
    }
}
