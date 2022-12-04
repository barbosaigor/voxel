pub mod app;
pub mod event;
pub mod scene;
pub mod state;
pub mod ticker;
pub mod actor;
pub mod renderer;
pub mod camera;
pub mod fly_camera;
pub mod delta_time;
pub mod ecs;

pub fn run(game: Box<dyn scene::Scene>) {
    app::App::new(game).run();
}
