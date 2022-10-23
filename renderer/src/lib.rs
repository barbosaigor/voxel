pub mod renderer;
mod render;
mod model;
mod texture;
mod camera;
mod instance;
mod resources;
pub mod spawner;

pub fn run_window() {
    let rndr = renderer::WindowRenderer::new();
    rndr.run();
}