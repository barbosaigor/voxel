use crate::game_ticker;

use super::renderer::window;
use super::scene;
use super::state;

pub struct App {
    pub global_state: state::State,
    pub game_ticker: game_ticker::GameTicker,
    window: winit::window::Window,
    ev_loop: winit::event_loop::EventLoop<()>,
}

impl App {
    pub fn new(scene: Box<dyn scene::Scene>) -> Self {
        env_logger::init();
        let (ev_loop, window) = window::create_win();
        let game_ticker = game_ticker::GameTicker{};
        Self {
            global_state: state::State::new(scene, &window),
            game_ticker,
            ev_loop,
            window,
        }
    }

    pub fn run(self) {        
        window::run(self.ev_loop, self.window, self.game_ticker, self.global_state);
    }
}
