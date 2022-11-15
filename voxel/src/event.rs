#[derive(Clone, Debug)]
pub enum WinEvent {
    Space,
    W,
    A,
    S,
    D,
    Up,
    Down,
    Left,
    Right,
    Esc,
    Close,
    Redraw,
    Resize(u32, u32),
    Nothing,
}

#[derive(Default)]
pub struct WinEvents {
    pub events: Vec<WinEvent>,
}