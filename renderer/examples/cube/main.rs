use renderer::renderer;

fn main() {
    let mut rndr = renderer::WindowRenderer::new();

    rndr.rendr
        .as_ref()
        .unwrap()
        .borrow_mut()
        .push_model("/res/cube.obj");

    rndr.run();
}
