use actor::transform;
use cgmath::Rotation3;

fn main() {
    let mut app = voxel::app::App::new();

    app.state.borrow_mut().spawn(
        transform::Transform {
            position: cgmath::Vector3 {
                x: 2.5,
                y: 2.5,
                z: 2.5,
            },
            rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
        },
        "/res/cube.obj",
        Some([0.7, 0.3, 0.3, 1.0]),
    );

    app.state.borrow_mut().spawn(
        transform::Transform {
            position: cgmath::Vector3 {
                x: -2.5,
                y: -2.5,
                z: -2.5,
            },
            rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
        },
        "/res/cube.obj",
        Some([0.3, 0.7, 0.3, 1.0]),
    );

    app.state.borrow_mut().spawn(
        transform::Transform {
            position: cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
        },
        "/res/cube.obj",
        Some([0.3, 0.3, 0.7, 1.0]),
    );

    app.run();
}
