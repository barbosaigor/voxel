use super::spawner;
use super::transform;
use super::model;
use specs::{Component, DenseVecStorage};

#[derive(Clone, Component, Debug)]
pub struct Actor {
    pub transform: transform::Transform,
    pub model: model::Model,
}

impl Actor {
    pub fn new(
        transform: transform::Transform,
        obj_path: &str,
        color: Option<[f32; 4]>,
    ) -> Self {
        let m = spawner::load_model(obj_path, color);
        Self {
            transform,
            model: m,
        }
    }

}
