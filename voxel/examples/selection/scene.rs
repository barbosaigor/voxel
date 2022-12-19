use cgmath::Rotation3;
use core::time;
use rand::{self, Rng};
use specs::prelude::*;
use voxel::{
    self,
    actor::{self, transform},
    camera::{self, CameraController},
    delta_time::{self, DeltaTime},
    event::{self, WinEvent, WinEvents},
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
            .with(SpawnerSys, "spawner_sys", &[])
            .with(CameraSys {}, "camera_sys", &[])
            .with(MousePosUpdaterSys {}, "mouse_pos_sys", &[])
            .with(RayCastSys {}, "raycast_sys", &[])
            .with(WinSizeUpdaterSys {}, "win_size_updater_sys", &[])
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
        let controller = fly_camera::FlyCameraController::new(1500.0, 9.0);
        global_state.world.insert(camera::CameraBundle::from_camera(
            camera, projection, controller,
        ));

        global_state.world.insert(DeltaTime::default());
        global_state.world.insert(delta_time::now());
        global_state.world.insert(LastMousePos(0.0, 0.0));
        global_state.world.insert(WinSize(0, 0));

        // Spawn entities
        {
            global_state
                .world
                .create_entity()
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
                    "/res/plane.obj",
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

struct SpawnerSys;

impl<'a> System<'a> for SpawnerSys {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Write<'a, time::Duration>,
    );

    fn run(&mut self, (entites, updater, mut last_time): Self::SystemData) {
        log::trace!("running SpawnerSys system");

        let now = delta_time::now();
        let dt = now - *last_time;
        if dt.as_secs_f32() > 1.0 {
            *last_time = now;
            let mut r = rand::thread_rng();

            let (x, y, z): (f32, f32, f32) = (
                r.gen_range(-5.0..5.0),
                r.gen_range(-5.0..5.0),
                r.gen_range(-5.0..5.0),
            );
            let (red, green, blue): (f32, f32, f32) = r.gen();

            let degree: f32 = r.gen();
            let vel = r.gen_range(-2.0..2.0);

            let entity = entites.create();
            updater.insert(entity, Vel(vel));
            updater.insert(
                entity,
                actor::Actor::new(
                    transform::Transform {
                        position: cgmath::Vector3 {
                            x: x * 10.0,
                            y: y * 10.0,
                            z: z * 10.0,
                        },
                        rotation: cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(degree * 360.0),
                        ),
                    },
                    "/res/halfcube.obj",
                    Some([red, green, blue, 1.0]),
                ),
            );
        }
    }
}

#[derive(Debug, Default)]
struct LastMousePos(f64, f64);

struct MousePosUpdaterSys;

impl<'a> System<'a> for MousePosUpdaterSys {
    type SystemData = (Read<'a, WinEvents>, Write<'a, LastMousePos>);

    fn run(&mut self, (win_events, mut last_mouse_pos): Self::SystemData) {
        log::trace!("running MousePosUpdaterSys system");

        for ev in &win_events.events {
            match ev {
                WinEvent::MouseMoved(x, y) => {
                    last_mouse_pos.0 = *x;
                    last_mouse_pos.1 = *y;
                    log::trace!(
                        "MousePosUpdaterSys: updating last mouse position to ({:?})",
                        *last_mouse_pos
                    );
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Default)]
struct WinSize(u32, u32);

struct WinSizeUpdaterSys;

impl<'a> System<'a> for WinSizeUpdaterSys {
    type SystemData = (Read<'a, WinEvents>, Write<'a, WinSize>);

    fn run(&mut self, (win_events, mut win_size): Self::SystemData) {
        log::trace!("running WinSizeUpdaterSys system");

        for ev in &win_events.events {
            match ev {
                WinEvent::Resize(w, h) => {
                    win_size.0 = *w;
                    win_size.1 = *h;

                    log::trace!("WinSizeUpdaterSys: updating winsize to ({:?})", *win_size);
                }
                _ => {}
            }
        }
    }
}

struct RayCastSys;

impl<'a> System<'a> for RayCastSys {
    type SystemData = (
        Read<'a, WinEvents>,
        Write<'a, LastMousePos>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, WinSize>,
        Write<'a, camera::CameraBundle>,
    );

    fn run(&mut self, (win_events, cursor, entities, updater, win_size, camera): Self::SystemData) {
        log::trace!("running RayCastSys system");

        for ev in &win_events.events {
            match ev {
                WinEvent::MouseButtons(event::MouseButton::Left) => {
                    let intersection = raycast::intersection(
                        (cursor.0, cursor.1),
                        (win_size.0, win_size.1),
                        &camera,
                    );

                    if let None = intersection {
                        return;
                    }

                    let (x, y, z) = intersection.unwrap();

                    let mut r = rand::thread_rng();

                    let (red, green, blue): (f32, f32, f32) = r.gen();

                    let degree: f32 = r.gen();
                    let vel = r.gen_range(-2.0..2.0);

                    let entity = entities.create();
                    updater.insert(entity, Vel(vel * 3.0));
                    updater.insert(
                        entity,
                        actor::Actor::new(
                            transform::Transform {
                                position: cgmath::Vector3 {
                                    x: x as f32,
                                    y: y as f32,
                                    z: z as f32,
                                },
                                rotation: cgmath::Quaternion::from_axis_angle(
                                    cgmath::Vector3::unit_z(),
                                    cgmath::Deg(degree * 360.0),
                                ),
                            },
                            "/res/halfcube.obj",
                            Some([red, green, blue, 1.0]),
                        ),
                    );
                }
                _ => {}
            }
        }
    }
}

mod raycast {
    use voxel::camera;

    pub fn ray(
        cursor: (f64, f64),
        win: (u32, u32),
        camera_projection: [[f32; 4]; 4],
        view_matrix: [[f32; 4]; 4],
    ) -> nalgebra::Vector3<f64> {
        let (x, y) = normalize_cursor(cursor, win);

        let ray_clip = nalgebra::Vector4::new(x as f32, y as f32, -1.0, 1.0);

        let inverse_camera_projection = nalgebra::Matrix4::from(camera_projection)
            .try_inverse()
            .unwrap();

        let ray_eye = inverse_camera_projection * ray_clip;

        let ray_eye_unproj = nalgebra::Vector4::new(ray_eye.x as f32, ray_eye.y as f32, -1.0, 0.0);

        let inverse_view_matrix = nalgebra::Matrix4::from(view_matrix).try_inverse().unwrap();

        let ray_world_matrix = inverse_view_matrix * ray_eye_unproj;

        // let ray_world = ray_world_matrix.normalize();
        let ray_world = ray_world_matrix;
        // let ray_world = ray_world_matrix.xyz();

        nalgebra::Vector3::new(ray_world.x as f64, ray_world.y as f64, ray_world.z as f64)
    }

    // normalizes cursor to -1 to 1
    fn normalize_cursor(cursor: (f64, f64), win: (u32, u32)) -> (f64, f64) {
        (
            (2.0 * cursor.0) / (win.0 as f64) - 1.0,
            1.0 - 2.0 * cursor.1 / (win.1 as f64),
        )
    }

    pub fn intersection(
        cursor: (f64, f64),
        win_size: (u32, u32),
        cam: &camera::CameraBundle,
    ) -> Option<(f64, f64, f64)> {
        // TODO: how do I get the size of the window?
        // maybe I should externalize render as resource
        let ray_norm = ray(
            (cursor.0, cursor.1),
            (win_size.0, win_size.1),
            cam.projection.build_view_projection_matrix().into(),
            cam.camera.matrix().into(),
        );

        let ray_pos = nalgebra::Vector3::new(
            cam.camera.position[0] as f64,
            cam.camera.position[1] as f64,
            cam.camera.position[2] as f64,
        );

        let plane = nalgebra::Vector3::new(0.0, 1.0, 0.0);

        let plane_dot_ray = plane.dot(&ray_norm);

        let t = -(ray_pos.dot(&plane)) / plane_dot_ray;
        if t < 0.0 {
            return None;
        }

        // get intersection position
        let v = (ray_pos + ray_norm * t).xyz();
        Some((v.x, v.y, v.z))
    }
}
