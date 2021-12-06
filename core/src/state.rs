use crate::Events;

pub trait State {
    fn resize(&mut self) {}

    fn input(&mut self, _events: Events) -> Result<(), ()> {
        Ok(())
    }

    fn update(&mut self, _dt: f32) -> Result<(), ()> {
        Ok(())
    }
}
