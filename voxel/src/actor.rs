use super::transform;
use render::model;

#[allow(dead_code)]
pub struct Actor {
    pub transform: transform::Transform,
    pub model: model::Model,
}

