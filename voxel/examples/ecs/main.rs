use actor::transform;
use cgmath::Rotation3;
use specs::prelude::*;

fn main() {
    let mut app = voxel::app::App::new();

    let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();

    dispatcher.setup(&mut app.world);

    app.world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    app.world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    app.world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    // This entity does not have `Vel`, so it won't be dispatched.
    app.world.create_entity().with(Pos(2.0)).build();

    // This dispatches all the systems in parallel (but blocking).
    dispatcher.dispatch(&app.world);

    app.state.borrow_mut().spawn(
        transform::Transform {
            position: cgmath::Vector3 {
                x: 2.5,
                y: 2.5,
                z: 2.5,
            },
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
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
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
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
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
        },
        "/res/cube.obj",
        Some([0.3, 0.3, 0.7, 1.0]),
    );

    app.run();
}

#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        // You could also use `par_join()` to get a rayon `ParallelIterator`.
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
    }
}
