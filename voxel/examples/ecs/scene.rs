use cgmath::Rotation3;
use specs::{prelude::*, rayon::ThreadPool};
use std::{sync::Arc, time::SystemTime};
use voxel::actor::{self, spawner, transform};
use voxel::event;
use voxel::scene;

pub struct Scene {}

impl Scene {
    pub fn new() -> Self {
        Self {}
    }
}

impl scene::Scene for Scene {
    fn setup_systems(&mut self, mut world: &mut World, thread_pool: Arc<ThreadPool>) -> Dispatcher {
        let mut dispatcher = specs::DispatcherBuilder::new()
            .with_pool(thread_pool)
            .with(AutoMovementSys {}, "auto_movement", &[])
            .build();

        dispatcher.setup(&mut world);

        dispatcher
    }

    fn setup(&self, world: &mut World) {
        world.register::<Vel>();
        world.register::<actor::Actor>();

        world.insert(event::WinEvents::default());

        // Spawn entities
        {
            world
                .create_entity()
                .with(Vel(0.0005))
                .with(build_actor(
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
                ))
                .build();

            world
                .create_entity()
                .with(Vel(-0.0005))
                .with(build_actor(
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
                ))
                .build();

            world
                .create_entity()
                .with(Vel(0.05))
                .with(build_actor(
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
                ))
                .build();
        }
    }
}

pub fn build_actor(
    transform: transform::Transform,
    obj_path: &str,
    color: Option<[f32; 4]>,
) -> actor::Actor {
    let m = spawner::load_model(obj_path, color);
    actor::Actor {
        transform,
        model: m,
    }
}

#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

struct AutoMovementSys {}

impl<'a> System<'a> for AutoMovementSys {
    type SystemData = (ReadStorage<'a, Vel>, WriteStorage<'a, actor::Actor>);

    fn run(&mut self, (vels, mut actors): Self::SystemData) {
        log::trace!("running autoMovement system");

        for (vel, actor) in (&vels, &mut actors).join() {
            actor.transform.position.x += vel.0;
            actor.transform.position.y +=
                5.0 * f32::sin(SystemTime::now().elapsed().unwrap().as_millis() as f32);
        }
    }
}
