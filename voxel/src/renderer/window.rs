use crate::event::*;
use crate::state;
use crate::ticker::Ticker;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn create_win() -> (winit::event_loop::EventLoop<()>, winit::window::Window) {
    let ev_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&ev_loop).unwrap();

    (ev_loop, window)
}

pub fn run(
    ev_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
    mut global_state: state::State,
) {
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
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => match keycode {
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
                        VirtualKeyCode::LShift => {
                            log::debug!("pushing {:?} to event bus", WinEvent::LShift);
                            win_events.push(WinEvent::LShift);
                        }
                        _ => {
                            log::debug!("event not mapped: {:?}", keycode);
                        }
                    },
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
                        win_events
                            .push(WinEvent::Resize(physical_size.width, physical_size.height));
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
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => win_events.push(WinEvent::MouseMotion(delta.0, delta.1)),
            _ => {}
        };

        global_state.tick(win_events);
    });
}
