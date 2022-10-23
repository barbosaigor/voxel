use futures::executor;
use renderer::renderer;
fn main() {
    let rndr = renderer::WindowRenderer::new();
    
    // let _ = executor::block_on(rndr.());

    rndr.run();
}

struct Cube {
    // transform: renderer::spawner::Transform,
}