use actor;
use std::{
    default,
    sync::{Arc, Mutex},
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Default)]
pub struct WindowRenderer {
    ev_loop: Option<winit::event_loop::EventLoop<()>>,
    pub window: Option<winit::window::Window>,
    pub events: Arc<Mutex<Vec<WinEvent>>>,
    pub should_exit: Arc<Mutex<bool>>,
}

unsafe impl Sync for WindowRenderer {}

unsafe impl Send for WindowRenderer {}

impl WindowRenderer {
    pub fn new() -> Self {
        // TODO: replace it
        env_logger::init();
        let (ev_loop, window) = Self::create_win();
        WindowRenderer {
            ev_loop: Some(ev_loop),
            events: Arc::new(Mutex::new(Vec::new())),
            window: Some(window),
            should_exit: Arc::new(Mutex::new(false)),
        }
    }

    pub fn create_win() -> (winit::event_loop::EventLoop<()>, winit::window::Window) {
        let ev_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&ev_loop).unwrap();

        (ev_loop, window)
    }

    pub fn run(&mut self, actors: Vec<actor::Actor>) {
        let ev_loop = self.ev_loop.take().unwrap();
        let window = self.window.take().unwrap();

        let events = self.events.clone();

        ev_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => window.request_redraw(),
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    match event {
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state,
                                    virtual_keycode: Some(keycode),
                                    ..
                                },
                            ..
                        } => {
                            let is_pressed = *state == ElementState::Pressed;
                            if is_pressed {
                                match keycode {
                                    VirtualKeyCode::Space => {
                                        events.lock().unwrap().push(WinEvent::Space);
                                    }
                                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                                        events.lock().unwrap().push(WinEvent::W);
                                    }
                                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                                        events.lock().unwrap().push(WinEvent::A);
                                    }
                                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                                        events.lock().unwrap().push(WinEvent::S);
                                    }
                                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                                        events.lock().unwrap().push(WinEvent::D);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    };

                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            events
                                .lock()
                                .unwrap()
                                .push(WinEvent::Resize(physical_size.width, physical_size.height));
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            events.lock().unwrap().push(WinEvent::Resize(
                                new_inner_size.width,
                                new_inner_size.height,
                            ));
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    events.lock().unwrap().push(WinEvent::Redraw);
                }
                _ => {}
            };
        });
    }
}

#[derive(Clone, Debug)]
pub enum WinEvent {
    Space,
    W,
    A,
    S,
    D,
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
