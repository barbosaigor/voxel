use cgmath::Rotation3;
use rand::{self, Rng};
use rapier3d::prelude::*;
use specs::prelude::*;
use std::fmt::Debug;
use voxel::{
    self,
    actor::{self, transform},
    camera::{self, CameraController},
    delta_time::{self, DeltaTime},
    event::{self, WinEvent},
    fly_camera, physics, scene, state,
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
            .with(PhysicsSys {}, "physics_scenario_sys", &[])
    }

    fn setup(&mut self, global_state: &mut state::State) {
        global_state.world.register::<Vel>();
        global_state.world.register::<actor::Actor>();
        global_state.world.register::<physics::RigidBodyComponent>();

        global_state.world.insert(event::WinEvents::default());

        let camera = camera::Camera::new((0.0, 15.0, 20.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection = camera::Projection::new(
            global_state.render.size.0,
            global_state.render.size.1,
            cgmath::Deg(45.0),
            0.1,
            100.0,
        );
        let controller = fly_camera::FlyCameraController::new(500.0, 7.0);
        global_state.world.insert(camera::CameraBundle::from_camera(
            camera, projection, controller,
        ));

        global_state.world.insert(DeltaTime::default());
        global_state.world.insert(delta_time::now());

        // Spawn entities
        {
            let bounding_actor = actor::Actor::new(
                transform::Transform {
                    position: cgmath::Vector3 {
                        x: 2.5,
                        y: 100.5,
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

            let mut physics_eng = Physics::new();
            init_scenario(&mut physics_eng);

            /* Create the bounding cube. */
            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![
                    bounding_actor.transform.position.x,
                    bounding_actor.transform.position.y,
                    bounding_actor.transform.position.z
                ])
                .build();
            let collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0)
                .restitution(0.99)
                .build();
            let body_handle = physics_eng.rigid_body_set.insert(rigid_body);
            physics_eng.collider_set.insert_with_parent(
                collider,
                body_handle,
                &mut physics_eng.rigid_body_set,
            );

            global_state
                .world
                .create_entity()
                .with(Vel(0.0005))
                .with(bounding_actor)
                .with(physics::RigidBodyComponent::new(body_handle))
                .build();

            /* Create the second bounding cube. */
            let second_cube = actor::Actor::new(
                transform::Transform {
                    position: cgmath::Vector3 {
                        x: -2.5,
                        y: 10.5,
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
            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![
                    second_cube.transform.position.x,
                    second_cube.transform.position.y,
                    second_cube.transform.position.z
                ])
                .build();
            let collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0)
                .restitution(0.50)
                .build();
            let body_handle = physics_eng.rigid_body_set.insert(rigid_body);
            physics_eng.collider_set.insert_with_parent(
                collider,
                body_handle,
                &mut physics_eng.rigid_body_set,
            );
            global_state
                .world
                .create_entity()
                .with(Vel(-0.0005))
                .with(second_cube)
                .with(physics::RigidBodyComponent::new(body_handle))
                .build();

            /* Create se third bounding cube */
            let third_cube = actor::Actor::new(
                transform::Transform {
                    position: cgmath::Vector3 {
                        x: 0.0,
                        y: 50.0,
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
            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![
                    third_cube.transform.position.x,
                    third_cube.transform.position.y,
                    third_cube.transform.position.z
                ])
                .build();
            let collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0)
                .restitution(0.50)
                .build();
            let body_handle = physics_eng.rigid_body_set.insert(rigid_body);
            physics_eng.collider_set.insert_with_parent(
                collider,
                body_handle,
                &mut physics_eng.rigid_body_set,
            );

            global_state
                .world
                .create_entity()
                .with(Vel(0.05))
                .with(third_cube)
                .with(physics::RigidBodyComponent::new(body_handle))
                .build();

            /* Create the bounding cube. */
            for _ in 0..100 {
                let mut r = rand::thread_rng();

                let (x, y, z): (f32, f32, f32) = (
                    r.gen_range(-20.0..20.0),
                    r.gen_range(5.0..100.0),
                    r.gen_range(-20.0..20.0),
                );
                let (red, green, blue): (f32, f32, f32) = r.gen();
                let cube_actor = actor::Actor::new(
                    transform::Transform {
                        position: cgmath::Vector3 { x, y, z },
                        rotation: cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        ),
                    },
                    "/res/cube.obj",
                    Some([red, green, blue, 1.0]),
                );
                let rigid_body = RigidBodyBuilder::dynamic()
                    .translation(vector![
                        cube_actor.transform.position.x,
                        cube_actor.transform.position.y,
                        cube_actor.transform.position.z
                    ])
                    .build();
                let collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0)
                    .restitution(r.gen_range(0.4..1.0))
                    .build();
                let body_handle = physics_eng.rigid_body_set.insert(rigid_body);
                physics_eng.collider_set.insert_with_parent(
                    collider,
                    body_handle,
                    &mut physics_eng.rigid_body_set,
                );

                global_state
                    .world
                    .create_entity()
                    .with(Vel(0.0005))
                    .with(cube_actor)
                    .with(physics::RigidBodyComponent::new(body_handle))
                    .build();
            }

            global_state.world.insert(physics_eng);
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
            actor.transform.position.x += vel.0 * dt.dt.as_secs_f32();
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

struct Physics {
    pipeline: rapier3d::prelude::PhysicsPipeline,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    gravity: nalgebra::Vector3<f32>,
    physics_hooks: (),
    event_handler: (),
}

impl Physics {
    fn new() -> Self {
        Self {
            pipeline: Default::default(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            gravity: vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
        }
    }

    fn step(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self::new()
    }
}

struct PhysicsSys {}

impl<'a> System<'a> for PhysicsSys {
    type SystemData = (
        Write<'a, Physics>,
        WriteStorage<'a, actor::Actor>,
        ReadStorage<'a, physics::RigidBodyComponent>,
    );

    fn run(&mut self, (mut physics, mut actors, rbs): Self::SystemData) {
        log::trace!("running Physics system");
        physics.step();
        // update actors' transform
        for (actor, rb) in (&mut actors, &rbs).join() {
            let actor_body = &physics.rigid_body_set.get(rb.rigid_body);
            if let None = *actor_body {
                continue;
            }
            let actor_body = actor_body.unwrap();
            actor.transform.position.x = actor_body.translation().x;
            actor.transform.position.y = actor_body.translation().y;
            actor.transform.position.z = actor_body.translation().z;

            actor.transform.rotation.s = actor_body.rotation().scalar();
            actor.transform.rotation.v = cgmath::Vector3::from((
                actor_body.rotation().vector()[0],
                actor_body.rotation().vector()[1],
                actor_body.rotation().vector()[2],
            ));
        }
    }
}

fn init_scenario(physics: &mut Physics) {
    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
    physics.collider_set.insert(collider);
}
