use super::game_state::GameState;
use super::game_window::{GameLoopState, GameWindow};
use super::scene::Scene;
use crate::error::BreakoutResult;

use log::{error, info};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
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
    pub(crate) fn apply_window(game_window: &mut GameWindow, render_settings: Vec<RenderSettings>) {
        for settings in render_settings {
            match settings {
                RenderSettings::DisplaySize((width, height)) => {
                    game_window.set_render_size(glam::uvec2(width, height));
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

    // TODO: apply_window isn't working anymore
    pub(crate) fn apply_window(game_window: &mut GameWindow, engine_settings: Vec<WindowSettings>) {
        let window = game_window.window();
        let window = window.borrow();
        for settings in engine_settings {
            match settings {
                WindowSettings::Title(title) => {
                    window.set_title(&title);
                }
                WindowSettings::WindowSize((width, height)) => {
                    window.set_inner_size(PhysicalSize::new(width, height));
                }
                WindowSettings::Fullscreen(set) => {
                    if set {
                        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    } else {
                        window.set_fullscreen(None);
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

    pub fn build<'a>(self) -> BreakoutResult<Engine<'a>> {
        let mut window_builder = winit::window::WindowBuilder::new();
        window_builder = WindowSettings::apply_builder(window_builder, self.window_settings);

        let mut game_window = GameWindow::build(window_builder);

        RenderSettings::apply_window(&mut game_window, self.render_settings);

        let engine = Engine { game_window };
        Ok(engine)
    }
}

pub struct Engine<'a> {
    game_window: GameWindow<'a>,
}

impl<'a> Engine<'static> {
    pub fn run<S>(self, state: S) -> BreakoutResult<()>
    where
        S: Scene + 'static,
    {
        let mut engine_timer = EngineTimer::new();
        let mut game_state = GameState::new(state, self.game_window.renderer())?;

        self.game_window.run(move |game_loop_state, control_flow| {
            match game_loop_state {
                GameLoopState::Input(event) => match game_state.input(event) {
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
                GameLoopState::Update => {
                    let delta = engine_timer.update();
                    if let Ok(updated) = game_state.update(delta) {
                        if !updated {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                }
                GameLoopState::Render(renderer) => {
                    // let settings = game_state.take_settings();
                    // WindowSettings::apply_window(&mut self.window.borrow_mut(), settings);

                    match game_state.render(renderer, engine_timer.view_time()) {
                        Ok(_) => {}
                        Err(e) => error!("Render Broken {:?}", e),
                    }
                }
                GameLoopState::Wait => engine_timer.wait(),
            }
        });

        Ok(())
    }
}
