use renderer::renderer;

fn main() {
    let mut rndr = renderer::WindowRenderer::new();

    rndr.rendr
        .as_ref()
        .unwrap()
        .borrow_mut()
        .push_uniform_color_model("/res/cube.obj", [0.7, 0.3, 0.3, 1.0]);

    rndr.run();
}
