use futures::executor;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, self},
};
use super::render;

pub struct WindowRenderer {}

impl WindowRenderer {
    pub fn new() -> Self {
        WindowRenderer{
        }
    }

    pub fn run(&self) {
        let _ = executor::block_on(self.run_async());
    }

    pub async fn run_async(&self) {
        // TODO: replace it
        env_logger::init();

        let ev_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&ev_loop).unwrap();
    
        self.event_loop(ev_loop, window).await;
    }

    async fn event_loop(&self, ev_loop: EventLoop<()>, window: window::Window) {
        let mut rendr = render::Render::new(&window).await;

        ev_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            
            match event {
                Event::MainEventsCleared => window.request_redraw(),
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !rendr.input(event) {
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
                                rendr.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                rendr.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    rendr.update();
                    match rendr.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => rendr.resize(rendr.size),
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
