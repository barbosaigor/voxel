use specs::WorldExt;

use crate::{actor, camera, delta_time, event, state::State, ticker};

pub struct GameTicker {}

impl GameTicker {
    fn draw(
        &mut self,
        global_state: &mut State,
        events: &Vec<event::WinEvent>,
        actors: &Vec<actor::Actor>,
    ) {
        use event::WinEvent::*;

        let mut camera = global_state.world.write_resource::<camera::CameraBundle>();
        let dt = global_state.world.read_resource::<delta_time::DeltaTime>();

        for ev in events.iter() {
            log::trace!("render system processing {:?}", ev);

            match ev {
                Redraw => {
                    camera.update(dt.dt);

                    let res = global_state.render.draw(
                        actors,
                        &camera.build_bind_group(&global_state.render.device),
                    );
                    match res {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            log::warn!("wgpu surface lost or outdated");
                            let size = global_state.render.size.clone();
                            global_state.render.resize(size);
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
                Resize(w, h) => global_state.render.resize((*w, *h)),
                _ => {
                    // log::trace!("calling rendeder_engine input");
                }
            }
        }
    }
}

impl ticker::Ticker for GameTicker {
    fn tick(&mut self, global_state: &mut State, win_events: Vec<event::WinEvent>) {
        log::trace!("running tick for game ticker");

        let actors: Vec<actor::Actor> = global_state
            .world
            .read_component::<actor::Actor>()
            .as_slice()
            .iter()
            .cloned()
            .collect();

        self.draw(global_state, &win_events, &actors);

        global_state
            .world
            .write_resource::<event::WinEvents>()
            .events = win_events;

        let mut dispatcher_builder =
            specs::DispatcherBuilder::new().with_pool(global_state.ecs_thread_pool.clone());
        dispatcher_builder = global_state.setup_global_system(dispatcher_builder);
        dispatcher_builder = global_state
            .scene
            .as_mut()
            .unwrap()
            .setup_systems(dispatcher_builder);
        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut global_state.world);
        dispatcher.dispatch(&global_state.world);

        global_state.world.maintain();

        global_state
            .world
            .write_resource::<event::WinEvents>()
            .events
            .clear();
    }
}
