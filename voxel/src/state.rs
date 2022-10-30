use std::{rc::Rc, cell::RefCell};

use actor::{self, spawner, transform};

pub struct State {
    pub actors: Rc<RefCell<Vec<actor::Actor>>>,
}

impl State {
    pub fn new() -> Self {
        Self { actors: Rc::new(RefCell::new(vec![])) }
    }

    pub fn spawn(&mut self, transform: transform::Transform, obj_path: &str, color: Option<[f32; 4]>) {
        let m = spawner::load_model(obj_path, color);
        self.actors.borrow_mut().push(actor::Actor { transform, model: m });
    }
}
