use crate::ecs;
use crate::renderer;
use crate::scene;
use specs::WorldExt;

pub struct State {
    pub world: specs::World,
    pub render: renderer::render::Render,
    pub scene: Option<Box<dyn scene::Scene>>,
}

impl State {
    pub fn new(scene: Box<dyn scene::Scene>, window: &winit::window::Window) -> Self {
        let mut this = Self {
            world: specs::World::new(),
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
