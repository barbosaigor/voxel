use super::transform;
use super::model;
use specs::{Component, DenseVecStorage};

#[derive(Clone, Component, Debug)]
pub struct Actor {
    pub transform: transform::Transform,
    pub model: model::Model,
}

