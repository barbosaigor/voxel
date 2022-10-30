use std::{cell::RefCell, rc::Rc};
use render::renderer;
use super::state;

pub struct App {
    pub rndr: render::renderer::WindowRenderer,
    pub state: Rc<RefCell<state::State>>,
    pub bus: Rc<RefCell<Vec<render::renderer::WinEvent>>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            rndr: renderer::WindowRenderer::new(),
            state: Rc::new(RefCell::new(state::State::new())),
            bus:  Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn run(&mut self) {
        self.rndr.run(self.state.borrow().actors.clone(), self.bus.clone());
    }
}
