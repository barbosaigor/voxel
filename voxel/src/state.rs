use crate::actor;
use crate::event;
use crate::render;
use crate::scene;
use crate::ticker;
use specs::rayon::ThreadPool;
use specs::rayon::ThreadPoolBuilder;
use specs::WorldExt;
use std::sync::Arc;

pub struct State {
    pub world: specs::World,
    pub thread_pool: Arc<ThreadPool>,
    pub render: render::render::Render,
    pub scene: Box<dyn scene::Scene>,
}

impl State {
    // TODO: let it configurable
    const MAX_THREADS: usize = 8;

    pub fn new(scene: Box<dyn scene::Scene>, window: &winit::window::Window) -> Self {
        let thread_pool = Arc::new(
            ThreadPoolBuilder::new()
                .num_threads(Self::MAX_THREADS)
                .thread_name(|i| format!("rayon-voxel-{}", i))
                .build()
                .unwrap(),
        );

        Self {
            world: Self::setup(&scene),
            thread_pool,
            render: render::render::Render::new(window),
            scene,
        }
    }

    pub fn setup(scene: &Box<dyn scene::Scene>) -> specs::World {
        let mut world = specs::World::new();

        scene.setup(&mut world);

        world
    }

    fn draw(&mut self, events: &Vec<event::WinEvent>, actors: &Vec<actor::Actor>) {
        use event::WinEvent::*;

        for ev in events.iter() {
            log::trace!("render system processing {:?}", ev);

            match ev {
                Redraw => {
                    self.render.update();
                    let res = self.render.draw(actors);
                    match res {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            log::warn!("wgpu surface lost or outdated");
                            let size = self.render.size.clone();
                            self.render.resize(size);
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
                Resize(w, h) => self.render.resize((*w, *h)),
                _ => {
                    log::trace!("calling rendeder_engine input");
                    self.render.input(ev);
                }
            }
        }
    }
}

impl ticker::Ticker for State {
    fn tick(&mut self, win_events: Vec<event::WinEvent>) {
        log::trace!("running tick for state");

        let actors: Vec<actor::Actor> = self
            .world
            .read_component::<actor::Actor>()
            .as_slice()
            .iter()
            .cloned()
            .collect();

        self.draw(&win_events, &actors);

        self.world.write_resource::<event::WinEvents>().events = win_events;

        self.scene.
            setup_systems(&mut self.world, self.thread_pool.clone())
            .dispatch(&self.world);

        self.world.maintain();

        self.world.write_resource::<event::WinEvents>().events.clear();
    }
}
