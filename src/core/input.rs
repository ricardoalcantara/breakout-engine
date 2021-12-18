// pub use winint::event::;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent};

pub use winit::event::{MouseButton, VirtualKeyCode};

#[derive(Debug)]
pub enum Event {
    KeyboardInput {
        key_code: VirtualKeyCode,
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
    keys_pressed: Vec<VirtualKeyCode>,
    keys_released: Vec<VirtualKeyCode>,
    mouses_pressed: Vec<MouseButton>,
    mouses_released: Vec<MouseButton>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            keys_pressed: Vec::with_capacity(10),
            keys_released: Vec::with_capacity(10),
            mouses_pressed: Vec::with_capacity(3),
            mouses_released: Vec::with_capacity(3),
        }
    }

    pub(crate) fn on_event(&mut self, event: &winit::event::WindowEvent) -> Option<Event> {
        let on_event = match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key_code) = input.virtual_keycode {
                    let is_pressed = match input.state {
                        ElementState::Pressed => {
                            if !self.keys_pressed.contains(&key_code) {
                                self.keys_pressed.push(key_code.clone());
                            }
                            true
                        }
                        ElementState::Released => {
                            if self.keys_pressed.contains(&key_code) {
                                let index = self
                                    .keys_pressed
                                    .iter()
                                    .position(|x| *x == key_code)
                                    .unwrap();
                                self.keys_pressed.remove(index);
                            }
                            if !self.keys_released.contains(&key_code) {
                                self.keys_released.push(key_code.clone());
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
                        if !self.mouses_pressed.contains(button) {
                            self.mouses_pressed.push(button.clone());
                        }
                        true
                    }
                    ElementState::Released => {
                        if self.mouses_pressed.contains(button) {
                            let index = self
                                .mouses_pressed
                                .iter()
                                .position(|x| *x == *button)
                                .unwrap();
                            self.mouses_pressed.remove(index);
                        }
                        if !self.mouses_released.contains(button) {
                            self.mouses_released.push(button.clone());
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
        self.mouses_pressed.contains(&button)
    }

    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        self.mouses_released.contains(&button)
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: VirtualKeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    pub(crate) fn end_frame(&mut self) {
        self.keys_released.clear();
        self.mouses_released.clear();
    }
}
