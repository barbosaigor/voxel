use actor::{spawner, transform};
use cgmath::Rotation3;
use render::renderer::{self, WinEvents};
use specs::prelude::*;

fn main() {
    let mut world = specs::World::new();

    let win_renderer = renderer::WindowRenderer::new();
    let render_sys = RenderSys {
        render_engine: render::render::Render::new(win_renderer.window.as_ref().unwrap()),
    };

    world.insert(win_renderer);

    world.insert(WinEvents::default());

    let mut dispatcher = DispatcherBuilder::new()
        .with(AutoMovement {}, "auto_movement", &[])
        .with_thread_local(WinSys{})
        .with_thread_local(render_sys)
        .build();

    dispatcher.setup(&mut world);

    let actor = spawn_actor(
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

    // spawn(
    //     &mut actors,
    //     transform::Transform {
    //         position: cgmath::Vector3 {
    //             x: -2.5,
    //             y: -2.5,
    //             z: -2.5,
    //         },
    //         rotation: cgmath::Quaternion::from_axis_angle(
    //             cgmath::Vector3::unit_z(),
    //             cgmath::Deg(0.0),
    //         ),
    //     },
    //     "/res/cube.obj",
    //     Some([0.3, 0.7, 0.3, 1.0]),
    // );

    // spawn(
    //     &mut actors,
    //     transform::Transform {
    //         position: cgmath::Vector3 {
    //             x: 0.0,
    //             y: 0.0,
    //             z: 0.0,
    //         },
    //         rotation: cgmath::Quaternion::from_axis_angle(
    //             cgmath::Vector3::unit_z(),
    //             cgmath::Deg(0.0),
    //         ),
    //     },
    //     "/res/cube.obj",
    //     Some([0.3, 0.3, 0.7, 1.0]),
    // );

    world.create_entity().with(Vel(0.5)).with(actor).build();

    loop {
        // This dispatches all the systems in parallel (but blocking).
        dispatcher.dispatch(&world);
        world.maintain();
    }
}

pub fn spawn(
    actors: &mut Vec<actor::Actor>,
    transform: transform::Transform,
    obj_path: &str,
    color: Option<[f32; 4]>,
) {
    actors.push(spawn_actor(transform, obj_path, color));
}

pub fn spawn_actor(
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

struct AutoMovement {}

impl<'a> System<'a> for AutoMovement {
    type SystemData = (ReadStorage<'a, Vel>, WriteStorage<'a, actor::Actor>);

    fn run(&mut self, data: Self::SystemData) {
        let (vels, mut actors) = data;

        for (vel, actor) in (&vels, &mut actors).join() {
            // println!("vel: {:?}", vel);
            // println!("actor: {:?}", actor);

            actor.transform.position.x += vel.0;
        }
    }
}

struct RenderSys {
    render_engine: render::render::Render,
}

impl<'a> System<'a> for RenderSys {
    type SystemData = (
        Write<'a, renderer::WindowRenderer>,
        Read<'a, renderer::WinEvents>,
        ReadStorage<'a, actor::Actor>,
    );

    fn run(&mut self, (win, events, actors): Self::SystemData) {
        use renderer::WinEvent::*;

        for ev in &events.events {
            log::info!("render system processing {:?}", ev);

            match ev {
                Redraw => {
                    self.render_engine.update();
                    let r = self.render_engine.draw(&actors.join().cloned().collect());
                    match r {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            self.render_engine.resize(self.render_engine.size)
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *win.should_exit.lock().unwrap() = true
                        }
                        // We're ignoring timeouts
                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                _ => {
                    self.render_engine.input(ev);
                }
            }
        }
    }
}

struct WinSys {}

impl<'a> System<'a> for WinSys {
    type SystemData = (
        Write<'a, renderer::WindowRenderer>,
        Write<'a, renderer::WinEvents>,
    );

    fn run(&mut self, (win_renderer, mut win_events): Self::SystemData) {
        for ev in win_renderer.events.lock().unwrap().iter() {
            win_events.events.push(ev.clone());

            println!("publishing event: {:?}", ev);
        }
    }
}
