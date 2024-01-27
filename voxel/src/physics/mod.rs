use specs::{Component, DenseVecStorage};

#[derive(Clone, Component, Debug)]
pub struct RigidBodyComponent {
    pub rigid_body: rapier3d::dynamics::RigidBodyHandle,
}

impl RigidBodyComponent {
    pub fn new(rigid_body: rapier3d::prelude::RigidBodyHandle) -> Self {
        Self { rigid_body }
    }
}

impl Default for RigidBodyComponent {
    fn default() -> Self {
        Self {
            rigid_body: Default::default(),
        }
    }
}
