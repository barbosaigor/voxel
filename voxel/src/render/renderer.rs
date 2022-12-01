use crate::{actor, state};
use std::sync::{Arc, Mutex};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::event::*;
use crate::ticker::Ticker;

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

    pub fn run(&mut self, _: Vec<actor::Actor>) {
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
                                        log::debug!("pushing {:?} to event bus", WinEvent::Space);
                                        events.lock().unwrap().push(WinEvent::Space);
                                    }
                                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                                        log::debug!("pushing {:?} to event bus", WinEvent::Up);
                                        events.lock().unwrap().push(WinEvent::Up);
                                    }
                                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                                        log::debug!("pushing {:?} to event bus", WinEvent::Left);
                                        events.lock().unwrap().push(WinEvent::Left);
                                    }
                                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                                        log::debug!("pushing {:?} to event bus", WinEvent::Down);
                                        events.lock().unwrap().push(WinEvent::Down);
                                    }
                                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                                        log::debug!("pushing {:?} to event bus", WinEvent::Right);
                                        events.lock().unwrap().push(WinEvent::Right);
                                    }
                                    _ => {
                                        log::debug!("event not mapped: {:?}", keycode);
                                    }
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
                        } => {
                            log::debug!("changing control flow to exit");
                            *control_flow = ControlFlow::Exit
                        }
                        WindowEvent::Resized(physical_size) => {
                            log::debug!(
                                "pushing resize ({:?}) to event bus",
                                WinEvent::Resize(physical_size.width, physical_size.height)
                            );
                            events
                                .lock()
                                .unwrap()
                                .push(WinEvent::Resize(physical_size.width, physical_size.height));
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            log::debug!(
                                "pushing resize ({:?}) to event bus",
                                WinEvent::Resize(new_inner_size.width, new_inner_size.height)
                            );
                            events.lock().unwrap().push(WinEvent::Resize(
                                new_inner_size.width,
                                new_inner_size.height,
                            ));
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    log::debug!("pushing redraw ({:?}) to event bus", WinEvent::Redraw);
                    events.lock().unwrap().push(WinEvent::Redraw);
                }
                _ => {}
            };
        });
    }
}

pub fn run(
    ev_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
    mut global_state: state::State,
)
{
    ev_loop.run(move |event, _, control_flow| {
        log::trace!("running event loop");
        *control_flow = ControlFlow::Poll;
        let mut win_events = Vec::<WinEvent>::new();

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
                                    log::debug!("pushing {:?} to event bus", WinEvent::Space);
                                    win_events.push(WinEvent::Space);
                                }
                                VirtualKeyCode::W | VirtualKeyCode::Up => {
                                    log::debug!("pushing {:?} to event bus", WinEvent::Up);
                                    win_events.push(WinEvent::Up);
                                }
                                VirtualKeyCode::A | VirtualKeyCode::Left => {
                                    log::debug!("pushing {:?} to event bus", WinEvent::Left);
                                    win_events.push(WinEvent::Left);
                                }
                                VirtualKeyCode::S | VirtualKeyCode::Down => {
                                    log::debug!("pushing {:?} to event bus", WinEvent::Down);
                                    win_events.push(WinEvent::Down);
                                }
                                VirtualKeyCode::D | VirtualKeyCode::Right => {
                                    log::debug!("pushing {:?} to event bus", WinEvent::Right);
                                    win_events.push(WinEvent::Right);
                                }
                                _ => {
                                    log::debug!("event not mapped: {:?}", keycode);
                                }
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
                    } => {
                        log::debug!("changing control flow to exit");
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::Resized(physical_size) => {
                        log::debug!(
                            "pushing resize ({:?}) to event bus",
                            WinEvent::Resize(physical_size.width, physical_size.height)
                        );
                        win_events.push(WinEvent::Resize(physical_size.width, physical_size.height));
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        log::debug!(
                            "pushing resize ({:?}) to event bus",
                            WinEvent::Resize(new_inner_size.width, new_inner_size.height)
                        );
                        win_events.push(WinEvent::Resize(
                            new_inner_size.width,
                            new_inner_size.height,
                        ));
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                log::debug!("pushing redraw ({:?}) to event bus", WinEvent::Redraw);
                win_events.push(WinEvent::Redraw);
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                ..
            } => {
                win_events.push(WinEvent::MouseMotion(delta.0, delta.1))
            } 
            _ => {}
        };
        
        global_state.tick(win_events);
    });
}