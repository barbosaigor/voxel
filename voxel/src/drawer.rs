use crate::actor;

use super::render;

pub struct Drawer<'a> {
    pub render: &'a render::Render,
}

impl<'a> Drawer<'a> {
    pub fn draw(&self, actors: &Vec<actor::Actor>) {}
}