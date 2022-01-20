use crate::{
    core::{asset_manager::AssetManager, engine_context::EngineContext, game_context::GameContext},
    error::BreakoutResult,
};

use super::{
    input::{Event, Input},
    ui_context::UIContext,
};

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
}

pub enum InputHandled {
    None,
    Captured,
    Transition(Transition),
}

pub trait Scene {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
        _engine: &mut EngineContext,
    ) -> BreakoutResult {
        Ok(())
    }

    fn ui(&mut self, _context: &mut GameContext, _ui: &mut UIContext) {}

    fn input(
        &mut self,
        _event: Event,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<InputHandled> {
        Ok(InputHandled::None)
    }

    fn update(
        &mut self,
        _dt: f32,
        _input: &mut Input,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> BreakoutResult<Transition> {
        Ok(Transition::None)
    }
}
