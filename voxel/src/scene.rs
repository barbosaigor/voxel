use super::{drawer, state};
use specs::{prelude::*, rayon::ThreadPool};
use std::sync::Arc;

pub trait Scene {
    fn setup(&mut self, global_state: &mut state::State);
    fn setup_systems(&mut self, world: &mut World, thread_pool: Arc<ThreadPool>) -> Dispatcher;
}
