use crate::actor;
use crate::camera;
use crate::delta_time;
use crate::ecs;
use crate::event;
use crate::renderer;
use crate::scene;
use crate::ticker;
use specs::rayon::ThreadPool;
use specs::rayon::ThreadPoolBuilder;
use specs::WorldExt;
use std::sync::Arc;

pub struct State {
    pub world: specs::World,
    pub ecs_thread_pool: Arc<ThreadPool>,
    pub render: renderer::render::Render,
    pub scene: Option<Box<dyn scene::Scene>>,
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

        let mut this = Self {
            world: specs::World::new(),
            ecs_thread_pool: thread_pool,
            render: renderer::render::Render::new(window),
            scene: None,
        };

        this.setup(scene);

        this
    }

    fn setup(&mut self, mut scene: Box<dyn scene::Scene>) {
        scene.setup(self);
        self.scene = Some(scene);
    }

    fn draw(&mut self, events: &Vec<event::WinEvent>, actors: &Vec<actor::Actor>) {
        use event::WinEvent::*;

        let mut camera = self.world.write_resource::<camera::CameraBundle>();
        let dt = self.world.read_resource::<delta_time::DeltaTime>();

        for ev in events.iter() {
            log::trace!("render system processing {:?}", ev);

            match ev {
                Redraw => {
                    camera.update(dt.dt);

                    let res = self
                        .render
                        .draw(actors, &camera.build_bind_group(&self.render.device));
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
                    // log::trace!("calling rendeder_engine input");
                }
            }
        }
    }

    fn setup_global_system<'a, 'b>(&self, dispatcher: specs::DispatcherBuilder<'a, 'b>) -> specs::DispatcherBuilder<'a, 'b> {
        dispatcher
            .with(
                ecs::systems::delta_time::DeltaTimeSys {},
                "delta_time_sys",
                &[],
            )
            .with_barrier()
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

        let mut dispatcher_builder = specs::DispatcherBuilder::new().with_pool(self.ecs_thread_pool.clone());
        dispatcher_builder = self.setup_global_system(dispatcher_builder);
        dispatcher_builder = self.scene.as_mut().unwrap().setup_systems(dispatcher_builder);
        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut self.world);
        dispatcher.dispatch(&self.world);

        self.world.maintain();

        self.world
            .write_resource::<event::WinEvents>()
            .events
            .clear();
    }
}
