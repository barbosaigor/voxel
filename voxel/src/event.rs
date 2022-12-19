#[derive(Clone, Debug)]
pub enum WinEvent {
    LShift,
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
    MouseMoved(f64, f64),
    MouseButtons(MouseButton),
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

#[derive(Clone, Debug)]
pub enum MouseButton {
    Right,
    Left,
}

#[derive(Default)]
pub struct WinEvents {
    pub events: Vec<WinEvent>,
}
