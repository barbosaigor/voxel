use actor::{self, spawner, transform};
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct State {
    pub actors: Vec<actor::Actor>,
}

impl State {
    pub fn new() -> Self {
        Self { actors: Vec::new() }
    }

    pub fn spawn(&mut self, transform: transform::Transform, obj_path: &str, color: Option<[f32; 4]>) {
        let m = spawner::load_model(obj_path, color);
        self.actors.push(actor::Actor { transform, model: m });
    }
}
