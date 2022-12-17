use crate::ecs;
use crate::renderer;
use crate::scene;
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

    pub fn setup_global_system<'a, 'b>(
        &self,
        dispatcher: specs::DispatcherBuilder<'a, 'b>,
    ) -> specs::DispatcherBuilder<'a, 'b> {
        dispatcher
            .with(
                ecs::systems::delta_time::DeltaTimeSys {},
                "delta_time_sys",
                &[],
            )
            .with_barrier()
    }
}
