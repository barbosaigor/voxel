use std::cell::RefCell;
use std::rc::Rc;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::model;

use super::render;
use actor;

pub struct WindowRenderer {
    ev_loop: Option<winit::event_loop::EventLoop<()>>,
    window: Option<winit::window::Window>,
    pub rendr: Option<Rc<RefCell<render::Render>>>,
}

impl WindowRenderer {
    pub fn new() -> Self {
        // TODO: replace it
        env_logger::init();
        let (ev_loop, window) = Self::create_win();
        let rendr = Rc::new(RefCell::new(render::Render::new(&window)));
        WindowRenderer {
            ev_loop: Some(ev_loop),
            window: Some(window),
            rendr: Some(rendr),
        }
    }

    pub fn create_win() -> (winit::event_loop::EventLoop<()>, winit::window::Window) {
        let ev_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&ev_loop).unwrap();

        (ev_loop, window)
    }

    pub fn run(&mut self, mut buffActors: Vec<model::BuffActor>) {
        let ev_loop = self.ev_loop.take().unwrap(); 
        let window = self.window.take().unwrap();
        
        let rendr = self.rendr.as_ref().unwrap().clone();

        ev_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => window.request_redraw(),
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !rendr.borrow_mut().input(event) {
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
                                rendr.borrow_mut().resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                rendr.borrow_mut().resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    rendr.borrow_mut().update();
                    match rendr.borrow_mut().draw(&mut buffActors) {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            rendr.borrow_mut().resize(rendr.borrow().size)
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // We're ignoring timeouts
                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                _ => {}
            };
        });
    }
}
