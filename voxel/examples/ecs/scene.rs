use cgmath::Rotation3;
use specs::{prelude::*, rayon::ThreadPool};
use std::{sync::Arc, time::SystemTime};
use voxel::{self, fly_camera, camera, event, scene, state, actor::{self, transform}};

pub struct Scene {}

impl Scene {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl scene::Scene for Scene {
    fn setup_systems(&mut self, mut world: &mut World, thread_pool: Arc<ThreadPool>) -> Dispatcher {
        let mut dispatcher = specs::DispatcherBuilder::new()
            .with_pool(thread_pool)
            .with(AutoMovementSys {}, "auto_movement_sys", &[])
            .with(CameraSys {}, "camera_sys", &[])
            .build();

        dispatcher.setup(&mut world);

        dispatcher
    }

    fn setup(&mut self, global_state: &mut state::State) {
        global_state.world.register::<Vel>();
        global_state.world.register::<actor::Actor>();

        global_state.world.insert(event::WinEvents::default());

        let c = camera::Camera::default(global_state.render.size.0, global_state.render.size.1);
        let controller = fly_camera::FlyCameraController::new(0.2);
        global_state
            .world
            .insert(camera::CameraBundle::from_camera(c, controller));

        // Spawn entities
        {
            global_state
                .world
                .create_entity()
                .with(Vel(0.0005))
                .with(actor::Actor::new(
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

            global_state
                .world
                .create_entity()
                .with(Vel(-0.0005))
                .with(actor::Actor::new(
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

            global_state
                .world
                .create_entity()
                .with(Vel(0.05))
                .with(actor::Actor::new(
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

struct CameraSys {

}

impl<'a> System<'a> for CameraSys {
    type SystemData = (
        Write<'a, camera::CameraBundle>,
        Read<'a, event::WinEvents>,
    );

    fn run(&mut self, (mut camera, events): Self::SystemData) {
        log::trace!("running autoMovement system");
        use event::WinEvent;

        for ev in &events.events {
            match ev {
                WinEvent::Space => {
                    camera.controller.is_up_pressed = true;
                }
                WinEvent::Up => {
                    camera.controller.is_forward_pressed = true;
                }
                WinEvent::Left => {
                    camera.controller.is_left_pressed = true;
                }
                WinEvent::Down => {
                    camera.controller.is_backward_pressed = true;
                }
                WinEvent::Right => {
                    camera.controller.is_right_pressed = true;
                }
                _ => {}
            }
        }
    }
}
