use render::{renderer, spawner, actor};

fn main() {
    let mut rndr = renderer::WindowRenderer::new();

    // rndr.rendr
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .push_model("/res/cube.obj", Some([0.7, 0.3, 0.3, 1.0]));
    let mut state = State{
        actors: vec![],
    };
    let m = spawner::load_model("/res/cube.obj", Some([0.7, 0.3, 0.3, 1.0]));
    spawner::push_actor(&mut state.actors, m, render::transform::Transform { 
        position: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 }, 
        rotation: cgmath::Quaternion::from((0.0, 0.0, 0.0, 0.0)),
    });

    rndr.run(state.actors);
}

struct State {
    actors: Vec::<actor::Actor>,
}
