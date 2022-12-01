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
    MouseMotion(f64, f64),
    Scroll(MouseScroll),
    Esc,
    Close,
    Redraw,
    Resize(u32, u32),
    Nothing,
}

#[derive(Clone, Debug)]
pub enum MouseScroll {
    Line(f64),
    Pixel(f64),
}

#[derive(Default)]
pub struct WinEvents {
    pub events: Vec<WinEvent>,
}
