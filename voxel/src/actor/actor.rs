use super::model;
use super::spawner;
use super::transform;
use specs::{Component, DenseVecStorage};

#[derive(Clone, Component, Debug)]
pub struct Actor {
    pub name: String,
    pub transform: transform::Transform,
    pub model: model::Model,
}

impl Actor {
    pub fn new(transform: transform::Transform, obj_path: &str, color: Option<[f32; 4]>) -> Self {
        Self::new_with_name(String::new(), transform, obj_path, color)
    }

    pub fn new_with_name(
        name: String,
        transform: transform::Transform,
        obj_path: &str,
        color: Option<[f32; 4]>,
    ) -> Self {
        let m = spawner::load_model(obj_path, color);
        Self {
            name,
            transform,
            model: m,
        }
    }
}
