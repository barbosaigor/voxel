use super::{drawer, state};
use specs::{prelude::*, rayon::ThreadPool};
use std::sync::Arc;

pub trait Scene {
    fn setup(&mut self, global_state: &mut state::State);
    fn setup_systems<'a, 'b>(&mut self, dispatcher_builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>;
}
