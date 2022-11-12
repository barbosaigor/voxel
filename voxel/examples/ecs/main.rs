use actor::{spawner, transform};
use cgmath::Rotation3;
use render::renderer::{self, WinEvents};
use specs::prelude::*;
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

fn main() {
    let mut world = specs::World::new();

    let ev_bus_res = EventBus {
        events: Arc::new(Mutex::new(WinEvents::default())),
    };
    let bus = ev_bus_res.events.clone();
    world.insert(ev_bus_res);

    // window renderer replacement
    env_logger::init();
    let (ev_loop, window) = renderer::WindowRenderer::create_win();

    world.insert(render::render::Render::new(&window));

    let mut dispatcher = DispatcherBuilder::new()
        .with(AutoMovement {}, "auto_movement", &[])
        .with_thread_local(RenderSys {
            // render_engine: render::render::Render::new(&window),
        })
        .with_thread_local(EventBusCleanerSys {})
        .build();

    dispatcher.setup(&mut world);

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

    let dispatcher_fn = move || {
        dispatcher.dispatch(&world);
        world.maintain();
    };

    renderer::run(ev_loop, window, bus, dispatcher_fn);
}

#[derive(Default)]
struct EventBus {
    events: Arc<Mutex<renderer::WinEvents>>,
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

struct AutoMovement {}

impl<'a> System<'a> for AutoMovement {
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

struct RenderSys {
    // render_engine: render::render::Render,
}

impl<'a> System<'a> for RenderSys {
    type SystemData = (
        Write<'a, render::render::Render>,
        Read<'a, EventBus>,
        ReadStorage<'a, actor::Actor>,
    );

    fn run(&mut self, (mut render_engine, events_holder, actors): Self::SystemData) {
        use renderer::WinEvent::*;
        log::trace!("render system running");

        for ev in events_holder.events.lock().unwrap().events.iter() {
            log::trace!("render system processing {:?}", ev);

            match ev {
                Redraw => {
                    render_engine.update();
                    let res = render_engine.draw(&actors.join().cloned().collect());
                    match res {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            log::warn!("wgpu surface lost or outdated");
                            let size = render_engine.size.clone();
                            render_engine.resize(size);
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            log::warn!("wgpu surface out of memory");
                            // TODO
                            // *win.should_exit.lock().unwrap() = true
                            panic!("wgpu surface out of memory");
                        }
                        // We're ignoring timeouts
                        Err(wgpu::SurfaceError::Timeout) => log::warn!("surface timeout"),
                    }
                }
                Resize(w, h) => render_engine.resize((*w, *h)),
                _ => {
                    log::trace!("calling rendeder_engine input");
                    render_engine.input(ev);
                }
            }
        }
    }
}

struct EventBusCleanerSys {}

impl<'a> System<'a> for EventBusCleanerSys {
    type SystemData = Write<'a, EventBus>;

    fn run(&mut self, events_holder: Self::SystemData) {
        log::trace!("EventBusCleanerSys running");
        let events = &mut events_holder.events.lock().unwrap().events;
        if events.len() != 0 {
            log::trace!("EventBusCleanerSys: cleanning events: {:?}", events);
            events.clear();
        }
    }
}
