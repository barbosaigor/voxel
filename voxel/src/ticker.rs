use super::event;

pub trait Ticker {
    fn tick(&mut self, win_events: Vec<event::WinEvent>);
}
