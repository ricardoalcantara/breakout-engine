use std::{any::Any, cell::RefCell};

use super::world::World;

pub trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

pub trait ComponentBundle {
    fn spawn_in_world(self, world: &mut World, entity: usize);
}

impl<A: 'static> ComponentBundle for (A,) {
    fn spawn_in_world(self, world: &mut World, entity: usize) {
        world.add_component_to_entity(entity, self.0)
    }
}
impl<A: 'static, B: 'static> ComponentBundle for (A, B) {
    fn spawn_in_world(self, world: &mut World, entity: usize) {
        world.add_component_to_entity(entity, self.0);
        world.add_component_to_entity(entity, self.1);
    }
}
impl<A: 'static, B: 'static, C: 'static> ComponentBundle for (A, B, C) {
    fn spawn_in_world(self, world: &mut World, entity: usize) {
        world.add_component_to_entity(entity, self.0);
        world.add_component_to_entity(entity, self.1);
        world.add_component_to_entity(entity, self.2);
    }
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}
