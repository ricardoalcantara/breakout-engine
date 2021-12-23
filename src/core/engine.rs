use super::scene::Scene;
use crate::render::renderer::RenderAPI;
use crate::render::window::MyWindow;
use crate::{
    core::game_state::GameState,
    error::{BreakoutError, BreakoutResult},
};
use log::{error, info};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

struct EngineTimer {
    fps: u32,
    fps_count: u32,
    fps_time: f32,
    frame_time_count: f32,
    frame_time_avg: f32,
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
            fps: 0,
            fps_count: 0,
            fps_time: 0.0,
            frame_time_count: 0.0,
            frame_time_avg: 0.0,
            frame_time_spike_per_seconds: 0.0,
            delta_target,
            time,
        }
    }

    fn update(&mut self) -> f32 {
        let delta = self.time.elapsed().as_secs_f32();
        self.time = std::time::Instant::now();

        self.fps_count += 1;
        self.fps_time += delta;

        if self.fps_time >= 1.0 {
            self.fps = self.fps_count;
            self.fps_count = 0;
            self.fps_time = 0.0;

            self.frame_time_avg = self.frame_time_count / self.fps as f32;
            self.frame_time_count = 0.0;

            info!(
                "
Fps:             {:}
Frame_time_high: {:}
Frame_time_avg:  {:}",
                self.fps, self.frame_time_spike_per_seconds, self.frame_time_avg
            );
            self.frame_time_spike_per_seconds = 0.0;
        }

        delta
    }

    fn wait(&mut self) {
        let frame_time = self.time.elapsed().as_secs_f32();
        self.frame_time_count += frame_time;

        if frame_time > self.frame_time_spike_per_seconds {
            self.frame_time_spike_per_seconds = frame_time;
        }

        if let Some(delta_target) = self.delta_target {
            let remaining_delta = delta_target - frame_time;

            if remaining_delta > 0.0 {
                std::thread::sleep(std::time::Duration::from_secs_f32(remaining_delta));
            }
        }
    }
}

pub enum EngineSettings {
    Title(String),
    WindowSize((u32, u32)),
    Fullscreen(bool),
}

impl EngineSettings {
    pub(crate) fn apply_builder(
        window_builder: winit::window::WindowBuilder,
        engine_settings: Vec<EngineSettings>,
    ) -> winit::window::WindowBuilder {
        let mut window_builder = window_builder;
        for settings in engine_settings {
            match settings {
                EngineSettings::Title(title) => {
                    window_builder = window_builder.with_title(title);
                }
                EngineSettings::WindowSize((width, height)) => {
                    window_builder =
                        window_builder.with_inner_size(PhysicalSize::new(width, height));
                }
                EngineSettings::Fullscreen(set) => {
                    if set {
                        window_builder = window_builder
                            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    }
                }
            }
        }

        window_builder
    }

    pub(crate) fn apply_window(window: &mut MyWindow, engine_settings: Vec<EngineSettings>) {
        for settings in engine_settings {
            match settings {
                EngineSettings::Title(title) => {
                    window.window().set_title(&title);
                }
                EngineSettings::WindowSize((width, height)) => {
                    window
                        .window()
                        .set_inner_size(PhysicalSize::new(width, height));
                }
                EngineSettings::Fullscreen(set) => {
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
    engine_settings: Vec<EngineSettings>,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            engine_settings: Vec::new(),
        }
    }
}
impl EngineBuilder {
    pub fn new() -> EngineBuilder {
        EngineBuilder::default()
    }

    pub fn with_settings(mut self, engine_settings: EngineSettings) -> Self {
        self.engine_settings.push(engine_settings);
        self
    }

    pub fn build(self) -> BreakoutResult<Engine> {
        let mut window_builder = winit::window::WindowBuilder::new();
        window_builder = EngineSettings::apply_builder(window_builder, self.engine_settings);

        let my_window = crate::render::build_window(window_builder, RenderAPI::OpenGL);
        Ok(Engine { window: my_window })
    }
}

pub struct Engine {
    window: MyWindow,
}

impl Engine {
    pub fn run<S>(mut self, state: S) -> BreakoutResult<()>
    where
        S: Scene + 'static,
    {
        let render = self.window.create_renderer_2d()?;
        let event_loop = self.window.event_loop.take().unwrap();
        let mut game_state = GameState::new(state, render, &self.window)?;

        let mut engine_timer = EngineTimer::new();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, .. } => {
                    // TODO: windows_id is not required for the engine
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
                    EngineSettings::apply_window(&mut self.window, settings);

                    match game_state.render(&self.window) {
                        Ok(_) => {}
                        Err(e) => error!("Render Broken {:?}", e),
                    }
                }
                Event::RedrawRequested(_) => {
                    // TODO: windows_id is not required for the engine
                    self.window.window().request_redraw();
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
