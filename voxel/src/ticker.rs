use super::{event, state};

pub trait Ticker {
    fn tick(&mut self, global_state: &mut state::State, win_events: Vec<event::WinEvent>);
}
