use std::cell::RefCell;
use std::rc::Rc;

use super::scene::Scene;
use crate::render::renderer::{RenderAPI, Renderer2D};
use crate::render::window::MyWindow;
use crate::{core::game_state::GameState, error::BreakoutResult};
use hecs::Ref;
use log::{error, info};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

pub struct EngineTimerView {
    pub delta: f32,
    pub fps: u32,
    pub frame_time_avg: f32,
    pub frame_time_spike_per_seconds: f32,
}

struct EngineTimer {
    delta: f32,
    fps: u32,
    fps_count: u32,
    fps_time: f32,
    frame_time_count: f32,
    frame_time_avg: f32,
    frame_time_spike: f32,
    frame_time_spike_per_seconds: f32,
    delta_target: Option<f32>,
    time: std::time::Instant,
}

impl EngineTimer {
    fn new() -> EngineTimer {
        let frame_target = std::env::var("FPS_LOCK")
            .unwrap_or(String::from("0.0"))
            .parse::<f32>()
            .unwrap_or(0.0);

        let delta_target = if frame_target > 0.0 {
            Some(1.0 / frame_target)
        } else {
            None
        };

        let time = std::time::Instant::now();
        EngineTimer {
            delta: 0.0,
            fps: 0,
            fps_count: 0,
            fps_time: 0.0,
            frame_time_count: 0.0,
            frame_time_avg: 0.0,
            frame_time_spike: 0.0,
            frame_time_spike_per_seconds: 0.0,
            delta_target,
            time,
        }
    }

    fn view_time(&self) -> EngineTimerView {
        EngineTimerView {
            delta: self.delta,
            fps: self.fps,
            frame_time_avg: self.frame_time_avg,
            frame_time_spike_per_seconds: self.frame_time_spike_per_seconds,
        }
    }

    fn update(&mut self) -> f32 {
        self.delta = self.time.elapsed().as_secs_f32();
        self.time = std::time::Instant::now();

        self.fps_count += 1;
        self.fps_time += self.delta;

        if self.fps_time >= 1.0 {
            self.fps = self.fps_count;
            self.fps_count = 0;
            self.fps_time = 0.0;

            self.frame_time_avg = self.frame_time_count / self.fps as f32;
            self.frame_time_count = 0.0;
            self.frame_time_spike_per_seconds = self.frame_time_spike;
            info!(
                "
Fps:             {:}
Frame_time_high: {:}
Frame_time_avg:  {:}",
                self.fps, self.frame_time_spike_per_seconds, self.frame_time_avg
            );
            self.frame_time_spike = 0.0;
        }

        self.delta
    }

    fn wait(&mut self) {
        let frame_time = self.time.elapsed().as_secs_f32();
        self.frame_time_count += frame_time;

        if frame_time > self.frame_time_spike {
            self.frame_time_spike = frame_time;
        }

        if let Some(delta_target) = self.delta_target {
            let remaining_delta = delta_target - frame_time;

            if remaining_delta > 0.0 {
                std::thread::sleep(std::time::Duration::from_secs_f32(remaining_delta));
            }
        }
    }
}

pub enum RenderSettings {
    DisplaySize((u32, u32)),
}

impl RenderSettings {
    pub(crate) fn apply_window(window: &mut MyWindow, render_settings: Vec<RenderSettings>) {
        for settings in render_settings {
            match settings {
                RenderSettings::DisplaySize((width, height)) => {
                    window.render_size = Some(glam::uvec2(width, height))
                }
            }
        }
    }
}

pub enum WindowSettings {
    Title(String),
    WindowSize((u32, u32)),
    Fullscreen(bool),
}

impl WindowSettings {
    pub(crate) fn apply_builder(
        window_builder: winit::window::WindowBuilder,
        engine_settings: Vec<WindowSettings>,
    ) -> winit::window::WindowBuilder {
        let mut window_builder = window_builder;
        for settings in engine_settings {
            match settings {
                WindowSettings::Title(title) => {
                    window_builder = window_builder.with_title(title);
                }
                WindowSettings::WindowSize((width, height)) => {
                    window_builder =
                        window_builder.with_inner_size(PhysicalSize::new(width, height));
                }
                WindowSettings::Fullscreen(set) => {
                    if set {
                        window_builder = window_builder
                            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    }
                }
            }
        }

        window_builder
    }

    pub(crate) fn apply_window(window: &mut MyWindow, engine_settings: Vec<WindowSettings>) {
        for settings in engine_settings {
            match settings {
                WindowSettings::Title(title) => {
                    window.window().set_title(&title);
                }
                WindowSettings::WindowSize((width, height)) => {
                    window
                        .window()
                        .set_inner_size(PhysicalSize::new(width, height));
                }
                WindowSettings::Fullscreen(set) => {
                    if set {
                        window
                            .window()
                            .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    } else {
                        window.window().set_fullscreen(None);
                    }
                }
            }
        }
    }
}

pub struct EngineBuilder {
    window_settings: Vec<WindowSettings>,
    render_settings: Vec<RenderSettings>,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            window_settings: Vec::new(),
            render_settings: Vec::new(),
        }
    }
}
impl EngineBuilder {
    pub fn new() -> EngineBuilder {
        EngineBuilder::default()
    }

    pub fn with_window_settings(mut self, window_settings: WindowSettings) -> Self {
        self.window_settings.push(window_settings);
        self
    }

    pub fn with_render_settings(mut self, engine_settings: RenderSettings) -> Self {
        self.render_settings.push(engine_settings);
        self
    }

    pub fn build(self) -> BreakoutResult<Engine> {
        let mut window_builder = winit::window::WindowBuilder::new();
        window_builder = WindowSettings::apply_builder(window_builder, self.window_settings);

        let mut my_window = crate::render::build_window(window_builder, RenderAPI::OpenGL);
        RenderSettings::apply_window(&mut my_window, self.render_settings);

        let engine = Engine {
            window: Rc::new(RefCell::new(my_window)),
        };
        Ok(engine)
    }
}

pub struct Engine {
    window: Rc<RefCell<MyWindow>>,
}

impl Engine {
    pub fn run<S>(self, state: S) -> BreakoutResult<()>
    where
        S: Scene + 'static,
    {
        let render = self.window.borrow().create_renderer_2d()?;

        let event_loop = self.window.borrow_mut().event_loop.take().unwrap();
        let mut game_state = GameState::new(state, render, Rc::clone(&self.window))?;

        let mut engine_timer = EngineTimer::new();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, .. } => {
                    // if window_id == self.window.window().id() =>
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            game_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            game_state.resize(**new_inner_size);
                        }
                        _ => match game_state.input(event) {
                            Ok(handled) => {
                                if !handled {
                                    if let WindowEvent::KeyboardInput {
                                        input:
                                            KeyboardInput {
                                                state: ElementState::Pressed,
                                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                                ..
                                            },
                                        ..
                                    } = event
                                    {
                                        *control_flow = ControlFlow::Exit
                                    }
                                }
                            }
                            Err(e) => error!("Input Broken: {:?}", e),
                        },
                    }
                }
                Event::MainEventsCleared => {
                    let delta = engine_timer.update();
                    if let Ok(updated) = game_state.update(delta) {
                        if !updated {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                    let settings = game_state.take_settings();
                    WindowSettings::apply_window(&mut self.window.borrow_mut(), settings);

                    match game_state.render(engine_timer.view_time()) {
                        Ok(_) => {}
                        Err(e) => error!("Render Broken {:?}", e),
                    }
                }
                Event::RedrawRequested(_) => {
                    // windows_id is not required for the engine
                    self.window.borrow().window().request_redraw();
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.

                    // https://github.com/rust-windowing/winit/blob/master/examples/control_flow.rs
                    engine_timer.wait();
                    *control_flow = ControlFlow::Poll;
                }
                _ => {}
            }
        });
    }
}
