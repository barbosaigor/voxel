use super::render::renderer;
use super::scene;
use super::state;

pub struct App {
    pub global_state: state::State,
    window: winit::window::Window,
    ev_loop: winit::event_loop::EventLoop<()>,
}

impl App {
    pub fn new(scene: Box<dyn scene::Scene>) -> Self {
        env_logger::init();
        let (ev_loop, window) = renderer::WindowRenderer::create_win();

        Self {
            global_state: state::State::new(scene, &window),
            ev_loop,
            window,
        }
    }

    pub fn run(self) {        
        renderer::run(self.ev_loop, self.window, self.global_state);
    }
}
