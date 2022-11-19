use specs::{prelude::*, rayon::ThreadPool};
use std::sync::Arc;

pub trait Scene {
    fn setup(&self, world: &mut World);
    fn setup_systems(&mut self, world: &mut World, thread_pool: Arc<ThreadPool>) -> Dispatcher;
}
