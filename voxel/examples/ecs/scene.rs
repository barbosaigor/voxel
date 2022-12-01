use cgmath::Rotation3;
use specs::{prelude::*, rayon::ThreadPool};
use std::{time::SystemTime};
use voxel::{
    self,
    actor::{self, transform},
    camera::{self, CameraController},
    delta_time::DeltaTime,
    event::{self, WinEvent},
    fly_camera, scene, state,
};

pub struct Scene {}

impl Scene {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl scene::Scene for Scene {
    fn setup_systems<'a, 'b>(
        &mut self,
        dispatcher_builder: DispatcherBuilder<'a, 'b>,
    ) -> DispatcherBuilder<'a, 'b> {
        dispatcher_builder
            .with(AutoMovementSys {}, "auto_movement_sys", &[])
            .with(CameraSys {}, "camera_sys", &[])
    }

    fn setup(&mut self, global_state: &mut state::State) {
        global_state.world.register::<Vel>();
        global_state.world.register::<actor::Actor>();

        global_state.world.insert(event::WinEvents::default());

        let camera = camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection = camera::Projection::new(
            global_state.render.size.0,
            global_state.render.size.1,
            cgmath::Deg(45.0),
            0.1,
            100.0,
        );
        let controller = fly_camera::FlyCameraController::new(4.0, 0.4);
        global_state.world.insert(camera::CameraBundle::from_camera(
            camera, projection, controller,
        ));

        global_state.world.insert(DeltaTime::default());

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
                .with(Vel(0.005))
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
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Vel>,
        WriteStorage<'a, actor::Actor>,
    );

    fn run(&mut self, (dt, vels, mut actors): Self::SystemData) {
        log::trace!("running AutoMovementSys system");

        for (vel, actor) in (&vels, &mut actors).join() {
            actor.transform.position.x += vel.0;
            actor.transform.position.y +=
                5.0 * f32::sin(SystemTime::now().elapsed().unwrap().as_millis() as f32);
        }
    }
}

struct CameraSys {}

impl<'a> System<'a> for CameraSys {
    type SystemData = (Write<'a, camera::CameraBundle>, Read<'a, event::WinEvents>);

    fn run(&mut self, (mut camera, events): Self::SystemData) {
        log::trace!("running cameraSys system");

        for ev in &events.events {
            camera.controller.process_events(ev);
            match ev {
                WinEvent::Resize(w, h) => camera.projection.resize((*w, *h)),
                _ => {}
            }
        }
    }
}
