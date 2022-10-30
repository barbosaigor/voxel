use std::{rc::Rc, cell::RefCell};

use render::renderer;
use actor::{self, spawner, transform};

fn main() {
    let mut rndr = renderer::WindowRenderer::new();

    let m = spawner::load_model("/res/cube.obj", Some([0.7, 0.3, 0.3, 1.0]));
    let actors = Rc::new(RefCell::new(vec![])); 
    spawner::push_actor(&mut actors.borrow_mut(), m, transform::Transform { 
        position: cgmath::Vector3 { x: 5.0, y: 0.0, z: 0.0 }, 
        rotation: cgmath::Quaternion::from((0.0, 0.0, 0.0, 0.0)),
    });

    let events = Rc::new(RefCell::new(vec![])); 
    rndr.run(actors.clone(), events.clone());
}
