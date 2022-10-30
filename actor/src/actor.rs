use super::transform;
use super::model;

#[derive(Clone)]
pub struct Actor {
    pub transform: transform::Transform,
    pub model: model::Model,
}

