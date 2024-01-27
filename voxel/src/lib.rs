pub mod actor;
pub mod app;
pub mod camera;
pub mod delta_time;
pub mod ecs;
pub mod event;
pub mod fly_camera;
pub mod game_ticker;
pub mod physics;
pub mod renderer;
pub mod scene;
pub mod state;
pub mod ticker;

pub fn run(game: Box<dyn scene::Scene>) {
    app::App::new(game).run();
}
