use actor::transform;

fn main() {
    let mut app = voxel::app::App::new();

    app.state.borrow_mut().spawn(
        transform::Transform {
            position: cgmath::Vector3 {
                x: 5.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: cgmath::Quaternion::from((0.0, 0.0, 0.0, 0.0)),
        },
        "/res/cube.obj",
        Some([0.7, 0.3, 0.3, 1.0]),
    );

    app.state.borrow_mut().spawn(
        transform::Transform {
            position: cgmath::Vector3 {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
            rotation: cgmath::Quaternion::from((0.0, 0.0, 0.0, 0.0)),
        },
        "/res/cube.obj",
        Some([0.7, 0.3, 0.3, 1.0]),
    );

    app.state.borrow_mut().spawn(
        transform::Transform {
            position: cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            rotation: cgmath::Quaternion::from((0.0, 0.0, 0.0, 0.0)),
        },
        "/res/cube.obj",
        Some([0.7, 0.3, 0.3, 1.0]),
    );

    app.run();
}
