pub mod renderer;
mod render;
mod model;
mod texture;
mod camera;
pub mod instance;
mod resources;
pub mod spawner;

pub fn run_window() {
    let mut rndr = renderer::WindowRenderer::new();
    rndr.run();
}